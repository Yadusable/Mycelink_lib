use crate::decode_error::DecodeError;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;

pub struct UriGeneratedMessage {
    pub identifier: UniqueIdentifier,
    pub uri: URI,
}

impl TryFrom<Message> for UriGeneratedMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(NodeMessageType::URIGenerated)?;

        Ok(Self {
            identifier: value
                .fields()
                .get_or_err("Identifier")?
                .value()
                .try_into()?,
            uri: value.fields().get_or_err("URI")?.value().try_into()?,
        })
    }
}
