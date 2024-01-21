use crate::model::contact::ContactIdentifier;

pub struct Message {
    sender: ContactIdentifier,
    message_id: MessageId,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reaction] which reference this Message.
    reactions: Box<[MessageId]>,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reply] which reference this Message as thread start.
    replies: Box<[MessageId]>,
    timestamp: u64,
}

pub struct MessageId {
    id: Box<str>,
}
