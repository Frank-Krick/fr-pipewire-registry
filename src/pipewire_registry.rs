use std::collections::BTreeMap;

use tokio::select;
use tokio::sync::mpsc::UnboundedReceiver;

use fr_logging::Logger;

use crate::pipewire_event_consumer::{
    PipewireApplicationUpdate, PipewireDeviceUpdate, PipewireNodeUpdate, PipewirePortUpdate,
};

mod registry_application;
mod registry_device;
mod registry_node;
mod registry_port;

use crate::pipewire_registry::registry_application::Application;
use crate::pipewire_registry::registry_device::Device;
use crate::pipewire_registry::registry_node::Node;
use crate::pipewire_registry::registry_port::Port;
pub use crate::pipewire_registry::registry_port::PortDirection;

pub struct GetDevicesListRequest {
    pub reply_sender: tokio::sync::oneshot::Sender<Vec<Device>>,
}

pub struct GetNodesListRequest {
    pub reply_sender: tokio::sync::oneshot::Sender<Vec<Node>>,
}

pub struct GetPortsListRequest {
    pub reply_sender: tokio::sync::oneshot::Sender<Vec<Port>>,
}

pub struct GetApplicationsListRequest {
    pub reply_sender: tokio::sync::oneshot::Sender<Vec<Application>>,
}

pub struct PipewireRegistry {
    device_update_receiver: UnboundedReceiver<PipewireDeviceUpdate>,
    port_update_receiver: UnboundedReceiver<PipewirePortUpdate>,
    node_update_receiver: UnboundedReceiver<PipewireNodeUpdate>,
    application_update_receiver: UnboundedReceiver<PipewireApplicationUpdate>,
    node_list_request_receiver: UnboundedReceiver<GetNodesListRequest>,
    port_list_request_receiver: UnboundedReceiver<GetPortsListRequest>,
    application_list_request_receiver: UnboundedReceiver<GetApplicationsListRequest>,
    device_list_request_receiver: UnboundedReceiver<GetDevicesListRequest>,
    ports: BTreeMap<(PortDirection, u16, u16), Port>,
    nodes: Vec<Node>,
    applications: Vec<Application>,
    devices: Vec<Device>,
}

impl PipewireRegistry {
    pub fn new(
        device_update_receiver: UnboundedReceiver<PipewireDeviceUpdate>,
        port_update_receiver: UnboundedReceiver<PipewirePortUpdate>,
        node_update_receiver: UnboundedReceiver<PipewireNodeUpdate>,
        application_update_receiver: UnboundedReceiver<PipewireApplicationUpdate>,
        node_list_request_receiver: UnboundedReceiver<GetNodesListRequest>,
        port_list_request_receiver: UnboundedReceiver<GetPortsListRequest>,
        application_list_request_receiver: UnboundedReceiver<GetApplicationsListRequest>,
        device_list_request_receiver: UnboundedReceiver<GetDevicesListRequest>,
    ) -> Self {
        PipewireRegistry {
            device_update_receiver,
            port_update_receiver,
            node_update_receiver,
            application_update_receiver,
            node_list_request_receiver,
            port_list_request_receiver,
            application_list_request_receiver,
            device_list_request_receiver,
            ports: BTreeMap::new(),
            nodes: Vec::new(),
            applications: Vec::new(),
            devices: Vec::new(),
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
                node_update = self.node_update_receiver.recv() => {
                    if let Some(node_update) = node_update {
                        logger.log_info("Received node update message");
                        let node = Node::from_pipewire_node_update(node_update);
                        self.nodes.push(node);
                    } else {
                        logger.log_info("Received None Message for node update");
                    }
                }
                application_update = self.application_update_receiver.recv() => {
                    if let Some(application_update) = application_update {
                        logger.log_info("Received application update message");
                        let application = Application::from_pipewire_application_update(application_update);
                        self.applications.push(application);
                    } else {
                        logger.log_info("Received None Message for node update");
                    }
                }
                device_update = self.device_update_receiver.recv() => {
                    if let Some(device_update) = device_update {
                        logger.log_info("Received device update message");
                        let device = Device::from_pipewire_device_update(device_update);
                        self.devices.push(device);

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
                application_list_request = self.application_list_request_receiver.recv() => {
                    logger.log_info("Received application list request");
                    application_list_request.unwrap().reply_sender.send(
                        self.applications.clone()
                    ).unwrap();
                }
                device_list_request = self.device_list_request_receiver.recv() => {
                    logger.log_info("Received device list request");
                    device_list_request.unwrap().reply_sender.send(self.devices.clone()).unwrap();
                }
            };
        }
    }
}
