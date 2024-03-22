use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChatMessage {
    timestamp: u64,
    id: MycelinkChatMessageId,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChatMessageType {
    Standard {
        content: MycelinkChatMessageContent,
    },
    Reply {
        thread_start: MycelinkChatMessageId,
        content: MycelinkChatMessageContent,
    },
    Reaction {
        target_message: MycelinkChatMessageId,
        indicator: char,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChatMessageContent {
    Text(Box<str>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChatMessageId([u8; 32]);

impl AsRef<[u8]> for MycelinkChatMessageId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
