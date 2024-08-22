use pipewire::core::Core;

use crate::pipewire_loop::Factories;

#[derive(Debug)]
pub enum PipewireFactoryRequest {
    CreateLink {
        output_port_id: String,
        input_port_id: String,
        output_node_id: String,
        input_node_id: String,
    },
}

pub struct PipewireFactory {
    pub factories: Factories,
    pub core: Core,
}

impl PipewireFactory {
    pub fn process_command(&self, request: PipewireFactoryRequest) {
        match request {
            PipewireFactoryRequest::CreateLink {
                output_port_id,
                input_port_id,
                output_node_id,
                input_node_id,
            } => {
                self.core
                    .create_object::<pipewire::link::Link>(
                        &self.factories.link,
                        &pipewire::properties::properties! {
                                    "link.output.port" => output_port_id,
                                    "link.input.port" => input_port_id,
                                    "link.output.node" => output_node_id,
                                    "link.input.node" => input_node_id,
                                    "object.linger" => "1"
                        },
                    )
                    .unwrap();
            }
        }
    }
}
