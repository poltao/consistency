use core::time;
use std::{
    collections::{BTreeMap, BTreeSet},
    result::Result,
    sync::{Arc, RwLock},
};

use derivative::Derivative;
use derive_new::new as New;
use tonic::{transport::Server, Request, Response, Status};

use self::{raft_server::Raft as RaftTrait, raft_server::RaftServer};
use crate::raft::*;

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("raft_descriptor");

#[derive(Debug, New)]
pub struct Leading {
    granted_by: BTreeSet<u64>,
    progresses: BTreeMap<u64, Progress>,
    log_index_range: (u64, u64),
}

#[derive(Debug, Clone, Derivative, PartialEq, New)]
#[derivative(Default)]
pub struct Progress {
    acked: LogId,
    len: u64,
    /// It is a token to indicate if it can send an RPC, e.g., there is no inflight RPC sending.
    /// It is set to `None` when an RPC is sent, and set to `Some(())` when the RPC is finished.
    #[derivative(Default(value = "Some(())"))]
    ready: Option<()>,
}

#[derive(Debug, Default)]
pub struct Store {
    /// the raft instance id
    id: u32,
    /// the candidate term id
    term: u64,
    /// the server voted leader_id for in current term
    voted_for: u32,
    /// log entries configs, just for membership
    configs: BTreeMap<u64, Vec<BTreeSet<u64>>>,
    /// log entries
    logs: Vec<Log>,
}

impl Store {
    pub fn new(id: u32) -> Self {
        Store {
            id,
            term: 0,
            voted_for: 0,
            configs: BTreeMap::new(),
            logs: Vec::new(),
        }
    }

    pub fn get_last(&self) -> LogId {
        let last = self.logs.last();
        match last {
            Some(log) => log.id.clone().unwrap(),
            None => LogId::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Raft {
    /// raft instance id
    pub id: u32,
    /// raft peers process info, for leader
    pub leading: Arc<RwLock<Option<Leading>>>,
    /// raft instance committed log index
    pub commit: Arc<RwLock<u64>>,
    /// raft peers client
    pub peers: Arc<RwLock<Vec<String>>>,
    /// raft instance backend storage impl
    pub sto: Arc<RwLock<Store>>,
    /// last time heart beat timestamp
    pub last_hb: Arc<RwLock<u128>>,
}

impl Raft {
    /// id is the raft instance id
    /// peers is a string of peer addresses, separated by commas, like '127.0.0.1:8080,127.0.0.1:8081'
    pub fn new(id: u32, peers: String) -> Self {
        let peers = peers.split(',').map(|peer| peer.to_string()).collect();
        let cur_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();

        Raft {
            id,
            leading: Arc::new(RwLock::new(None)),
            commit: Arc::new(RwLock::new(0)),
            peers: Arc::new(RwLock::new(peers)),
            sto: Arc::new(RwLock::new(Store::new(id))),
            last_hb: Arc::new(RwLock::new(cur_ts)),
        }
    }

    // run the raft instance
    pub async fn run(instance: Raft) {
        let peers = instance.peers.read().unwrap().clone();
        let addr = peers[instance.id as usize].clone();

        let reflection_service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();
        println!("Acceptors server listening on: {}", addr);
        let exec_result = Server::builder()
            .add_service(RaftServer::new(instance))
            .add_service(reflection_service)
            .serve(addr.parse().unwrap())
            .await;
        if let Err(e) = exec_result {
            eprintln!("Failed to serve acceptor: {}", e);
        }
    }

    pub async fn scheduler(&self) {
        loop {
            let cur_ts = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
            let last_hb = { *self.last_hb.read().unwrap() };

            if cur_ts - last_hb > 1000 {
                // request elect rpc
                let (term, last_log_id) = {
                    let sto_read = self.sto.read().unwrap();
                    (sto_read.term, Some(sto_read.get_last()))
                };
                let request = ElectRequest {
                    id: self.id,
                    term,
                    last_log_id,
                };
                {
                    let mut sto = self.sto.write().unwrap();
                    sto.term += 1;
                }
                let _ = self.elect(Request::new(request)).await;
            }
            {
                let mut last_hb = self.last_hb.write().unwrap();
                *last_hb = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis();
            }
            tokio::time::sleep(time::Duration::from_millis(100)).await;
        }
    }
}

#[tonic::async_trait]
impl RaftTrait for Raft {
    async fn elect(&self, request: Request<ElectRequest>) -> Result<Response<ElectResponse>, Status> {
        let req = request.into_inner();
        let mut sto = self.sto.write().unwrap();
        let mut resp = ElectResponse::default();
        if sto.voted_for == 0 {
            sto.voted_for = req.id;
            resp.granted = true;
        }
        Ok(Response::new(resp))
    }

    async fn append_log(&self, request: Request<AppendLogRequest>) -> Result<Response<AppendLogResponse>, Status> {
        let mut req = request.into_inner();
        let mut sto = self.sto.write().unwrap();
        let mut resp = AppendLogResponse::default();
        if sto.voted_for == req.id {
            sto.logs.append(&mut req.log);
            resp.success = true;
        }
        Ok(Response::new(resp))
    }
}
