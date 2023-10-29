use crate::decode_error::DecodeError;
use crate::messages::FCPEncodable;
use crate::model::MessageIdentifier::MessageIdentifier;
use crate::peekable_reader::PeekableReader;
use async_trait::async_trait;
use tokio::io::{AsyncRead, AsyncReadExt};

pub const EXPECTED_VERSION: &str = "2.0";
const IDENTIFIER: MessageIdentifier = MessageIdentifier::ClientHello;

pub struct ClientHelloMessage {
    name: String,
    expected_version: String,
}

#[async_trait]
impl FCPEncodable for ClientHelloMessage {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(IDENTIFIER.name());
        builder.push_str("Name=");
        builder.push_str(self.name.as_str());
        builder.push_str("\nExpectedVersion=");
        builder.push_str(self.expected_version.as_str());
        builder.push_str("\nEndMessage\n");

        builder
    }

    async fn decode(
        encoded: &mut PeekableReader<impl AsyncRead + Unpin + Send>,
    ) -> Result<Self, DecodeError> {
        let mut identifier_buffer = [0; IDENTIFIER.len()];
        encoded.peek_exact(&mut identifier_buffer).await?;

        DecodeError::expect_message_identifier(&identifier_buffer, IDENTIFIER)?;

        todo!()
    }
}
