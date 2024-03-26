#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod paxoskv;
pub mod proposer;
pub mod server;

use server::serve_acceptors;
use tonic::Status;

pub use crate::paxoskv::{BallotNum, PaxosInstanceId, Proposer, Value};
use crate::server::NOT_ENOUGH_QUORUM;

// test non-conflict paxos phase
#[tokio::test]
async fn test_non_conflict_paxos_phase() {
    let acceptor_ids = vec![1, 2, 3];
    serve_acceptors(&acceptor_ids).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut proposer = Proposer {
        id: Some(PaxosInstanceId { key: "i".to_string(), ver: 0 }),
        bal: Some(BallotNum { n: 1, proposer_id: 0 }),
        val: None,
    };
    let val = Some(Value { vi64: 1 });
    let res = proposer.run_paxos(acceptor_ids, val.clone()).await;
    assert_eq!(res, val);
}

// test conflict paxos phase
#[tokio::test]
async fn test_conflict_paxos_phase() {
    let acceptor_ids = vec![1, 2, 3];
    let quorum = 2;
    serve_acceptors(&acceptor_ids).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut px = Proposer {
        id: Some(PaxosInstanceId { key: "i".to_string(), ver: 0 }),
        bal: Some(BallotNum { n: 1, proposer_id: 11 }),
        val: None,
    };
    let px_val = Some(Value { vi64: 100 });
    let mut py = Proposer {
        id: Some(PaxosInstanceId { key: "i".to_string(), ver: 0 }),
        bal: Some(BallotNum { n: 2, proposer_id: 12 }),
        val: None,
    };
    let py_val = Some(Value { vi64: 200 });

    // px run paxos with phase 1
    let px_phase1 = px.phase1([1, 2].to_vec(), quorum).await.unwrap();
    assert_eq!(px_phase1, None);
    // py run paxos with phase 1
    let py_phase1 = py.phase1([2, 3].to_vec(), quorum).await.unwrap();
    assert_eq!(py_phase1, None);
    // px run paxos with phase 2
    px.val = px_val;
    let px_phase2 = px.phase2([2, 3].to_vec(), quorum).await;
    match px_phase2 {
        Ok(_) => panic!("px should not accept"),
        Err(err) => {
            eprintln!("px error: {:?}", err);
        }
    }
    // py run paxos with phase 2
    py.val = py_val.clone();
    let py_phase2 = py.phase2([1, 2].to_vec(), quorum).await.unwrap();

    //  reagain the px_phase1
    let px_phase1 = px.phase1([2, 3].to_vec(), quorum).await.unwrap();
    assert_eq!(px_phase1, py_val);
    assert_eq!(px.bal, Some(BallotNum { n: 3, proposer_id: 11 }));
    px.val = py_val;
    // px run paxos with phase 2
    px.phase2([1, 2].to_vec(), quorum).await.unwrap();
}

// test proposer run paxos
#[tokio::test]
async fn test_proposer_run_paxos() {
    let acceptor_ids = vec![1, 2, 3];
    serve_acceptors(&acceptor_ids).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // set key="i", ver=1
    {
        let mut proposer = Proposer {
            id: Some(PaxosInstanceId { key: "i".to_string(), ver: 1 }),
            bal: Some(BallotNum { n: 1, proposer_id: 1 }),
            val: None,
        };
        let val = Some(Value { vi64: 1 });
        let res = proposer.run_paxos(acceptor_ids.clone(), val.clone()).await;
        assert_eq!(res, val);
        // get the value of key="i", ver=1
        let res = proposer.run_paxos(acceptor_ids.clone(), None).await;
        assert_eq!(res, val);
    }
    // set key="i", ver=2
    {
        let mut proposer = Proposer {
            id: Some(PaxosInstanceId { key: "i".to_string(), ver: 2 }),
            bal: Some(BallotNum { n: 1, proposer_id: 1 }),
            val: None,
        };
        let val = Some(Value { vi64: 2 });
        let res = proposer.run_paxos(acceptor_ids.clone(), val.clone()).await;
        assert_eq!(res, val);
        // get the value of key="i", ver=2
        let res = proposer.run_paxos(acceptor_ids.clone(), None).await;
        assert_eq!(res, val);
    }
}
