use std::error::Error;
use std::thread;

use rlg::log::Log;

mod grpc_services;
mod grpc_services_loop;
mod pipewire_event_consumer;
mod pipewire_factory;
mod pipewire_loop;
mod pipewire_registry;

fn main() -> Result<(), Box<dyn Error>> {
    fr_logging::setup_logging();
    let (logger_send, logger_receive) = tokio::sync::mpsc::unbounded_channel::<Log>();
    let logger_factory = fr_logging::LoggerFactory::new(logger_send);
    let _logger_thread = thread::spawn(move || fr_logging::run_logging_loop(logger_receive));

    let main_logger = logger_factory.new_logger(String::from("main_loop"));

    let (get_node_list_request_sender, get_node_list_request_receiver) =
        tokio::sync::mpsc::unbounded_channel();
    let (get_port_list_request_sender, get_port_list_request_receiver) =
        tokio::sync::mpsc::unbounded_channel();
    let (get_application_list_request_sender, get_application_list_request_receiver) =
        tokio::sync::mpsc::unbounded_channel();
    let (get_device_list_request_sender, get_device_list_request_receiver) =
        tokio::sync::mpsc::unbounded_channel();
    let grpc_logger = logger_factory.new_logger(String::from("gRPC Service"));

    main_logger.log_info("Starting grpc services");
    let _grpc_services_thread = thread::spawn(move || {
        grpc_services_loop::run_grpc_service(
            &grpc_logger,
            get_node_list_request_sender,
            get_port_list_request_sender,
            get_application_list_request_sender,
            get_device_list_request_sender,
        )
    });

    let (device_update_sender, device_update_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (port_update_sender, port_update_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (node_update_sender, node_update_receiver) = tokio::sync::mpsc::unbounded_channel();
    let (application_update_sender, application_update_receiver) =
        tokio::sync::mpsc::unbounded_channel();
    let pipewire_registry_logger = logger_factory.new_logger(String::from("pipewire_registry"));
    let _pipewire_registry_thread = thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let mut pipewire_registry = pipewire_registry::PipewireRegistry::new(
                    device_update_receiver,
                    port_update_receiver,
                    node_update_receiver,
                    application_update_receiver,
                    get_node_list_request_receiver,
                    get_port_list_request_receiver,
                    get_application_list_request_receiver,
                    get_device_list_request_receiver,
                );
                pipewire_registry.run(&pipewire_registry_logger).await;
            });
    });

    let pipewire_logger = logger_factory.new_logger(String::from("pipewire_loop"));
    let (factory_sender, factories_receiver) = tokio::sync::oneshot::channel();
    pipewire_loop::run_pipewire_loop(
        &pipewire_logger,
        device_update_sender,
        port_update_sender,
        node_update_sender,
        application_update_sender,
        factory_sender,
    )
    .unwrap();

    let _pipewire_factory_thread = thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let pipewire_factory =
                    pipewire_factory::PipewireFactory::wait_and_new(factories_receiver).await;
                pipewire_factory.run().await;
            });
    });

    Ok(())
}
