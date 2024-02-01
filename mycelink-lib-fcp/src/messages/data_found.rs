use crate::decode_error::DecodeError;
use crate::model::content_type::ContentType;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;
use crate::model::unique_identifier::UniqueIdentifier;

pub struct DataFoundMessage {
    pub identifier: UniqueIdentifier,
    pub content_type: ContentType,
    pub data_length: usize,
}

impl TryFrom<Message> for DataFoundMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(NodeMessageType::DataFound)?;

        Ok(Self {
            identifier: value
                .fields()
                .get_or_err("Identifier")?
                .value()
                .try_into()?,
            content_type: value
                .fields()
                .get_or_err("Metadata.ContentType")?
                .value()
                .parse()?,
            data_length: value.fields().get_or_err("DataLength")?.value().parse()?,
        })
    }
}
