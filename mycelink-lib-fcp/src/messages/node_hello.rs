use crate::decode_error::DecodeError;
use crate::fcp_parser::FCPParser;
use crate::messages::FCPDecodable;
use crate::model::connection_identifier::ConnectionIdentifier;
use crate::model::fcp_version::FCPVersion;
use crate::model::message_identifier::NodeMessageIdentifier;
use async_trait::async_trait;
use tokio::io::AsyncRead;

#[derive(Debug, Eq, PartialEq)]
pub struct NodeHelloMessage {
    pub fcp_version: FCPVersion,
    pub node: Box<str>,
    pub connection_identifier: ConnectionIdentifier,
}

#[async_trait]
impl FCPDecodable for NodeHelloMessage {
    async fn decode(
        encoded: &mut FCPParser<impl AsyncRead + Unpin + Send>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        encoded
            .expect_node_identifier(NodeMessageIdentifier::NodeHello)
            .await?;

        let fields = encoded.parse_fields().await?;

        Ok(NodeHelloMessage {
            fcp_version: fields.get("FCPVersion")?.value().try_into()?,
            node: fields.get("Node")?.value().into(),
            connection_identifier: fields.get("ConnectionIdentifier")?.value().into(),
        })
    }
}
