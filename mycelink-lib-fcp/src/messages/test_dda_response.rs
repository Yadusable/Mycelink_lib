use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::TestDDAResponse;
use crate::model::message_type_identifier::MessageType;

const MESSAGE_TYPE: MessageType = MessageType::Client(TestDDAResponse);

pub struct TestDDAResponseMessage {
    pub directory: str,
    pub read_content: str,
}

impl From<TestDDAResponseMessage> for Message {
    fn from(value: TestDDAResponse) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![
                Field::new("Directory".into(), value.directory.into()),
                Field::new("ReadContent".into(), value.read_content.into()),
            ]
            .into(),
            None,
        )
    }
}
