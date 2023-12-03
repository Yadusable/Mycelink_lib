use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::TestDDAResponse;
use crate::model::message_type_identifier::MessageType;
use std::path::Path;

const MESSAGE_TYPE: MessageType = MessageType::Client(TestDDAResponse);

pub struct TestDDAResponseMessage {
    pub directory: Box<Path>,
    pub read_content: Box<str>,
}

impl From<TestDDAResponseMessage> for Message {
    fn from(value: TestDDAResponseMessage) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![
                Field::new("Directory".into(), value.directory.to_string_lossy().into()),
                Field::new("ReadContent".into(), value.read_content),
            ]
            .into(),
            None,
        )
    }
}
