use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::GenerateSSK;
use crate::model::message_type_identifier::MessageType;
use crate::model::unique_identifier::UniqueIdentifier;

const MESSAGE_TYPE: MessageType = MessageType::Client(GenerateSSK);

pub struct GenerateSSKMessage {
    pub identifier: UniqueIdentifier,
}

impl From<GenerateSSKMessage> for Message {
    fn from(value: GenerateSSKMessage) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![Field::new("Identifier".into(), (&value.identifier).into())].into(),
            None,
        )
    }
}
