/// KvError is all kinds of Error of kv-server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KvError {
    Internal(String),

    InvalidCommand(String),

    NotFound(String, String),
}
