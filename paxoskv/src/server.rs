use std::fmt::Debug;
use std::{collections::HashMap, result::Result, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use crate::paxoskv::{
    paxos_kv_server::{PaxosKv, PaxosKvServer},
    Acceptor, BallotNum, PaxosInstanceId, Proposer,
};

pub const ACCEPTOR_BASE_PORT: i64 = 3333;
pub const NOT_ENOUGH_QUORUM: &str = "Not enough acceptors to form a quorum";
pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("paxoskv_descriptor");

#[derive(Clone, Debug)]
struct Version {
    acceptor: Arc<Mutex<Acceptor>>,
}

impl Default for Version {
    fn default() -> Self {
        Version {
            acceptor: Arc::new(Mutex::new(Acceptor::new())),
        }
    }
}

impl Acceptor {
    pub fn new() -> Self {
        Acceptor {
            last_bal: Some(BallotNum { n: 0, proposer_id: 0 }),
            v_bal: Some(BallotNum { n: 0, proposer_id: 0 }),
            val: None,
        }
    }
}

type Versions = HashMap<i64, Version>;

#[derive(Debug)]
pub struct KVServer {
    storage: Arc<Mutex<HashMap<String, Versions>>>,
}

impl Default for KVServer {
    fn default() -> Self {
        KVServer {
            storage: Arc::new(Mutex::new(HashMap::<String, Versions>::new())),
        }
    }
}

impl KVServer {
    async fn get_mutex_version(&self, id: Option<PaxosInstanceId>) -> Result<Version, Status> {
        let id = match id.as_ref() {
            Some(id) => id.to_owned(),
            None => return Err(Status::invalid_argument("No ID provided")),
        };
        if id.key == "" {
            return Err(Status::invalid_argument("Empty key provided"));
        }
        let mut storage = self.storage.lock().await;
        let versions = storage.entry(id.key).or_insert(Versions::default());
        let version = versions.entry(id.ver).or_insert(Version::default());
        Ok(version.to_owned())
    }
}

impl BallotNum {
    pub fn ge(&self, other: &BallotNum) -> bool {
        self.n > other.n || (self.n == other.n && self.proposer_id >= other.proposer_id)
    }

    pub fn less(&self, other: &BallotNum) -> bool {
        self.n < other.n || (self.n == other.n && self.proposer_id < other.proposer_id)
    }
}

#[tonic::async_trait]
impl PaxosKv for KVServer {
    async fn prepare(&self, request: Request<Proposer>) -> Result<Response<Acceptor>, Status> {
        let proposer = request.into_inner();

        let version = self.get_mutex_version(proposer.id.clone()).await?;
        let mut acceptor = version.acceptor.lock().await;

        let reply = acceptor.to_owned();

        let r_ballot = match proposer.bal {
            Some(bal) => bal,
            None => return Err(Status::invalid_argument("No ballot provided")),
        };
        if r_ballot.ge(acceptor.last_bal.as_ref().unwrap()) {
            acceptor.last_bal = Some(r_ballot);
        }
        Ok(Response::new(reply))
    }

    async fn accept(&self, request: Request<Proposer>) -> Result<Response<Acceptor>, Status> {
        let proposer = request.into_inner();

        let version = self.get_mutex_version(proposer.id).await?;
        let mut acceptor = version.acceptor.lock().await;
        let reply = acceptor.to_owned();

        let r_ballot = match proposer.bal {
            Some(bal) => bal,
            None => return Err(Status::invalid_argument("No ballot provided")),
        };
        if r_ballot.ge(acceptor.last_bal.as_ref().unwrap()) {
            acceptor.last_bal = Some(r_ballot.clone());
            acceptor.v_bal = Some(r_ballot);
            acceptor.val = proposer.val;
        }
        Ok(Response::new(reply))
    }
}

pub(crate) async fn serve_acceptors(acceptor_ids: &Vec<i64>) -> Result<(), Box<dyn std::error::Error>> {
    for &id in acceptor_ids {
        let addr = format!("127.0.0.1:{}", ACCEPTOR_BASE_PORT + id).parse()?;
        let kv_server = KVServer::default();
        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();
        tokio::spawn(async move {
            println!("Acceptors server listening on: {}", addr);
            let exec_result = Server::builder().add_service(PaxosKvServer::new(kv_server)).add_service(reflection_service).serve(addr).await;
            if let Err(e) = exec_result {
                eprintln!("Failed to serve acceptor: {}", e);
            }
        });
    }
    Ok(())
}
