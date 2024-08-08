use fr_pipewire_registry::node_server::{Node, NodeServer};
use fr_pipewire_registry::{ListNode, ListNodesReply, ListNodesRequest};
use tonic::{Request, Response, Status};

use crate::pipewire_registry::GetNodesListRequest;

pub mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry.nodes");
}

#[derive(Debug)]
pub struct NodeService {
    get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
}

impl NodeService {
    pub fn new(
        get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
    ) -> Self {
        NodeService {
            get_nodes_list_request_sender,
        }
    }

    pub fn new_server(
        get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
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
        let service_request = GetNodesListRequest {
            reply_sender: sender,
        };
        self.get_nodes_list_request_sender
            .send(service_request)
            .unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListNodesReply {
            nodes: service_reply
                .into_iter()
                .map(|n| ListNode { name: n.node_name })
                .collect(),
        };

        Ok(Response::new(reply))
    }
}
