use crate::{
    abi::{command_request::RequestData, *},
    error::KvError,
};

impl CommandRequest {
    /// 创建 HSET 命令
    pub fn new_hset(table: impl Into<String>, key: impl Into<String>, value: Value) -> Self {
        Self {
            request_data: Some(RequestData::Hset(Hset {
                table: table.into(),
                pair: Some(Kvpair::new(key, value)),
            })),
        }
    }
}

impl Kvpair {
    /// 创建一个新的 kv pair
    pub fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value: Some(value),
        }
    }
}

/// 从 String 转换成 Value
impl From<String> for Value {
    fn from(s: String) -> Self {
        Self {
            value: Some(value::Value::String(s)),
        }
    }
}

/// 从 &str 转换成 Value
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Self {
            value: Some(value::Value::String(s.into())),
        }
    }
}

/// 对存储的抽象，我们不关心数据存在哪儿，但需要定义外界如何和存储打交道
pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
    fn set(&self, table: &str, key: String, value: Value) -> Result<Option<Value>, KvError>;
    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;
    /// 从 HashTable 中删除一个 key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;
    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Vec<Kvpair>, KvError>;
    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = Kvpair>>, KvError>;
}

/// 对 Command 的处理的抽象
pub trait CommandService {
    /// 处理 Command，返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}

// impl CommandService for Hget {
//     fn execute(self, store: &impl Storage) -> CommandResponse {
//         match store.get(&self.table, &self.key) {
//             Ok(Some(v)) => v.into(),
//             Ok(None) => return KvError::NotFound(self.table, self.key).into(),
//             Err(e) => return e.into(),
//         }
//     }
// }

// // 从 Request 中得到 Response，目前处理 HGET/HGETALL/HSET
// pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse {
//     match cmd.request_data {
//         Some(RequestData::Hget(param)) => param.execute(store),
//         Some(RequestData::Hgetall(param)) => param.execute(store),
//         Some(RequestData::Hset(param)) => param.execute(store),
//         None => KvError::InvalidCommand("Request has no data".into()).into(),
//         _ => KvError::Internal("Not implemented".into()).into(),
//     }
// }
