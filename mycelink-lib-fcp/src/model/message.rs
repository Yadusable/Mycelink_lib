use crate::decode_error::DecodeError;
use crate::messages::{FCPEncodable, MessagePayload};
use crate::model::fields::{Fields, END_MESSAGE_LIT};
use crate::model::message_type_identifier::MessageType;
use crate::peekable_reader::{PeekableReader, Peeker};
use std::ops::Deref;
use tokio::io::AsyncRead;

pub struct Message {
    message_type: MessageType,
    fields: Fields,
    payload: Option<MessagePayload>,
}

impl FCPEncodable for Message {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(self.message_type.name());
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
    pub fn new(message_type: MessageType, fields: Fields, payload: Option<MessagePayload>) -> Self {
        Self {
            message_type,
            fields,
            payload,
        }
    }

    pub fn message_type(&self) -> MessageType {
        self.message_type
    }

    pub fn fields(&self) -> &Fields {
        &self.fields
    }

    pub fn payload(&self) -> &Option<MessagePayload> {
        &self.payload
    }

    pub async fn decode(
        encoded: &mut PeekableReader<impl AsyncRead + Unpin>,
    ) -> Result<Self, DecodeError> {
        let mut peeker = Peeker::new(encoded);
        let message_type = MessageType::decode(&mut peeker).await?;
        let fields = Fields::decode(&mut peeker).await?;

        if peeker
            .current_line()
            .await?
            .map(|e| e.deref() == END_MESSAGE_LIT)
            .unwrap_or(false)
        {
            let stats = peeker.into();
            encoded.advance_to_peeker_stats(stats);

            return Ok(Self {
                message_type,
                fields,
                payload: None,
            });
        }

        todo!("Cannot recover from failed message parse yet")
    }
}
