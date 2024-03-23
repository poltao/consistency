pub mod paxoskv;
pub mod server;

use paxoskv::paxos_kv_server::PaxosKvServer;
use server::KVServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:3004".parse()?;
    let inventory = KVServer::default();

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(server::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(PaxosKvServer::new(inventory))
        .add_service(reflection_service)
        .serve(addr)
        .await?;
    Ok(())
}
