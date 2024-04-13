use crate::model::protocol_config::Protocol;
use crate::mycelink::mycelink_chat::MycelinkChat;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatConfig {
    Mycelink(MycelinkChat),
}

impl ChatConfig {
    pub fn protocol(&self) -> Protocol {
        match self {
            ChatConfig::Mycelink(_) => Protocol::Mycelink,
        }
    }
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

impl From<MycelinkChat> for ChatConfig {
    fn from(value: MycelinkChat) -> Self {
        ChatConfig::Mycelink(value)
    }
}
