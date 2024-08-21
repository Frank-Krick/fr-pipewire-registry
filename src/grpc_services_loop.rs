use fr_logging::Logger;
use tonic::transport::Server;

use crate::grpc_services::application_service::ApplicationService;
use crate::grpc_services::device_service::DeviceService;
use crate::grpc_services::node_service::NodeService;
use crate::grpc_services::port_service::PortService;

use crate::pipewire_registry::{
    GetApplicationsListRequest, GetDevicesListRequest, GetNodesListRequest, GetPortsListRequest,
    PipewireRegistryRequests,
};

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
                .add_service(NodeService::new_server(request_sender.clone()))
                .add_service(PortService::new_server(request_sender.clone()))
                .add_service(ApplicationService::new_server(request_sender.clone()))
                .add_service(DeviceService::new_server(request_sender))
                .serve(addr)
                .await
                .unwrap();
        });
}
