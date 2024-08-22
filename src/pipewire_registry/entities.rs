#[derive(Debug, Clone)]
pub struct Application {
    pub object_serial: u16,
    pub module_id: u16,
    pub pipewire_protocol: String,
    pub pipewire_sec_pid: String,
    pub pipewire_sec_uid: String,
    pub pipewire_sec_gid: String,
    pub pipewire_sec_socket: String,
    pub pipewire_access: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub name: String,
    pub factory_id: u16,
    pub client_id: u16,
    pub description: String,
    pub nick: String,
    pub media_class: String,
    pub object_serial: u16,
}

#[derive(Copy, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub enum PortDirection {
    In,
    Out,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Port {
    pub id: u16,
    pub node_id: u16,
    pub object_serial: u16,
    pub name: String,
    pub direction: PortDirection,
    pub physical: bool,
    pub alias: String,
    pub group: String,
    pub path: String,
    pub dsp_format: String,
    pub audio_channel: String,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub object_serial: u16,
    pub factory_id: u16,
    pub client_id: u16,
    pub client_api: String,
    pub application_name: String,
    pub node_name: String,
    pub media_class: String,
}

#[derive(Debug, Clone)]
pub struct Link {
    pub object_serial: u16,
    pub factory_id: u16,
    pub client_id: u16,
    pub output_port_id: u16,
    pub input_port_id: u16,
    pub output_node_id: u16,
    pub input_node_id: u16,
}
