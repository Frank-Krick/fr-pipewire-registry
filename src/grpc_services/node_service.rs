use fr_pipewire_registry::node_server::{Node, NodeServer};
use fr_pipewire_registry::{ListNode, ListNodesReply, ListNodesRequest};
use tonic::{Request, Response, Status};

use crate::pipewire_registry::PipewireRegistryRequests;

pub mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry.nodes");
}

#[derive(Debug)]
pub struct NodeService {
    request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
}

impl NodeService {
    pub fn new(
        get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
    ) -> Self {
        NodeService {
            request_sender: get_nodes_list_request_sender,
        }
    }

    pub fn new_server(
        get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
    ) -> NodeServer<NodeService> {
        NodeServer::new(Self::new(get_nodes_list_request_sender))
    }
}

#[tonic::async_trait]
impl Node for NodeService {
    async fn list_nodes(
        &self,
        _request: Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = PipewireRegistryRequests::GetNodesListRequest {
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
}
