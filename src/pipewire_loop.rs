use std::cell::OnceCell;
use std::rc::Rc;

use anyhow::Result;

use fr_logging::Logger;

use pipewire::context::Context;
use pipewire::main_loop::MainLoop;
use pipewire::registry::Registry;
use pipewire::types::ObjectType;

use tokio::sync::mpsc::UnboundedSender as Sender;

use crate::pipewire_event_consumer::PipewireEventConsumer;
use crate::pipewire_event_consumer::PipewireUpdateEvent;
use crate::pipewire_factory::PipewireFactory;
use crate::pipewire_factory::PipewireFactoryRequest;

pub fn run_pipewire_loop(
    logger: &Logger,
    pipewire_update_event_sender: Sender<PipewireUpdateEvent>,
    pipewire_factory_request_receiver: pipewire::channel::Receiver<PipewireFactoryRequest>,
) -> Result<()> {
    logger.log_info("Starting Pipewire Loop");
    pipewire::init();
    let main_loop = MainLoop::new(None)?;
    let context = Context::new(&main_loop)?;
    let core = context.connect(None)?;
    let registry = core.get_registry()?;

    let consumer = PipewireEventConsumer::new(pipewire_update_event_sender);
    let listener = registry
        .add_listener_local()
        .global(move |global| consumer.process_pipewire_update(global))
        .register();

    let factories = get_factory_names(&main_loop, &registry).unwrap();
    let pipewire_factory = PipewireFactory { factories, core };

    let _receiver = pipewire_factory_request_receiver.attach(
        main_loop.loop_(),
        move |command: PipewireFactoryRequest| {
            pipewire_factory.process_command(command);
        },
    );
    main_loop.run();

    drop(listener);

    Ok(())
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Factories {
    pub link: String,
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
