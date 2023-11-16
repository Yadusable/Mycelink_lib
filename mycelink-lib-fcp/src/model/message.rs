use crate::decode_error::DecodeError;
use crate::messages::{FCPEncodable, MessagePayload};
use crate::model::fields::Fields;
use crate::model::message_type_identifier::MessageType;
use crate::peekable_reader_legacy::PeekableReaderLegacy;
use tokio::io::{AsyncRead, BufReader};
use crate::peekable_reader::PeekableReader;

pub struct Message {
    message_identifier: &'static MessageType,
    fields: Fields,
    payload: Option<MessagePayload>,
}

impl FCPEncodable for Message {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(self.message_identifier.name());
        builder.push('\n');

        for field in self.fields.iter() {
            builder.push_str(field.key());
            builder.push('=');
            builder.push_str(field.value());
            builder.push('\n');
        }

        match &self.payload {
            None => builder.push_str("EndMessage\n"),
            Some(_payload) => {
                todo!()
            }
        }

        builder
    }
}

impl Message {
    async fn decode(
        encoded: &mut PeekableReader<impl AsyncRead + Unpin>,
    ) -> Result<Self, DecodeError> {
        let message_type = MessageType::decode(encoded).await?;
        let fields = Fields::decode(encoded).await?;

        todo!()
    }
}
