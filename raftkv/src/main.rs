mod node;
mod raft;

use node::Raft;

async fn start_raft(id: u32, peers: &str) {
    let raft_instance = Raft::new(id, peers.to_string());
    let sch_instance = raft_instance.clone();
    // create a tokio task to run the raft instance
    tokio::spawn(async move {
        Raft::run(raft_instance).await;
    });

    // create a tokio task to run the raft instance scheduler
    tokio::spawn(async move {
        // sleep 1 s
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        sch_instance.scheduler().await;
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let peers = "127.0.0.1:9001,127.0.0.1:9002,127.0.0.1:9003";
    start_raft(0, peers).await;
    start_raft(1, peers).await;
    start_raft(2, peers).await;
    Ok(())
}
