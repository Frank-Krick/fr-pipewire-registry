use crate::pipewire_event_consumer::PipewireApplicationUpdate;

#[derive(Debug, Clone)]
#[allow(dead_code)]
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

impl Application {
    pub fn from_pipewire_application_update(
        application_message: PipewireApplicationUpdate,
    ) -> Self {
        Application {
            object_serial: application_message
                .object_serial
                .parse()
                .unwrap_or(u16::MAX),
            module_id: application_message.module_id.parse().unwrap_or(u16::MAX),
            pipewire_protocol: application_message.pipewire_protocol,
            pipewire_sec_pid: application_message.pipewire_sec_pid,
            pipewire_sec_uid: application_message.pipewire_sec_uid,
            pipewire_sec_gid: application_message.pipewire_sec_gid,
            pipewire_sec_socket: application_message.pipewire_sec_socket,
            pipewire_access: application_message.pipewire_access,
            name: application_message.name,
        }
    }
}
