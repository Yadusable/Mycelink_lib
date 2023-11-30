use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::ListPeer;
use crate::model::message_type_identifier::{ClientMessageType, MessageType};

const MESSAGE_TYPE: MessageType = MessageType::Client(ListPeer);

pub struct ListPeerMessage {
    pub node_identifier: str,
    pub with_metadata: bool,
    pub with_volatile: bool,
}

impl From<ListPeerMessage> for Message {
    fn from(value: ListPeerMessage) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![
                Field::new("NodeIdentifier".into(), value.node_identifier.into()),
                Field::new("WithMetadata".into(), value.with_metadata.into()),
                Field::new("WithVolatile".into(), value.with_volatile.into()),
            ]
            .into(),
            None,
        )
    }
}
