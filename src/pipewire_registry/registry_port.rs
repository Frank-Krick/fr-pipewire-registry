use crate::pipewire_event_consumer::PipewirePortUpdate;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Port {
    pub id: u16,
    pub node_id: u16,
    pub name: String,
    pub direction: PortDirection,
    pub physical: bool,
    pub alias: String,
    pub group: String,
    pub path: String,
    pub dsp_format: String,
    pub audio_channel: String,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
#[allow(dead_code)]
pub enum PortDirection {
    In,
    Out,
    Unknown,
}

impl Port {
    pub fn from_pipewire_port_update(port_message: PipewirePortUpdate) -> Self {
        Port {
            id: port_message.id.parse().unwrap(),
            node_id: port_message.node_id.parse().unwrap(),
            name: port_message.name,
            direction: match port_message.direction.as_str() {
                "in" => PortDirection::In,
                "out" => PortDirection::Out,
                _ => PortDirection::Unknown,
            },
            physical: port_message.physical.as_str() == "true",
            alias: port_message.alias,
            group: port_message.group,
            path: port_message.path,
            dsp_format: port_message.dsp_format,
            audio_channel: port_message.audio_channel,
        }
    }
}
