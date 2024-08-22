use std::collections::BTreeMap;

use tokio::select;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::pipewire_event_consumer::PipewireUpdateEvent;

mod entities;

pub use crate::pipewire_registry::entities::{
    Application, Device, Link, Node, Port, PortDirection,
};

pub enum PipewireRegistryRequests {
    GetDevicesList {
        reply_sender: tokio::sync::oneshot::Sender<Vec<Device>>,
    },
    GetNodesList {
        reply_sender: tokio::sync::oneshot::Sender<Vec<Node>>,
    },
    ListPorts {
        reply_sender: tokio::sync::oneshot::Sender<Vec<Port>>,
    },
    GetApplicationsList {
        reply_sender: tokio::sync::oneshot::Sender<Vec<Application>>,
    },
    GetPortByObjectSerial {
        object_serial: u16,
        reply_sender: tokio::sync::oneshot::Sender<Option<Port>>,
    },
    ListLinks {
        reply_sender: tokio::sync::oneshot::Sender<Vec<Link>>,
    },
}

pub struct PipewireRegistry {
    pipewire_event_receiver: UnboundedReceiver<PipewireUpdateEvent>,
    pipewire_registry_request_receiver: UnboundedReceiver<PipewireRegistryRequests>,
    ports: BTreeMap<(PortDirection, u16, u16), Port>,
    nodes: Vec<Node>,
    applications: Vec<Application>,
    devices: Vec<Device>,
    links: Vec<Link>,
}

impl PipewireRegistry {
    pub fn new(
        pipewire_event_receiver: UnboundedReceiver<PipewireUpdateEvent>,
        pipewire_registry_request_receiver: UnboundedReceiver<PipewireRegistryRequests>,
    ) -> Self {
        PipewireRegistry {
            pipewire_event_receiver,
            pipewire_registry_request_receiver,
            ports: BTreeMap::new(),
            nodes: Vec::new(),
            applications: Vec::new(),
            devices: Vec::new(),
            links: Vec::new(),
        }
    }

    pub async fn run(&mut self) {
        loop {
            select! {
                pipewire_registry_request = self.pipewire_registry_request_receiver.recv() => {
                    self.process_registry_request(pipewire_registry_request.unwrap()).await;
                }
                pipewire_event = self.pipewire_event_receiver.recv() => {
                    self.process_pipewire_event(pipewire_event.unwrap()).await;
                }
            };
        }
    }

    async fn process_pipewire_event(&mut self, event: PipewireUpdateEvent) {
        match event {
            PipewireUpdateEvent::Link {
                object_serial,
                factory_id,
                client_id,
                output_port_id,
                input_port_id,
                output_node_id,
                input_node_id,
            } => self.links.push(Link {
                object_serial: object_serial.parse().unwrap_or(u16::MAX),
                factory_id: factory_id.parse().unwrap_or(u16::MAX),
                client_id: client_id.parse().unwrap_or(u16::MAX),
                output_port_id: output_port_id.parse().unwrap_or(u16::MAX),
                input_port_id: input_port_id.parse().unwrap_or(u16::MAX),
                output_node_id: output_node_id.parse().unwrap_or(u16::MAX),
                input_node_id: input_node_id.parse().unwrap_or(u16::MAX),
            }),
            PipewireUpdateEvent::Node {
                object_serial,
                factory_id,
                client_id,
                client_api,
                application_name,
                node_name,
                media_class,
            } => self.nodes.push(Node {
                object_serial: object_serial.parse().unwrap_or(u16::MAX),
                factory_id: factory_id.parse().unwrap_or(u16::MAX),
                client_id: client_id.parse().unwrap_or(u16::MAX),
                client_api,
                application_name,
                node_name,
                media_class,
            }),
            PipewireUpdateEvent::Device {
                name,
                factory_id,
                client_id,
                description,
                nick,
                media_class,
                object_serial,
            } => self.devices.push(Device {
                name,
                factory_id: factory_id.parse().unwrap_or(u16::MAX),
                client_id: client_id.parse().unwrap_or(u16::MAX),
                description,
                nick,
                media_class,
                object_serial: object_serial.parse().unwrap_or(u16::MAX),
            }),
            PipewireUpdateEvent::Port {
                id,
                name,
                direction,
                physical,
                alias,
                group,
                path,
                dsp_format,
                node_id,
                audio_channel,
                object_serial,
            } => {
                let port = Port {
                    id: id.parse().unwrap_or(u16::MAX),
                    node_id: node_id.parse().unwrap_or(u16::MAX),
                    name,
                    direction: match direction.as_str() {
                        "in" => PortDirection::In,
                        "out" => PortDirection::Out,
                        _ => PortDirection::Unknown,
                    },
                    physical: physical.as_str() == "true",
                    alias,
                    group,
                    path,
                    dsp_format,
                    audio_channel,
                    object_serial: object_serial.parse().unwrap_or(u16::MAX),
                };
                self.ports
                    .insert((port.direction, port.node_id, port.id), port);
            }
            PipewireUpdateEvent::Application {
                object_serial,
                module_id,
                pipewire_protocol,
                pipewire_sec_pid,
                pipewire_sec_uid,
                pipewire_sec_gid,
                pipewire_sec_socket,
                pipewire_access,
                name,
            } => self.applications.push(Application {
                object_serial: object_serial.parse().unwrap_or(u16::MAX),
                module_id: module_id.parse().unwrap_or(u16::MAX),
                pipewire_protocol,
                pipewire_sec_pid,
                pipewire_sec_uid,
                pipewire_sec_gid,
                pipewire_sec_socket,
                pipewire_access,
                name,
            }),
        }
    }

    async fn process_registry_request(&mut self, request: PipewireRegistryRequests) {
        match request {
            PipewireRegistryRequests::ListLinks { reply_sender } => {
                reply_sender.send(self.links.clone()).unwrap();
            }
            PipewireRegistryRequests::GetDevicesList { reply_sender } => {
                reply_sender.send(self.devices.clone()).unwrap();
            }
            PipewireRegistryRequests::GetNodesList { reply_sender } => {
                reply_sender.send(self.nodes.clone()).unwrap();
            }
            PipewireRegistryRequests::ListPorts { reply_sender } => {
                reply_sender
                    .send(self.ports.values().cloned().collect())
                    .unwrap();
            }
            PipewireRegistryRequests::GetApplicationsList { reply_sender } => {
                reply_sender.send(self.applications.clone()).unwrap();
            }
            PipewireRegistryRequests::GetPortByObjectSerial {
                object_serial,
                reply_sender,
            } => {
                let port = self
                    .ports
                    .iter()
                    .map(|p| p.1)
                    .find(|p| p.object_serial == object_serial);
                reply_sender.send(port.cloned()).unwrap();
            }
        }
    }
}
