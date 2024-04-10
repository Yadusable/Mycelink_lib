use crate::db::actions::message_actions::MessageId;
use crate::model::contact::ContactDisplay;
use crate::model::message_types::MessageType;
use crate::mycelink::protocol::mycelink_chat_message::{
    MycelinkChatMessage, MycelinkChatMessageId,
};
use serde::{Deserialize, Serialize};

pub struct Message {
    pub sender: ContactDisplay,
    pub message_id: MessageId,
    pub protocol_message_meta: ProtocolMessageMeta,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reaction] which reference this Message.
    pub reactions: Vec<MessageId>,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reply] which reference this Message as thread start.
    pub replies: Vec<MessageId>,
    pub timestamp: u64,
    pub content: MessageType,
}

#[derive(Serialize, Deserialize)]
pub enum ProtocolMessageMeta {
    Mycelink { id: MycelinkChatMessageId },
}

impl ProtocolMessageMeta {
    pub fn mycelink_id(&self) -> Result<&MycelinkChatMessageId, ()> {
        if let Self::Mycelink { id } = self {
            Ok(id)
        } else {
            Err(())
        }
    }
}

impl From<&MycelinkChatMessage<'_>> for ProtocolMessageMeta {
    fn from(value: &MycelinkChatMessage) -> Self {
        ProtocolMessageMeta::Mycelink {
            id: value.id().clone(),
        }
    }
}
