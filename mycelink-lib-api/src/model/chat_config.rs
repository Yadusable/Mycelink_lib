use crate::mycelink::mycelink_chat::MycelinkChat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatConfig {
    Mycelink(MycelinkChat),
}

impl TryFrom<ChatConfig> for MycelinkChat {
    type Error = ();

    fn try_from(value: ChatConfig) -> Result<Self, Self::Error> {
        if let ChatConfig::Mycelink(chat) = value {
            Ok(chat)
        } else {
            Err(())
        }
    }
}
