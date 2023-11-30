use crate::decode_error::DecodeError;
use crate::messages::all_data::AllDataMessage;
use crate::messages::client_get::ClientGetMessage;
use crate::messages::client_hello::ClientHelloMessage;
use crate::messages::client_put::ClientPutMessage;
use crate::messages::node_hello::NodeHelloMessage;
use crate::model::fields::{Fields, DATA_LIT, END_MESSAGE_LIT};
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
    pub fn encode(&self) -> Vec<u8> {
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
            None => {
                builder.push_str(END_MESSAGE_LIT);
                builder.push('\n');
                builder.into_bytes()
            }
            Some(payload) => {
                builder.push_str(payload.data_len_identifier.deref());
                builder.push('=');
                builder.push_str(payload.data.len().to_string().as_str());
                builder.push('\n');

                builder.push_str(DATA_LIT);

                let mut buf = builder.into_bytes();

                buf.extend_from_slice(payload.data.as_slice());

                buf
            }
        }
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

    pub fn payload(self) -> Option<MessagePayload> {
        self.payload
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
        } else if peeker
            .current_line()
            .await?
            .map(|e| e.deref() == DATA_LIT)
            .unwrap_or(false)
        {
            let stats = peeker.into();
            encoded.advance_to_peeker_stats(stats);

            let size_hint = fields.get_payload_size_hint()?;
            let size_hint_key = size_hint.key().into();

            let mut payload = vec![0; size_hint.value().parse()?];
            encoded.read_exact(payload.as_mut_slice()).await?;

            return Ok(Self {
                message_type,
                fields,
                payload: Some(MessagePayload {
                    data: payload,
                    data_len_identifier: size_hint_key,
                }),
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
    AllData(AllDataMessage),
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
