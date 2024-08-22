use clap::{Parser, Subcommand};
use pmx::pipewire::pipewire_client::PipewireClient;
use pmx::pipewire::CreateLinkRequest;
use pmx::pipewire::ListApplicationsRequest;
use pmx::pipewire::ListDevicesRequest;
use pmx::pipewire::ListLinksRequest;
use pmx::pipewire::ListNodesRequest;
use pmx::pipewire::ListPortsRequest;
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
    ListPorts {
        #[arg(short, long)]
        node_id_filter: Option<u32>,
    },
    ListApplications {},
    ListDevices {},
    ListLinks {},
    CreateLink {
        #[arg(short = 'o', long)]
        output_port_id: u32,
        #[arg(short = 'i', long)]
        input_port_id: u32,
        #[arg(short = 'n', long)]
        output_node_id: u32,
        #[arg(short = 'm', long)]
        input_node_id: u32,
    },
}

pub mod pmx {
    pub mod pipewire {
        tonic::include_proto!("pmx.pipewire");

        pub mod node {
            tonic::include_proto!("pmx.pipewire.node");
        }

        pub mod port {
            tonic::include_proto!("pmx.pipewire.port");
        }

        pub mod application {
            tonic::include_proto!("pmx.pipewire.application");
        }

        pub mod device {
            tonic::include_proto!("pmx.pipewire.device");
        }

        pub mod link {
            tonic::include_proto!("pmx.pipewire.link");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_arguments = Arguments::parse();

    let mut client = PipewireClient::connect("http://127.0.0.1:50000").await?;

    if let Some(command) = cli_arguments.command {
        match command {
            Commands::CreateLink {
                output_port_id,
                input_port_id,
                output_node_id,
                input_node_id,
            } => {
                let request = Request::new(CreateLinkRequest {
                    output_port_id,
                    input_port_id,
                    output_node_id,
                    input_node_id,
                });
                let response = client.create_link(request).await;
                println!("Response={response:#?}");
            }
            Commands::ListLinks {} => {
                let request = Request::new(ListLinksRequest {});
                let response = client.list_links(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListNodes {} => {
                let request = Request::new(ListNodesRequest {});
                let response = client.list_nodes(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListPorts { node_id_filter } => {
                let request = Request::new(ListPortsRequest { node_id_filter });
                let response = client.list_ports(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListApplications {} => {
                let request = Request::new(ListApplicationsRequest {});
                let response = client.list_applications(request).await?;
                println!("Response={response:#?}");
            }
            Commands::ListDevices {} => {
                let request = Request::new(ListDevicesRequest {});
                let response = client.list_devices(request).await?;
                println!("Response={response:#?}");
            }
        };
    }

    Ok(())
}
