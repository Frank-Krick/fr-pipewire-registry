use crate::pipewire_event_consumer::PipewireDeviceUpdate;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Device {
    pub name: String,
    pub factory_id: u16,
    pub client_id: u16,
    pub description: String,
    pub nick: String,
    pub media_class: String,
    pub object_serial: u16,
}

impl Device {
    pub fn from_pipewire_device_update(device_message: PipewireDeviceUpdate) -> Self {
        Device {
            name: device_message.name,
            factory_id: device_message.factory_id.parse().unwrap_or(u16::MAX),
            client_id: device_message.client_id.parse().unwrap_or(u16::MAX),
            description: device_message.description,
            nick: device_message.nick,
            media_class: device_message.media_class,
            object_serial: device_message.object_serial.parse().unwrap_or(u16::MAX),
        }
    }
}
