use crate::mycelink::mycelink_chat::MycelinkChat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatConfig {
    Mycelink(MycelinkChat),
}
