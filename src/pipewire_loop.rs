use std::cell::OnceCell;
use std::rc::Rc;

use anyhow::Result;

use pipewire::context::Context;
use pipewire::main_loop::MainLoop;
use pipewire::registry::Registry;
use pipewire::types::ObjectType;

use tokio::sync::mpsc::UnboundedSender as Sender;

use crate::config::MixerConfig;
use crate::logging::Logger;
use crate::pipewire_event_consumer::PipewireApplicationUpdate;
use crate::pipewire_event_consumer::PipewireDeviceUpdate;
use crate::pipewire_event_consumer::PipewireEventConsumer;
use crate::pipewire_event_consumer::PipewireNodeUpdate;
use crate::pipewire_event_consumer::PipewirePortUpdate;

pub fn run_pipewire_loop(
    _mixer_config: &MixerConfig,
    logger: &Logger,
    pipewire_device_sender: Sender<PipewireDeviceUpdate>,
    pipewire_port_sender: Sender<PipewirePortUpdate>,
    pipewire_node_sender: Sender<PipewireNodeUpdate>,
    pipewire_application_sender: Sender<PipewireApplicationUpdate>,
    factory_names_one_shot_sender: tokio::sync::oneshot::Sender<Factories>,
) -> Result<()> {
    logger.log_info("Starting Pipewire Loop");
    pipewire::init();
    let main_loop = MainLoop::new(None)?;
    let context = Context::new(&main_loop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let consumer = PipewireEventConsumer::new(
        pipewire_device_sender,
        pipewire_port_sender,
        pipewire_node_sender,
        pipewire_application_sender,
    );
    let listener = registry
        .add_listener_local()
        .global(move |global| consumer.process_pipewire_update(global))
        .register();

    let factories = get_factory_names(&main_loop, &registry).unwrap();
    factory_names_one_shot_sender.send(factories).unwrap();

    main_loop.run();

    drop(listener);

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Factories {
    link: String,
}

fn get_factory_names(main_loop: &MainLoop, registry: &Registry) -> Result<Factories> {
    print!("Getting factory names");
    let factory: Rc<OnceCell<String>> = Rc::new(OnceCell::new());
    let factory_clone = factory.clone();
    let main_loop_weak = main_loop.downgrade();
    let listener = registry
        .add_listener_local()
        .global(move |global| {
            if let Some(main_loop) = main_loop_weak.upgrade() {
                main_loop.quit();
            }
            if let Some(props) = global.props {
                if props.get("factory.type.name") == Some(ObjectType::Link.to_str()) {
                    let factory_name = props.get("factory.name").expect("Factory has no name");
                    let _ = factory_clone.set(factory_name.to_owned());
                }

                if factory_clone.get().is_some() {}
            }
        })
        .register();

    main_loop.run();

    drop(listener);

    Ok(Factories {
        link: factory.get().unwrap().clone(),
    })
}
