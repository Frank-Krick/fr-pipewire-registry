use std::error::Error;

use fr_pipewire_registry::node_client::NodeClient;
use fr_pipewire_registry::ListNodesRequest;
use tonic::Request;

pub mod fr_pipewire_registry {
    tonic::include_proto!("fr_pipewire_registry");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = NodeClient::connect("http://127.0.0.1:50000").await?;

    let request = Request::new(ListNodesRequest {});

    let response = client.list_nodes(request).await?;

    println!("Response={response:#?}");

    Ok(())
}
