use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::TestDDARequest;
use crate::model::message_type_identifier::MessageType;
use std::path::Path;

const MESSAGE_TYPE: MessageType = MessageType::Client(TestDDARequest);

pub struct TestDDARequestMessage {
    pub directory: Box<Path>,
    pub want_read_directory: bool,
    pub want_write_directory: bool,
}

impl From<TestDDARequestMessage> for Message {
    fn from(value: TestDDARequestMessage) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![
                Field::new("Directory".into(), value.directory.to_string_lossy().into()),
                Field::new("WandReadDirectory".into(), value.want_read_directory.to_string().into()),
                Field::new(
                    "WantWriteDirectory".into(),
                    value.want_write_directory.to_string().into(),
                ),
            ]
            .into(),
            None,
        )
    }
}
