use crate::decode_error::DecodeError;
use crate::fcp_parser::FCPParser;
use crate::messages::FCPEncodable;
use crate::model::fcp_version::FCPVersion;
use crate::model::message_identifier::MessageIdentifier;
use async_trait::async_trait;
use tokio::io::{AsyncRead, BufReader};

pub const EXPECTED_VERSION: &str = "2.0";
const IDENTIFIER: MessageIdentifier = MessageIdentifier::ClientHello;

pub struct ClientHelloMessage {
    name: String,
    version: FCPVersion,
}

#[async_trait]
impl FCPEncodable for ClientHelloMessage {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(IDENTIFIER.name());
        builder.push_str("Name=");
        builder.push_str(self.name.as_str());
        builder.push_str("\nExpectedVersion=");
        builder.push_str(self.version.name());
        builder.push_str("\nEndMessage\n");

        builder
    }

    async fn decode(
        encoded: &mut FCPParser<BufReader<impl AsyncRead + Unpin>>,
    ) -> Result<Self, DecodeError> {
        todo!()
    }
}
