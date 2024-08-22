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

    let grpc_logger = logger_factory.new_logger(String::from("gRPC Service"));

    let (pipewire_registry_request_sender, pipewire_registry_request_receiver) =
        tokio::sync::mpsc::unbounded_channel();

    main_logger.log_info("Starting grpc services");
    let _grpc_services_thread = thread::spawn(move || {
        grpc_services_loop::run_grpc_service(&grpc_logger, pipewire_registry_request_sender)
    });

    let (pipewire_event_sender, pipewire_event_receiver) = tokio::sync::mpsc::unbounded_channel();

    let _pipewire_registry_thread = thread::spawn(move || {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let mut pipewire_registry = pipewire_registry::PipewireRegistry::new(
                    pipewire_event_receiver,
                    pipewire_registry_request_receiver,
                );
                pipewire_registry.run().await;
            });
    });

    let pipewire_logger = logger_factory.new_logger(String::from("pipewire_loop"));
    let (factory_sender, factories_receiver) = tokio::sync::oneshot::channel();

    pipewire_loop::run_pipewire_loop(&pipewire_logger, pipewire_event_sender, factory_sender)
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
