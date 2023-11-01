use crate::decode_error::DecodeError;
use crate::fcp_parser::FCPParser;
use crate::messages::FCPEncodable;
use crate::model::fcp_version::FCPVersion;
use crate::model::message_identifier::MessageIdentifier;
use async_trait::async_trait;
use tokio::io::{AsyncRead, BufReader};

pub struct NodeHello {
    version: FCPVersion,
}

#[async_trait]
impl FCPEncodable for NodeHello {
    fn encode(&self) -> String {
        todo!()
    }

    async fn decode(
        encoded: &mut FCPParser<BufReader<impl AsyncRead + Unpin + Send>>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        encoded
            .expect_identifier(MessageIdentifier::NodeHello)
            .await?;

        let fields = encoded.parse_fields().await?;

        todo!()
    }
}
