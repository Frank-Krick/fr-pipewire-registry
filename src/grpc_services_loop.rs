use tonic::transport::Server;

use crate::grpc_services::application_service::ApplicationService;
use crate::grpc_services::device_service::DeviceService;
use crate::grpc_services::node_service::NodeService;
use crate::grpc_services::port_service::PortService;

use crate::logging::Logger;
use crate::pipewire_registry::{
    GetApplicationsListRequest, GetDevicesListRequest, GetNodesListRequest, GetPortsListRequest,
};

pub fn run_grpc_service(
    logger: &Logger,
    get_nodes_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetNodesListRequest>,
    get_ports_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetPortsListRequest>,
    get_applications_list_request_sender: tokio::sync::mpsc::UnboundedSender<
        GetApplicationsListRequest,
    >,
    get_devices_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetDevicesListRequest>,
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
                .add_service(PortService::new_server(get_ports_list_request_sender))
                .add_service(ApplicationService::new_server(
                    get_applications_list_request_sender,
                ))
                .add_service(DeviceService::new_server(get_devices_list_request_sender))
                .serve(addr)
                .await
                .unwrap();
        });
}
