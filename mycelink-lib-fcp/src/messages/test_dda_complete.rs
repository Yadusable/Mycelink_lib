use std::path::{Path, PathBuf};
use crate::decode_error::DecodeError;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;

pub struct TestDDACompleteMessage {
    pub directory: Box<Path>,
    pub read_filename: Box<str>,
    pub write_filename: Box<str>,
}

impl TryFrom<Message> for TestDDACompleteMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(NodeMessageType::TestDDAComplete)?;

        Ok(Self {
            directory: PathBuf::from(value.fields().get("Directory")?.value()).into(),
            read_filename: value.fields().get("ReadFilename")?.value().into(),
            write_filename: value.fields().get("WriteFilename")?.value().into(),
        })
    }
}
