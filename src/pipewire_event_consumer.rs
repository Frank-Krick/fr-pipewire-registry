use libspa::utils::dict::DictRef;
use pipewire::registry::GlobalObject;

use tokio::sync::mpsc::UnboundedSender as Sender;

#[derive(Debug)]
#[allow(dead_code)]
pub struct PipewireDeviceUpdate {
    pub name: String,
    pub factory_id: String,
    pub client_id: String,
    pub description: String,
    pub nick: String,
    pub media_class: String,
    pub object_serial: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PipewirePortUpdate {
    pub id: String,
    pub name: String,
    pub direction: String,
    pub physical: String,
    pub alias: String,
    pub group: String,
    pub path: String,
    pub dsp_format: String,
    pub node_id: String,
    pub audio_channel: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PipewireNodeUpdate {
    pub object_serial: String,
    pub factory_id: String,
    pub client_id: String,
    pub client_api: String,
    pub application_name: String,
    pub node_name: String,
    pub media_class: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct PipewireApplicationUpdate {
    pub object_serial: String,
    pub module_id: String,
    pub pipewire_protocol: String,
    pub pipewire_sec_pid: String,
    pub pipewire_sec_uid: String,
    pub pipewire_sec_gid: String,
    pub pipewire_sec_socket: String,
    pub pipewire_access: String,
    pub name: String,
}

pub struct PipewireEventConsumer {
    device_update_sender: Sender<PipewireDeviceUpdate>,
    port_update_sender: Sender<PipewirePortUpdate>,
    node_update_sender: Sender<PipewireNodeUpdate>,
    application_update_sender: Sender<PipewireApplicationUpdate>,
}

impl PipewireEventConsumer {
    pub fn new(
        device_update_sender: Sender<PipewireDeviceUpdate>,
        port_update_sender: Sender<PipewirePortUpdate>,
        node_update_sender: Sender<PipewireNodeUpdate>,
        application_update_sender: Sender<PipewireApplicationUpdate>,
    ) -> PipewireEventConsumer {
        PipewireEventConsumer {
            device_update_sender,
            port_update_sender,
            node_update_sender,
            application_update_sender,
        }
    }

    pub fn process_pipewire_update(&self, update: &GlobalObject<&DictRef>) {
        if let Some(props) = update.props {
            if let Some(device_name) = props.get("device.name") {
                self.device_update_sender
                    .send(PipewireDeviceUpdate {
                        name: String::from(device_name),
                        factory_id: String::from(props.get("factory.id").unwrap()),
                        client_id: String::from(props.get("client.id").unwrap()),
                        description: String::from(props.get("device.description").unwrap()),
                        nick: String::from(props.get("device.nick").unwrap()),
                        media_class: String::from(props.get("media.class").unwrap()),
                        object_serial: String::from(props.get("object.serial").unwrap()),
                    })
                    .unwrap();
                return;
            };

            if let Some(_value) = props.get("port.name") {
                let port = PipewirePortUpdate {
                    id: String::from(props.get("port.id").unwrap()),
                    name: String::from(props.get("port.name").unwrap()),
                    direction: String::from(props.get("port.direction").unwrap()),
                    physical: String::from(props.get("port.physical").unwrap_or("")),
                    alias: String::from(props.get("port.alias").unwrap()),
                    group: String::from(props.get("port.group").unwrap_or("")),
                    path: String::from(props.get("object.path").unwrap()),
                    dsp_format: String::from(props.get("format.dsp").unwrap()),
                    node_id: String::from(props.get("node.id").unwrap()),
                    audio_channel: String::from(props.get("audio.channel").unwrap_or("")),
                };
                self.port_update_sender.send(port).unwrap();
                return;
            };

            if let Some(_value) = props.get("node.name") {
                let node = PipewireNodeUpdate {
                    object_serial: String::from(props.get("object.serial").unwrap()),
                    factory_id: String::from(props.get("factory.id").unwrap_or("")),
                    client_id: String::from(props.get("client.id").unwrap_or("")),
                    client_api: String::from(props.get("client.api").unwrap_or("")),
                    application_name: String::from(props.get("application.name").unwrap_or("")),
                    node_name: String::from(props.get("node.name").unwrap_or("")),
                    media_class: String::from(props.get(" media.class").unwrap_or("")),
                };
                self.node_update_sender.send(node).unwrap();
                return;
            }

            if let Some(_value) = props.get("application.name") {
                let application = PipewireApplicationUpdate {
                    object_serial: String::from(props.get("object.serial").unwrap()),
                    module_id: String::from(props.get("module.id").unwrap()),
                    pipewire_protocol: String::from(props.get("pipewire.protocol").unwrap()),
                    pipewire_sec_pid: String::from(props.get("pipewire.sec.pid").unwrap()),
                    pipewire_sec_uid: String::from(props.get("pipewire.sec.uid").unwrap()),
                    pipewire_sec_gid: String::from(props.get("pipewire.sec.gid").unwrap()),
                    pipewire_sec_socket: String::from(props.get("pipewire.sec.socket").unwrap()),
                    pipewire_access: String::from(props.get("pipewire.access").unwrap()),
                    name: String::from(props.get("application.name").unwrap()),
                };
                self.application_update_sender.send(application).unwrap();
                return;
            }

            println!("{props:#?}");
        }
    }
}
