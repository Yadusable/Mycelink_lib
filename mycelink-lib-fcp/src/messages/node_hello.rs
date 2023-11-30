use crate::decode_error::DecodeError;
use crate::model::connection_identifier::ConnectionIdentifier;
use crate::model::fcp_version::FCPVersion;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;

#[derive(Debug, Eq, PartialEq)]
pub struct NodeHelloMessage {
    pub fcp_version: FCPVersion,
    pub node: Box<str>,
    pub connection_identifier: ConnectionIdentifier,
}

impl TryFrom<Message> for NodeHelloMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(NodeMessageType::NodeHello)?;

        Ok(Self {
            fcp_version: value.fields().get("FCPVersion")?.try_into()?,
            node: value.fields().get("Node")?.value().into(),
            connection_identifier: value.fields().get("ConnectionIdentifier")?.value().into(),
        })
    }
}
