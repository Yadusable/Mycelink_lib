use crate::decode_error::DecodeError;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;
use crate::model::unique_identifier::UniqueIdentifier;

pub struct SSKKeypairMessage {
    pub identifier: UniqueIdentifier,
    pub request_uri: Box<str>, // todo maybe replace with uri type, has to contain "freenet:"
    pub insert_uri: Box<str>,
}

impl TryFrom<Message> for SSKKeypairMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(NodeMessageType::SSKKeypair)?;

        Ok(Self {
            identifier: value.fields().get("Identifier")?.value().try_into()?,
            request_uri: value.fields().get("RequestURI")?.value().into(),
            insert_uri: value.fields().get("InsertURI")?.value().into(),
        })
    }
}
