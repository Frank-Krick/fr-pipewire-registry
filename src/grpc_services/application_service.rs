use fr_pipewire_registry::application_server::{Application, ApplicationServer};
use fr_pipewire_registry::{ListApplication, ListApplicationsReply, ListApplicationsRequest};
use tonic::{Request, Response, Status};

use crate::pipewire_registry::PipewireRegistryRequests;

pub mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry.application");
}

#[derive(Debug)]
pub struct ApplicationService {
    request_sender: tokio::sync::mpsc::UnboundedSender<PipewireRegistryRequests>,
}

impl ApplicationService {
    pub fn new(
        get_application_list_request_sender: tokio::sync::mpsc::UnboundedSender<
            PipewireRegistryRequests,
        >,
    ) -> Self {
        ApplicationService {
            request_sender: get_application_list_request_sender,
        }
    }

    pub fn new_server(
        get_application_list_request_sender: tokio::sync::mpsc::UnboundedSender<
            PipewireRegistryRequests,
        >,
    ) -> ApplicationServer<Self> {
        ApplicationServer::new(ApplicationService::new(get_application_list_request_sender))
    }
}

#[tonic::async_trait]
impl Application for ApplicationService {
    async fn list_applications(
        &self,
        _request: Request<ListApplicationsRequest>,
    ) -> Result<Response<ListApplicationsReply>, Status> {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let service_request = PipewireRegistryRequests::GetApplicationsListRequest {
            reply_sender: sender,
        };
        self.request_sender.send(service_request).unwrap();
        let service_reply = receiver.await.unwrap();
        let reply = ListApplicationsReply {
            applications: service_reply
                .into_iter()
                .map(|a| ListApplication {
                    object_serial: a.object_serial as u32,
                    module_id: a.module_id as u32,
                    pipewire_protocol: a.pipewire_protocol,
                    pipewire_sec_pid: a.pipewire_sec_pid,
                    pipewire_sec_uid: a.pipewire_sec_uid,
                    pipewire_sec_gid: a.pipewire_sec_gid,
                    pipewire_sec_socket: a.pipewire_sec_socket,
                    pipewire_access: a.pipewire_access,
                    name: a.name,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }
}
