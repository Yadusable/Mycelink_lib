use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::TestDDARequest;
use crate::model::message_type_identifier::MessageType;

const MESSAGE_TYPE: MessageType = MessageType::Client(TestDDARequest);

pub struct TestDDARequestMessage {
    pub directory: str,
    pub want_read_directory: bool,
    pub want_write_directory: bool,
}

impl From<TestDDARequestMessage> for Message {
    fn from(value: TestDDARequest) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![Field::new("Directory".into(), value.directory.into()),
            Field::new("WandReadDirectory".into(), value.want_read_directory.into()),
            Field::new("WantWriteDirectory".into(), value.want_write_directory.into()),
            ].into(),
            None,
        )
    }
}
