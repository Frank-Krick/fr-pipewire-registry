use clap::{Parser, Subcommand};
use fr_pipewire_registry::applications::application_client::ApplicationClient;
use fr_pipewire_registry::applications::ListApplicationsRequest;
use fr_pipewire_registry::devices::device_client::DeviceClient;
use fr_pipewire_registry::devices::ListDevicesRequest;
use fr_pipewire_registry::nodes::node_client::NodeClient;
use fr_pipewire_registry::nodes::ListNodesRequest;
use fr_pipewire_registry::ports::port_client::PortClient;
use fr_pipewire_registry::ports::ListPortsRequest;
use std::error::Error;
use tonic::Request;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ListNodes {},
    ListPorts {},
    ListApplications {},
    ListDevices {},
}

pub mod fr_pipewire_registry {
    pub mod nodes {
        tonic::include_proto!("fr_pipewire_registry.nodes");
    }

    pub mod ports {
        tonic::include_proto!("fr_pipewire_registry.ports");
    }

    pub mod applications {
        tonic::include_proto!("fr_pipewire_registry.application");
    }

    pub mod devices {
        tonic::include_proto!("fr_pipewire_registry.device");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_arguments = Arguments::parse();

    let mut node_client = NodeClient::connect("http://127.0.0.1:50000").await?;
    let mut port_client = PortClient::connect("http://127.0.0.1:50000").await?;
    let mut applications_client = ApplicationClient::connect("http://127.0.0.1:50000").await?;
    let mut device_client = DeviceClient::connect("http://127.0.0.1:50000").await?;

    if let Some(command) = cli_arguments.command {
        match command {
            Commands::ListNodes {} => {
                let request = Request::new(ListNodesRequest {});
                let response = node_client.list_nodes(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListPorts {} => {
                let request = Request::new(ListPortsRequest {});
                let response = port_client.list_ports(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListApplications {} => {
                let request = Request::new(ListApplicationsRequest {});
                let response = applications_client.list_applications(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListDevices {} => {
                let request = Request::new(ListDevicesRequest {});
                let response = device_client.list_devices(request).await?;
                println!("Response={response:#?}");
            }
        };
    }

    Ok(())
}
