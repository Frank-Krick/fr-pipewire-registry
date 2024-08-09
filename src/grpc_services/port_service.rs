use fr_pipewire_registry::port_server::{Port, PortServer};
use fr_pipewire_registry::{ListPort, ListPortsReply, ListPortsRequest, PortDirection};
use tonic::{Request, Response, Status};

use crate::pipewire_registry::GetPortsListRequest;

pub mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry.ports");
}

pub struct PortService {
    get_ports_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetPortsListRequest>,
}

impl PortService {
    pub fn new(
        get_ports_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetPortsListRequest>,
    ) -> Self {
        PortService {
            get_ports_list_request_sender,
        }
    }

    pub fn new_server(
        get_ports_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetPortsListRequest>,
    ) -> PortServer<PortService> {
        PortServer::new(Self::new(get_ports_list_request_sender))
    }
}

#[tonic::async_trait]
impl Port for PortService {
    async fn list_ports(
        &self,
        _request: Request<ListPortsRequest>,
    ) -> std::result::Result<Response<ListPortsReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        self.get_ports_list_request_sender
            .send(GetPortsListRequest {
                reply_sender: sender,
            })
            .unwrap();

        let service_reply = receiver.await.unwrap();
        let reply = ListPortsReply {
            ports: service_reply
                .into_iter()
                .map(|p| ListPort {
                    id: p.id as u32,
                    node_id: p.node_id as u32,
                    name: p.name,
                    direction: match p.direction {
                        crate::pipewire_registry::PortDirection::In => PortDirection::In as i32,
                        crate::pipewire_registry::PortDirection::Out => PortDirection::Out as i32,
                        crate::pipewire_registry::PortDirection::Unknown => {
                            PortDirection::Unknown as i32
                        }
                    },
                    physical: p.physical,
                    alias: p.alias,
                    group: p.group,
                    path: p.path,
                    dsp_format: p.dsp_format,
                    audio_channel: p.audio_channel,
                })
                .collect(),
        };
        Ok(Response::new(reply))
    }
}
