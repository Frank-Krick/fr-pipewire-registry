use std::collections::BTreeMap;

use tokio::select;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::logging::Logger;
use crate::pipewire_event_consumer::{
    PipewireDeviceUpdate, PipewireNodeUpdate, PipewirePortUpdate,
};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum PortDirection {
    In,
    Out,
    Unknown,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Node {
    pub object_serial: u16,
    pub factory_id: u16,
    pub client_id: u16,
    pub client_api: String,
    pub application_name: String,
    pub node_name: String,
    pub media_class: String,
}

impl Node {
    fn from_pipewire_node_update(node_message: PipewireNodeUpdate) -> Self {
        Node {
            object_serial: node_message.object_serial.parse().unwrap_or(u16::MAX),
            factory_id: node_message.factory_id.parse().unwrap_or(u16::MAX),
            client_id: node_message.client_id.parse().unwrap_or(u16::MAX),
            client_api: node_message.client_api,
            application_name: node_message.application_name,
            node_name: node_message.node_name,
            media_class: node_message.media_class,
        }
    }
}

pub struct GetPortsListRequest {
    pub reply_sender: tokio::sync::oneshot::Sender<Vec<Port>>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Port {
    pub id: u16,
    pub node_id: u16,
    pub name: String,
    pub direction: PortDirection,
    pub physical: bool,
    pub alias: String,
    pub group: String,
    pub path: String,
    pub dsp_format: String,
    pub audio_channel: String,
}

impl Port {
    fn from_pipewire_port_update(port_message: PipewirePortUpdate) -> Self {
        Port {
            id: port_message.id.parse().unwrap(),
            node_id: port_message.node_id.parse().unwrap(),
            name: port_message.name,
            direction: match port_message.direction.as_str() {
                "in" => PortDirection::In,
                "out" => PortDirection::Out,
                _ => PortDirection::Unknown,
            },
            physical: port_message.physical.as_str() == "true",
            alias: port_message.alias,
            group: port_message.group,
            path: port_message.path,
            dsp_format: port_message.dsp_format,
            audio_channel: port_message.audio_channel,
        }
    }
}

pub struct GetNodesListRequest {
    pub reply_sender: tokio::sync::oneshot::Sender<Vec<Node>>,
}

pub struct PipewireRegistry {
    device_update_receiver: UnboundedReceiver<PipewireDeviceUpdate>,
    port_update_receiver: UnboundedReceiver<PipewirePortUpdate>,
    node_update_receiver: UnboundedReceiver<PipewireNodeUpdate>,
    node_list_request_receiver: UnboundedReceiver<GetNodesListRequest>,
    port_list_request_receiver: UnboundedReceiver<GetPortsListRequest>,
    ports: BTreeMap<(PortDirection, u16, u16), Port>,
    nodes: Vec<Node>,
}

impl PipewireRegistry {
    pub fn new(
        device_update_receiver: UnboundedReceiver<PipewireDeviceUpdate>,
        port_update_receiver: UnboundedReceiver<PipewirePortUpdate>,
        node_update_receiver: UnboundedReceiver<PipewireNodeUpdate>,
        node_list_request_receiver: UnboundedReceiver<GetNodesListRequest>,
        port_list_request_receiver: UnboundedReceiver<GetPortsListRequest>,
    ) -> Self {
        PipewireRegistry {
            device_update_receiver,
            port_update_receiver,
            node_update_receiver,
            node_list_request_receiver,
            port_list_request_receiver,
            ports: BTreeMap::new(),
            nodes: Vec::new(),
        }
    }

    pub async fn run(&mut self, logger: &Logger) {
        loop {
            select! {
                port_update = self.port_update_receiver.recv() => {
                    if let Some(port_update) = port_update {
                        logger.log_info("Received port update message");
                        let port = Port::from_pipewire_port_update(port_update);
                        self.ports.insert((port.direction, port.node_id, port.id), port);
                    } else {
                        logger.log_info("Received None Message for port update");
                    }
                }
                _ = self.device_update_receiver.recv() => {
                    logger.log_info("Received device update message");
                }
                node_update = self.node_update_receiver.recv() => {
                    if let Some(node_update) = node_update {
                        logger.log_info("Received node update message");
                        let node = Node::from_pipewire_node_update(node_update);
                        self.nodes.push(node);
                    } else {
                        logger.log_info("Received None Message for node update");
                    }
                }
                node_list_request = self.node_list_request_receiver.recv() => {
                    logger.log_info("Received node list request");
                    node_list_request.unwrap().reply_sender.send(self.nodes.clone()).unwrap();
                }
                port_list_request = self.port_list_request_receiver.recv() => {
                    logger.log_info("Received port list request");
                    port_list_request.unwrap().reply_sender.send(
                        self.ports.values().cloned().collect()).unwrap();
                }
            };
        }
    }
}
