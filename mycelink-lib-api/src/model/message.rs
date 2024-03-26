use crate::model::contact::Contact;

pub struct Message {
    sender_contact_id: Box<dyn Contact>,
    message_id: MessageId,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reaction] which reference this Message.
    reactions: Vec<MessageId>,
    /// All [MessageId] that belong to a Message of type [crate::model::message_types::MessageType::Reply] which reference this Message as thread start.
    replies: Vec<MessageId>,
    timestamp: u64,
}

pub struct MessageId {
    id: Box<[u8]>,
}
