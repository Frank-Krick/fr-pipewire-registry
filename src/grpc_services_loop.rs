use fr_logging::Logger;
use tonic::transport::Server;

use crate::grpc_services::PipewireService;

use crate::pipewire_registry::PipewireRegistryRequests;

pub fn run_grpc_service(
    logger: &Logger,
    request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
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
                .add_service(PipewireService::new_server(request_sender))
                .serve(addr)
                .await
                .unwrap();
        });
}
