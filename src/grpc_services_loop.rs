use fr_logging::Logger;
use tonic::transport::Server;

use crate::grpc_services::PipewireService;

use crate::pipewire_factory::PipewireFactoryRequest;
use crate::pipewire_registry::PipewireRegistryRequests;

pub fn run_grpc_service(
    logger: &Logger,
    request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
    pipewire_factory_request_sender: pipewire::channel::Sender<PipewireFactoryRequest>,
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
                .add_service(PipewireService::new_server(
                    request_sender,
                    pipewire_factory_request_sender,
                ))
                .serve(addr)
                .await
                .unwrap();
        });
}
