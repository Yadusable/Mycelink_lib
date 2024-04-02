use crate::db::actions::message_actions::MessageId;
use crate::model::contact::ContactDisplay;
use crate::model::message_types::MessageType;

pub struct Message {
    pub sender: ContactDisplay,
    pub message_id: MessageId,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reaction] which reference this Message.
    pub reactions: Vec<MessageId>,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reply] which reference this Message as thread start.
    pub replies: Vec<MessageId>,
    pub timestamp: u64,
    pub content: MessageType,
}
