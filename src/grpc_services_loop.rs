use fr_pipewire_registry::node_server::{Node, NodeServer};
use fr_pipewire_registry::{ListNode, ListNodesReply, ListNodesRequest};
use tonic::{transport::Server, Request, Response, Status};

use crate::logging::Logger;
use crate::pipewire_registry::GetNodesListRequest;

pub mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry");
}

#[derive(Debug)]
pub struct NodeService {
    get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
}

impl NodeService {
    fn new(
        get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
    ) -> NodeService {
        NodeService {
            get_nodes_list_request_sender,
        }
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

pub fn run_grpc_service(
    logger: &Logger,
    get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
) {
    let node_service = NodeService::new(get_nodes_list_request_sender);
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            logger.log_info("Starting RPC services");
            let addr = match "127.0.0.1:50000".parse() {
                Ok(addr) => addr,
                Err(error) => {
                    panic!("{error:#?}");
                }
            };

            Server::builder()
                .add_service(NodeServer::new(node_service))
                .serve(addr)
                .await
                .unwrap();
        });
}
