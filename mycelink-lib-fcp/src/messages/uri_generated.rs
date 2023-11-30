use crate::decode_error::DecodeError;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;

pub struct UriGeneratedMessage {
    pub identifier: UniqueIdentifier,
    pub URI: URI,
}

impl TryFrom<Message> for UriGeneratedMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value.message_type().expect_specific_node_message(NodeMessageType::URIGenerated)?;
        
        Ok(Self {
            identifier: value.fields().get("Identifier")?.value().try_into()?,
            URI: value.fields().get("URI")?.value().try_into()?,
        })
    }
}
