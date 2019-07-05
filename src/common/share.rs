use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub msg: String,
    pub query:String,
    pub status:i32,
}
