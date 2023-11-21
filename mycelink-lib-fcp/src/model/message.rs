use crate::decode_error::DecodeError;
use crate::messages::client_get::ClientGetMessage;
use crate::messages::client_hello::ClientHelloMessage;
use crate::messages::client_put::ClientPutMessage;
use crate::messages::node_hello::NodeHelloMessage;
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

impl Message {
    pub fn encode(&self) -> String {
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

pub enum ClientMessage {
    ClientHello(ClientHelloMessage),
    ClientGet(ClientGetMessage),
    ClientPut(ClientPutMessage),
}

impl From<ClientMessage> for Message {
    fn from(value: ClientMessage) -> Self {
        match value {
            ClientMessage::ClientHello(inner) => inner.into(),
            ClientMessage::ClientGet(inner) => (&inner).into(),
            ClientMessage::ClientPut(inner) => (&inner).into(),
        }
    }
}

pub enum NodeMessage {
    NodeHello(NodeHelloMessage),
}

pub struct MessagePayload {
    pub data: Vec<u8>,
    pub data_len_identifier: Box<str>,
}

pub trait FCPEncodable {
    fn to_message(self) -> Message;
}

impl<T: Into<Message>> FCPEncodable for T {
    fn to_message(self) -> Message {
        self.into()
    }
}
