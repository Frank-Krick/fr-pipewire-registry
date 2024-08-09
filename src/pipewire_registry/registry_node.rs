use crate::pipewire_event_consumer::PipewireNodeUpdate;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Node {
    pub object_serial: u16,
    pub factory_id: u16,
    pub client_id: u16,
    pub client_api: String,
    pub application_name: String,
    pub node_name: String,
    pub media_class: String,
}

impl Node {
    pub fn from_pipewire_node_update(node_message: PipewireNodeUpdate) -> Self {
        Node {
            object_serial: node_message.object_serial.parse().unwrap_or(u16::MAX),
            factory_id: node_message.factory_id.parse().unwrap_or(u16::MAX),
            client_id: node_message.client_id.parse().unwrap_or(u16::MAX),
            client_api: node_message.client_api,
            application_name: node_message.application_name,
            node_name: node_message.node_name,
            media_class: node_message.media_class,
        }
    }
}
