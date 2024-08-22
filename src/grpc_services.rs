use crate::pipewire_factory::PipewireFactoryRequest;
use crate::pipewire_registry::PipewireRegistryRequests;
use pmx::pipewire::pipewire_server::{Pipewire, PipewireServer};

use pmx::pipewire::application::ListApplication;
use pmx::pipewire::device::ListDevice;
use pmx::pipewire::node::ListNode;
use pmx::pipewire::port::ListPort;

use pmx::pipewire::{
    CreateLinkReply, CreateLinkRequest, GetPortByObjectSerialRequest, ListApplicationsReply,
    ListApplicationsRequest, ListDevicesReply, ListDevicesRequest, ListLinksReply,
    ListLinksRequest, ListNodesReply, ListNodesRequest, ListPortsReply, ListPortsRequest,
};

use std::result::Result;
use tonic::{Request, Response, Status};

pub mod pmx {
    pub mod pipewire {
        tonic::include_proto!("pmx.pipewire");

        pub mod node {
            tonic::include_proto!("pmx.pipewire.node");
        }

        pub mod port {
            tonic::include_proto!("pmx.pipewire.port");
        }

        pub mod application {
            tonic::include_proto!("pmx.pipewire.application");
        }

        pub mod device {
            tonic::include_proto!("pmx.pipewire.device");
        }

        pub mod link {
            tonic::include_proto!("pmx.pipewire.link");
        }
    }
}

pub struct PipewireService {
    request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
    pipewire_factory_request_sender: pipewire::channel::Sender<PipewireFactoryRequest>,
}

impl PipewireService {
    pub fn new_server(
        request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
        pipewire_factory_request_sender: pipewire::channel::Sender<PipewireFactoryRequest>,
    ) -> PipewireServer<Self> {
        PipewireServer::new(PipewireService {
            request_sender,
            pipewire_factory_request_sender,
        })
    }
}

#[tonic::async_trait]
impl Pipewire for PipewireService {
    async fn create_link(
        &self,
        request: Request<CreateLinkRequest>,
    ) -> Result<Response<CreateLinkReply>, Status> {
        let inner = request.into_inner();
        self.pipewire_factory_request_sender
            .send(PipewireFactoryRequest::CreateLink {
                output_port_id: inner.output_port_id.to_string(),
                input_port_id: inner.input_port_id.to_string(),
                output_node_id: inner.output_node_id.to_string(),
                input_node_id: inner.input_node_id.to_string(),
            })
            .unwrap();
        Ok(Response::new(CreateLinkReply {}))
    }

    async fn list_applications(
        &self,
        _request: Request<ListApplicationsRequest>,
    ) -> Result<Response<ListApplicationsReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = PipewireRegistryRequests::GetApplicationsList {
            reply_sender: sender,
        };
        self.request_sender.send(service_request).unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListApplicationsReply {
            applications: service_reply
                .into_iter()
                .map(|a| ListApplication {
                    object_serial: a.object_serial as u32,
                    module_id: a.module_id as u32,
                    pipewire_protocol: a.pipewire_protocol,
                    pipewire_sec_pid: a.pipewire_sec_pid,
                    pipewire_sec_uid: a.pipewire_sec_uid,
                    pipewire_sec_gid: a.pipewire_sec_gid,
                    pipewire_sec_socket: a.pipewire_sec_socket,
                    pipewire_access: a.pipewire_access,
                    name: a.name,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn list_links(
        &self,
        _request: Request<ListLinksRequest>,
    ) -> Result<Response<ListLinksReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = PipewireRegistryRequests::ListLinks {
            reply_sender: sender,
        };
        self.request_sender.send(service_request).unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListLinksReply {
            links: service_reply
                .into_iter()
                .map(|l| pmx::pipewire::link::Link {
                    object_serial: l.object_serial as u32,
                    factory_id: l.factory_id as u32,
                    client_id: l.client_id as u32,
                    output_port_id: l.output_port_id as u32,
                    input_port_id: l.input_port_id as u32,
                    output_node_id: l.output_node_id as u32,
                    input_node_id: l.input_node_id as u32,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn list_nodes(
        &self,
        _request: tonic::Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = PipewireRegistryRequests::GetNodesList {
            reply_sender: sender,
        };
        self.request_sender.send(service_request).unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListNodesReply {
            nodes: service_reply
                .into_iter()
                .map(|n| ListNode {
                    object_serial: n.object_serial as u32,
                    factory_id: n.factory_id as u32,
                    client_id: n.client_id as u32,
                    client_api: n.client_api,
                    application_name: n.application_name,
                    name: n.node_name,
                    media_class: n.media_class,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn list_devices(
        &self,
        _request: Request<ListDevicesRequest>,
    ) -> Result<Response<ListDevicesReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = PipewireRegistryRequests::GetDevicesList {
            reply_sender: sender,
        };
        self.request_sender.send(service_request).unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListDevicesReply {
            devices: service_reply
                .into_iter()
                .map(|d| ListDevice {
                    factory_id: d.factory_id as u32,
                    object_serial: d.object_serial as u32,
                    client_id: d.client_id as u32,
                    name: d.name,
                    description: d.description,
                    nick: d.nick,
                    media_class: d.media_class,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn list_ports(
        &self,
        request: Request<ListPortsRequest>,
    ) -> Result<Response<ListPortsReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.request_sender
            .send(PipewireRegistryRequests::ListPorts {
                reply_sender: sender,
            })
            .unwrap();

        let service_reply = receiver.await.unwrap().into_iter();

        if let Some(node_id_filter) = request.into_inner().node_id_filter {
            let reply = ListPortsReply {
                ports: service_reply
                    .filter(|p| p.node_id == node_id_filter as u16)
                    .map(|p| ListPort {
                        id: p.id as u32,
                        node_id: p.node_id as u32,
                        name: p.name,
                        direction: match p.direction {
                            crate::pipewire_registry::PortDirection::In => {
                                pmx::pipewire::port::PortDirection::In as i32
                            }
                            crate::pipewire_registry::PortDirection::Out => {
                                pmx::pipewire::port::PortDirection::Out as i32
                            }
                            crate::pipewire_registry::PortDirection::Unknown => {
                                pmx::pipewire::port::PortDirection::Unknown as i32
                            }
                        },
                        physical: p.physical,
                        alias: p.alias,
                        group: p.group,
                        path: p.path,
                        dsp_format: p.dsp_format,
                        audio_channel: p.audio_channel,
                        object_serial: p.object_serial as u32,
                    })
                    .collect(),
            };
            Ok(Response::new(reply))
        } else {
            let reply = ListPortsReply {
                ports: service_reply
                    .map(|p| ListPort {
                        id: p.id as u32,
                        node_id: p.node_id as u32,
                        name: p.name,
                        direction: match p.direction {
                            crate::pipewire_registry::PortDirection::In => {
                                pmx::pipewire::port::PortDirection::In as i32
                            }
                            crate::pipewire_registry::PortDirection::Out => {
                                pmx::pipewire::port::PortDirection::Out as i32
                            }
                            crate::pipewire_registry::PortDirection::Unknown => {
                                pmx::pipewire::port::PortDirection::Unknown as i32
                            }
                        },
                        physical: p.physical,
                        alias: p.alias,
                        group: p.group,
                        path: p.path,
                        dsp_format: p.dsp_format,
                        audio_channel: p.audio_channel,
                        object_serial: p.object_serial as u32,
                    })
                    .collect(),
            };
            Ok(Response::new(reply))
        }
    }

    async fn get_port_by_object_serial(
        &self,
        request: Request<GetPortByObjectSerialRequest>,
    ) -> Result<tonic::Response<ListPort>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.request_sender
            .send(PipewireRegistryRequests::GetPortByObjectSerial {
                object_serial: request.into_inner().object_serial as u16,
                reply_sender: sender,
            })
            .unwrap();
        let response = receiver.await;
        if let Some(port) = response.unwrap() {
            Ok(Response::new(ListPort {
                id: port.id as u32,
                node_id: port.node_id as u32,
                name: port.name,
                direction: match port.direction {
                    crate::pipewire_registry::PortDirection::In => {
                        pmx::pipewire::port::PortDirection::In as i32
                    }
                    crate::pipewire_registry::PortDirection::Out => {
                        pmx::pipewire::port::PortDirection::Out as i32
                    }
                    crate::pipewire_registry::PortDirection::Unknown => {
                        pmx::pipewire::port::PortDirection::Unknown as i32
                    }
                },
                physical: port.physical,
                alias: port.alias,
                group: port.group,
                path: port.path,
                dsp_format: port.dsp_format,
                audio_channel: port.audio_channel,
                object_serial: port.object_serial as u32,
            }))
        } else {
            Err(Status::not_found("Port not found"))
        }
    }
}
