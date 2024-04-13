use crate::mycelink::compressed_box::{CompressionHint, CompressionHinting};
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MycelinkChatMessage {
    timestamp: u64,
    id: MycelinkChatMessageId,
    message_type: MycelinkChatMessageType,
}

impl MycelinkChatMessage {
    pub fn new(
        timestamp: u64,
        id: MycelinkChatMessageId,
        message_type: MycelinkChatMessageType,
    ) -> Self {
        Self {
            timestamp,
            id,
            message_type,
        }
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn id(&self) -> &MycelinkChatMessageId {
        &self.id
    }
    pub fn message_type(&self) -> &MycelinkChatMessageType {
        &self.message_type
    }
}

impl CompressionHinting for MycelinkChatMessage {
    fn compression_hint(&self) -> CompressionHint {
        self.message_type.compression_hint()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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

impl CompressionHinting for MycelinkChatMessageType {
    fn compression_hint(&self) -> CompressionHint {
        match self {
            MycelinkChatMessageType::Standard { content } => content.compression_hint(),
            MycelinkChatMessageType::Reply { content, .. } => content.compression_hint(),
            MycelinkChatMessageType::Reaction { .. } => CompressionHint::Fast,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MycelinkChatMessageContent {
    Text(Box<str>),
}

impl CompressionHinting for MycelinkChatMessageContent {
    fn compression_hint(&self) -> CompressionHint {
        match self {
            MycelinkChatMessageContent::Text(_) => CompressionHint::High,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MycelinkChatMessageId(pub [u8; 16]);

impl MycelinkChatMessageId {
    pub fn new() -> Self {
        let mut bytes = [0; 16];
        rand::thread_rng().fill_bytes(&mut bytes);
        Self(bytes)
    }
}

impl AsRef<[u8]> for MycelinkChatMessageId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
