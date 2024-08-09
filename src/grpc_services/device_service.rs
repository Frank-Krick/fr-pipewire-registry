use fr_pipewire_registry::device_server::{Device, DeviceServer};
use fr_pipewire_registry::{ListDevice, ListDevicesReply, ListDevicesRequest};
use tonic::{Request, Response, Status};

use crate::pipewire_registry::GetDevicesListRequest;

mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry.device");
}

#[derive(Debug)]
pub struct DeviceService {
    get_devices_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetDevicesListRequest>,
}

#[tonic::async_trait]
impl Device for DeviceService {
    async fn list_devices(
        &self,
        _request: Request<ListDevicesRequest>,
    ) -> Result<Response<ListDevicesReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = GetDevicesListRequest {
            reply_sender: sender,
        };
        self.get_devices_list_request_sender
            .send(service_request)
            .unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListDevicesReply {
            devices: service_reply
                .into_iter()
                .map(|d| ListDevice {
                    factory_id: d.factory_id as u32,
                    object_serial: d.object_serial as u32,
                    client_id: d.client_id as u32,
                    name: d.name,
                    description: d.description,
                    nick: d.nick,
                    media_class: d.media_class,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }
}

impl DeviceService {
    pub fn new_server(
        get_devices_list_request_sender: tokio::sync::mpsc::UnboundedSender<GetDevicesListRequest>,
    ) -> DeviceServer<Self> {
        DeviceServer::new(DeviceService {
            get_devices_list_request_sender,
        })
    }
}
