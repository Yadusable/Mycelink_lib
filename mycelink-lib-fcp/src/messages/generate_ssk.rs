use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::GenerateSSK;
use crate::model::message_type_identifier::MessageType;

const MESSAGE_TYPE: MessageType = MessageType::Client(GenerateSSK);

pub struct GenerateSSKMessage {
    pub identifier: str,
}

impl From<GenerateSSKMessage> for Message {
    fn from(value: GenerateSSK) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![Field::new("Identifier".into(), value.identifier.into())].into(),
            None,
        )
    }
}
