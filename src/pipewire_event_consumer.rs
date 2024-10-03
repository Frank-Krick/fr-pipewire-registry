use libspa::utils::dict::DictRef;
use pipewire::registry::GlobalObject;

use tokio::sync::mpsc::UnboundedSender as Sender;

pub enum PipewireUpdateEvent {
    Device {
        name: String,
        factory_id: String,
        client_id: String,
        description: String,
        nick: String,
        media_class: String,
        object_serial: String,
    },
    Port {
        id: String,
        name: String,
        direction: String,
        physical: String,
        alias: String,
        group: String,
        path: String,
        dsp_format: String,
        node_id: String,
        audio_channel: String,
        object_serial: String,
    },
    Node {
        object_serial: String,
        factory_id: String,
        client_id: String,
        client_api: String,
        application_name: String,
        node_name: String,
        media_class: String,
    },
    Application {
        object_serial: String,
        module_id: String,
        pipewire_protocol: String,
        pipewire_sec_pid: String,
        pipewire_sec_uid: String,
        pipewire_sec_gid: String,
        pipewire_sec_socket: String,
        pipewire_access: String,
        name: String,
    },
    Link {
        object_serial: String,
        factory_id: String,
        client_id: String,
        output_port_id: String,
        input_port_id: String,
        output_node_id: String,
        input_node_id: String,
    },
}

pub struct PipewireEventConsumer {
    pipewire_update_event_sender: Sender<PipewireUpdateEvent>,
}

impl PipewireEventConsumer {
    pub fn new(pipewire_update_event_sender: Sender<PipewireUpdateEvent>) -> PipewireEventConsumer {
        PipewireEventConsumer {
            pipewire_update_event_sender,
        }
    }

    pub fn process_pipewire_update(&self, update: &GlobalObject<&DictRef>) {
        if let Some(props) = update.props {
            if props.keys().any(|p| p == "link.output.port") {
                self.pipewire_update_event_sender
                    .send(PipewireUpdateEvent::Link {
                        object_serial: String::from(props.get("object.serial").unwrap()),
                        factory_id: String::from(props.get("factory.id").unwrap()),
                        client_id: String::from(props.get("client.id").unwrap_or("")),
                        output_port_id: String::from(props.get("link.output.port").unwrap()),
                        input_port_id: String::from(props.get("link.input.port").unwrap()),
                        output_node_id: String::from(props.get("link.output.node").unwrap()),
                        input_node_id: String::from(props.get("link.input.node").unwrap()),
                    })
                    .unwrap();
                return;
            }

            if let Some(device_name) = props.get("device.name") {
                self.pipewire_update_event_sender
                    .send(PipewireUpdateEvent::Device {
                        name: String::from(device_name),
                        factory_id: String::from(props.get("factory.id").unwrap()),
                        client_id: String::from(props.get("client.id").unwrap()),
                        description: String::from(props.get("device.description").unwrap()),
                        nick: String::from(props.get("device.nick").unwrap_or("None")),
                        media_class: String::from(props.get("media.class").unwrap()),
                        object_serial: String::from(props.get("object.serial").unwrap()),
                    })
                    .unwrap();
                return;
            };

            if let Some(_value) = props.get("port.name") {
                self.pipewire_update_event_sender
                    .send(PipewireUpdateEvent::Port {
                        object_serial: String::from(props.get("object.serial").unwrap()),
                        id: String::from(props.get("port.id").unwrap()),
                        name: String::from(props.get("port.name").unwrap()),
                        direction: String::from(props.get("port.direction").unwrap()),
                        physical: String::from(props.get("port.physical").unwrap_or("")),
                        alias: String::from(props.get("port.alias").unwrap()),
                        group: String::from(props.get("port.group").unwrap_or("")),
                        path: String::from(props.get("object.path").unwrap()),
                        dsp_format: String::from(props.get("format.dsp").unwrap_or("None")),
                        node_id: String::from(props.get("node.id").unwrap()),
                        audio_channel: String::from(props.get("audio.channel").unwrap_or("")),
                    })
                    .unwrap();
                return;
            };

            if let Some(_value) = props.get("node.name") {
                self.pipewire_update_event_sender
                    .send(PipewireUpdateEvent::Node {
                        object_serial: String::from(props.get("object.serial").unwrap()),
                        factory_id: String::from(props.get("factory.id").unwrap_or("")),
                        client_id: String::from(props.get("client.id").unwrap_or("")),
                        client_api: String::from(props.get("client.api").unwrap_or("")),
                        application_name: String::from(props.get("application.name").unwrap_or("")),
                        node_name: String::from(props.get("node.name").unwrap_or("")),
                        media_class: String::from(props.get(" media.class").unwrap_or("")),
                    })
                    .unwrap();
                return;
            }

            if let Some(_value) = props.get("application.name") {
                let application = PipewireUpdateEvent::Application {
                    object_serial: String::from(props.get("object.serial").unwrap()),
                    module_id: String::from(props.get("module.id").unwrap_or("")),
                    pipewire_protocol: String::from(props.get("pipewire.protocol").unwrap_or("")),
                    pipewire_sec_pid: String::from(props.get("pipewire.sec.pid").unwrap_or("")),
                    pipewire_sec_uid: String::from(props.get("pipewire.sec.uid").unwrap_or("")),
                    pipewire_sec_gid: String::from(props.get("pipewire.sec.gid").unwrap_or("")),
                    pipewire_sec_socket: String::from(
                        props.get("pipewire.sec.socket").unwrap_or(""),
                    ),
                    pipewire_access: String::from(props.get("pipewire.access").unwrap_or("")),
                    name: String::from(props.get("application.name").unwrap_or("")),
                };
                self.pipewire_update_event_sender.send(application).unwrap();
                return;
            }

            println!("{props:#?}");
        }
    }
}
