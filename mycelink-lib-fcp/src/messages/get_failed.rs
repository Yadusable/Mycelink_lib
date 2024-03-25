use crate::decode_error::DecodeError;
use crate::model::content_type::ContentType;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType::GetFailed;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;

pub const DATA_NOT_FOUND_CODE: u32 = 13;

#[derive(Debug)]
pub struct GetFailedMessage {
    pub code: u32,
    pub identifier: UniqueIdentifier,
    pub code_description: Box<str>,
    pub short_code_description: Box<str>,
    pub extra_description: Option<Box<str>>,
    pub fatal: bool,
    pub redirect_uri: Option<URI>,
    pub expected_data_length: Option<usize>,
    pub expected_metadata_content_type: Option<ContentType>,
    pub finalized_expected: Option<bool>,
}

impl TryFrom<Message> for GetFailedMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(GetFailed)?;

        Ok(Self {
            code: value.fields().get_or_err("Code")?.value().parse()?,
            identifier: value.fields().get_or_err("Identifier")?.value().parse()?,
            code_description: value.fields().get_or_err("CodeDescription")?.value().into(),
            short_code_description: value
                .fields()
                .get_or_err("ShortCodeDescription")?
                .value()
                .into(),
            extra_description: value
                .fields()
                .get("ExtraDescription")
                .map(|e| e.value().into()),
            fatal: value.fields().get_or_err("Fatal")?.value().parse()?,
            redirect_uri: match value.fields().get("RedirectURI") {
                None => None,
                Some(field) => Some(field.value().parse()?),
            },
            expected_data_length: match value.fields().get("ExpectedDataLength") {
                None => None,
                Some(field) => Some(field.value().parse()?),
            },
            expected_metadata_content_type: match value.fields().get("ExpectedMetadata.ContentType")
            {
                None => None,
                Some(field) => Some(field.value().parse()?),
            },
            finalized_expected: value
                .fields()
                .get("FinalizedExpected")
                .and_then(|e| e.value().parse().ok()),
        })
    }
}
