use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::ListPeer;
use crate::model::message_type_identifier::MessageType;
use crate::model::unique_identifier::UniqueIdentifier;

const MESSAGE_TYPE: MessageType = MessageType::Client(ListPeer);

pub struct ListPeerMessage {
    pub node_identifier: UniqueIdentifier,
    pub with_metadata: bool,
    pub with_volatile: bool,
}

impl From<ListPeerMessage> for Message {
    fn from(value: ListPeerMessage) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![
                Field::new("NodeIdentifier".into(), (&value.node_identifier).into()),
                Field::new(
                    "WithMetadata".into(),
                    value.with_metadata.to_string().into(),
                ),
                Field::new(
                    "WithVolatile".into(),
                    value.with_volatile.to_string().into(),
                ),
            ]
            .into(),
            None,
        )
    }
}
