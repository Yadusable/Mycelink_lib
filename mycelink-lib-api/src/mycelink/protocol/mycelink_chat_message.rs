use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model;
use crate::model::message_types::{MessageContent, MessageType};
use crate::mycelink::protocol::compressed_box::{CompressionHint, CompressionHinting};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MycelinkChatMessage<'a> {
    timestamp: u64,
    id: MycelinkChatMessageId,
    message_type: MycelinkChatMessageType<'a>,
}

impl<'a> MycelinkChatMessage<'a> {
    pub fn new(
        timestamp: u64,
        id: MycelinkChatMessageId,
        message_type: MycelinkChatMessageType<'a>,
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

impl CompressionHinting for MycelinkChatMessage<'_> {
    fn compression_hint(&self) -> CompressionHint {
        self.message_type.compression_hint()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MycelinkChatMessageType<'a> {
    Standard {
        content: MycelinkChatMessageContent<'a>,
    },
    Reply {
        thread_start: MycelinkChatMessageId,
        content: MycelinkChatMessageContent<'a>,
    },
    Reaction {
        target_message: MycelinkChatMessageId,
        indicator: char,
    },
}

impl CompressionHinting for MycelinkChatMessageType<'_> {
    fn compression_hint(&self) -> CompressionHint {
        match self {
            MycelinkChatMessageType::Standard { content } => content.compression_hint(),
            MycelinkChatMessageType::Reply { content, .. } => content.compression_hint(),
            MycelinkChatMessageType::Reaction { .. } => CompressionHint::Fast,
        }
    }
}

impl MessageType {
    pub(crate) async fn as_mycelink(
        &self,
        db_connector: &DBConnector<Tenant>,
    ) -> MycelinkChatMessageType {
        match self {
            MessageType::Standard { content } => MycelinkChatMessageType::Standard {
                content: content.into(),
            },
            MessageType::Reply {
                thread_start,
                content,
            } => MycelinkChatMessageType::Reply {
                thread_start: db_connector
                    .get_message_meta(thread_start)
                    .await
                    .unwrap()
                    .unwrap()
                    .mycelink_id()
                    .unwrap()
                    .clone(),
                content: content.into(),
            },
            MessageType::Reaction {
                target_message,
                indicator,
            } => MycelinkChatMessageType::Reaction {
                target_message: db_connector
                    .get_message_meta(target_message)
                    .await
                    .unwrap()
                    .unwrap()
                    .mycelink_id()
                    .unwrap()
                    .clone(),
                indicator: *indicator,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum MycelinkChatMessageContent<'a> {
    Text(Cow<'a, str>),
}

impl CompressionHinting for MycelinkChatMessageContent<'_> {
    fn compression_hint(&self) -> CompressionHint {
        match self {
            MycelinkChatMessageContent::Text(_) => CompressionHint::High,
        }
    }
}

impl From<model::message_types::MessageContent> for MycelinkChatMessageContent<'_> {
    fn from(value: MessageContent) -> Self {
        match value {
            MessageContent::Text { content } => {
                MycelinkChatMessageContent::Text(Cow::Owned(content.into()))
            }
            MessageContent::Media { .. } => {
                todo!()
            }
        }
    }
}

impl<'a> From<&'a model::message_types::MessageContent> for MycelinkChatMessageContent<'a> {
    fn from(value: &'a MessageContent) -> Self {
        match value {
            MessageContent::Text { content } => {
                MycelinkChatMessageContent::Text(Cow::Borrowed(content.as_ref()))
            }
            MessageContent::Media { .. } => {
                todo!()
            }
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
