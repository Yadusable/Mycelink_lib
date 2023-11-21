use crate::decode_error::DecodeError;
use crate::fcp_parser_legacy::FCPParserLegacy;
use crate::messages::FCPDecodable;
use crate::model::connection_identifier::ConnectionIdentifier;
use crate::model::fcp_version::FCPVersion;
use crate::model::message::Message;
use crate::model::message_type_identifier::MessageType::Node;
use crate::model::message_type_identifier::NodeMessageType;
use async_trait::async_trait;
use tokio::io::AsyncRead;

#[derive(Debug, Eq, PartialEq)]
pub struct NodeHelloMessage {
    pub fcp_version: FCPVersion,
    pub node: Box<str>,
    pub connection_identifier: ConnectionIdentifier,
}

impl TryFrom<Message> for NodeHelloMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        if value
            .message_type()
            .is_specific_node_message(&NodeMessageType::NodeHello)
        {
            Ok(Self {
                fcp_version: value.fields().get("FCPVersion")?.try_into()?,
                node: value.fields().get("Node")?.value().into(),
                connection_identifier: value.fields().get("ConnectionIdentifier")?.value().into(),
            })
        } else {
            Err(DecodeError::ExpectedDifferentMessageType {
                expected: Node(NodeMessageType::NodeHello),
                got: value.message_type(),
            })
        }
    }
}

#[async_trait]
impl FCPDecodable for NodeHelloMessage {
    async fn decode(
        encoded: &mut FCPParserLegacy<impl AsyncRead + Unpin + Send>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        encoded
            .expect_node_identifier(NodeMessageType::NodeHello)
            .await?;

        let fields = encoded.parse_fields().await?;

        Ok(NodeHelloMessage {
            fcp_version: fields.get("FCPVersion")?.value().try_into()?,
            node: fields.get("Node")?.value().into(),
            connection_identifier: fields.get("ConnectionIdentifier")?.value().into(),
        })
    }
}
