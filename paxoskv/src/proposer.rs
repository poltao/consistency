use tonic::Status;

use crate::paxoskv::Value;
use crate::paxoskv::{paxos_kv_client::PaxosKvClient, Acceptor, BallotNum, Proposer};

use crate::server::{ACCEPTOR_BASE_PORT, NOT_ENOUGH_QUORUM};

impl Proposer {
    async fn rpc_to_quorum(&self, acceptor_ids: Vec<i64>, action: &str) -> Result<Vec<Acceptor>, Box<dyn std::error::Error>> {
        let mut acceptors = Vec::new();
        for id in acceptor_ids {
            let addr = format!("http://127.0.0.1:{}", ACCEPTOR_BASE_PORT + id);
            let mut client = PaxosKvClient::connect(addr).await?;
            let request = tonic::Request::new(self.clone());
            let response = match action {
                "prepare" => client.prepare(request).await?,
                "accept" => client.accept(request).await?,
                _ => return Err(Box::new(Status::invalid_argument("Invalid action"))),
            };
            let acceptor = response.into_inner();
            acceptors.push(acceptor);
        }
        Ok(acceptors)
    }

    // phase1 is used for prepare phase
    pub(crate) async fn phase1(&mut self, acceptor_ids: Vec<i64>, quorum: usize) -> Result<Option<Value>, Box<dyn std::error::Error>> {
        let replies = self.rpc_to_quorum(acceptor_ids, "prepare").await?;
        let mut highest_bal = match self.bal.as_ref() {
            Some(bal) => bal.to_owned(),
            None => return Err(Box::new(Status::invalid_argument("No ballot provided"))),
        };
        let voted_bal = highest_bal.clone();
        let mut max_vbal = Acceptor::new();
        let mut ok = 0;

        for reply in replies {
            let r_last_bal = match reply.last_bal.as_ref() {
                Some(bal) => bal.to_owned(),
                None => return Err(Box::new(Status::invalid_argument("Acceptor No last ballot provided"))),
            };
            // not a voted acceptor
            if voted_bal.less(&r_last_bal) {
                if highest_bal.less(&r_last_bal) {
                    highest_bal = r_last_bal.clone();
                }
                continue;
            }
            // voted acceptor
            ok += 1;
            let r_bal = match reply.v_bal.as_ref() {
                Some(bal) => bal.to_owned(),
                None => BallotNum::default(),
            };
            if r_bal.ge(max_vbal.v_bal.as_ref().unwrap()) {
                max_vbal = reply;
            }
            if ok >= quorum {
                return Ok(max_vbal.val);
            }
        }
        // not enough votes, need update ballot numer of proposer
        self.bal.as_mut().unwrap().n = highest_bal.n + 1;
        Err(Box::new(Status::unavailable(NOT_ENOUGH_QUORUM)))
    }

    // phase2 is used for accept phase
    pub(crate) async fn phase2(&mut self, acceptor_ids: Vec<i64>, quorum: usize) -> Result<(), Box<dyn std::error::Error>> {
        let replies = self.rpc_to_quorum(acceptor_ids, "accept").await?;
        let mut highest_bal = match self.bal.as_ref() {
            Some(bal) => bal.to_owned(),
            None => return Err(Box::new(Status::invalid_argument("No ballot provided"))),
        };
        let voted_bal = highest_bal.clone();
        let mut ok = 0;

        for reply in replies {
            let r_last_bal = match reply.last_bal.as_ref() {
                Some(bal) => bal.to_owned(),
                None => return Err(Box::new(Status::invalid_argument("Acceptor No last ballot provided"))),
            };
            // not a voted acceptor
            if voted_bal.less(&r_last_bal) {
                if highest_bal.less(&r_last_bal) {
                    highest_bal = r_last_bal.clone();
                }
                continue;
            }
            // voted acceptor
            ok += 1;
            if ok >= quorum {
                return Ok(());
            }
        }
        // not enough votes, need update ballot numer of proposer
        self.bal.as_mut().unwrap().n = highest_bal.n + 1;
        Err(Box::new(Status::unavailable(NOT_ENOUGH_QUORUM)))
    }

    pub async fn run_paxos(&mut self, acceptor_ids: Vec<i64>, mut val: Option<Value>) -> Option<Value> {
        let quorum = acceptor_ids.len() / 2 + 1;
        loop {
            self.val = None;
            let prepare_res = self.phase1(acceptor_ids.clone(), quorum).await;
            match prepare_res {
                Ok(r_val) => {
                    if r_val.is_some() {
                        val = r_val;
                    }
                }
                Err(err) => {
                    eprintln!("Prepare phase error: {:?}", err);
                    continue;
                }
            };
            if val.is_none() {
                println!("No value to propose");
                return None;
            }
            self.val = val.clone();
            let accept_res = self.phase2(acceptor_ids.clone(), quorum).await;
            match accept_res {
                Ok(_) => {
                    println!("Paxos success: {:?}", val);
                    return val;
                }
                Err(err) => {
                    eprintln!("Accept phase error: {:?}", err);
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_proposer() {
        let mut x = 0;
        loop {
            match x % 2 {
                0 => x += 1,
                1 => {
                    x += 1;
                    continue;
                }
                _ => panic!("unexpected"),
            }
            if x > 10 {
                break;
            }
            println!("{}", x);
        }
    }
}
