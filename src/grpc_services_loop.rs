use tonic::transport::Server;

use crate::grpc_services::node_service::NodeService;
use crate::logging::Logger;
use crate::pipewire_registry::GetNodesListRequest;

pub fn run_grpc_service(
    logger: &Logger,
    get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
) {
    tokio::runtime::Builder::new_current_thread()
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
                .add_service(NodeService::new_server(get_nodes_list_request_sender))
                .serve(addr)
                .await
                .unwrap();
        });
}
