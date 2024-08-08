use clap::{Parser, Subcommand};
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
}

pub mod fr_pipewire_registry {
    pub mod nodes {
        tonic::include_proto!("fr_pipewire_registry.nodes");
    }

    pub mod ports {
        tonic::include_proto!("fr_pipewire_registry.ports");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_arguments = Arguments::parse();

    let mut node_client = NodeClient::connect("http://127.0.0.1:50000").await?;
    let mut port_client = PortClient::connect("http://127.0.0.1:50000").await?;

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
        };
    }

    Ok(())
}
