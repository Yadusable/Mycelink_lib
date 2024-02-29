use crate::decode_error::DecodeError;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType::PutFailed;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;

#[derive(Debug)]
pub struct PutFailedMessage {
    pub code: u32,
    pub identifier: UniqueIdentifier,
    pub expected_uri: URI,
    pub code_description: Box<str>,
    pub short_code_description: Box<str>,
    pub extra_description: Box<str>,
    pub fatal: bool,
}

impl TryFrom<Message> for PutFailedMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(PutFailed)?;

        Ok(Self {
            code: value.fields().get_or_err("Code")?.value().parse()?,
            identifier: value.fields().get_or_err("Identifier")?.value().parse()?,
            expected_uri: value.fields().get_or_err("ExpectedURI")?.value().parse()?,
            code_description: value.fields().get_or_err("CodeDescription")?.value().into(),
            short_code_description: value
                .fields()
                .get_or_err("ShortCodeDescription")?
                .value()
                .into(),
            extra_description: value
                .fields()
                .get_or_err("ExtraDescription")?
                .value()
                .into(),
            fatal: value.fields().get_or_err("Fatal")?.value().parse()?,
        })
    }
}
