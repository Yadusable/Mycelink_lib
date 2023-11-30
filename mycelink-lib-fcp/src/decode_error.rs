use crate::model::message_type_identifier::MessageType;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum DecodeError {
    TokioIoError(tokio::io::Error),
    ProtocolBreak(Box<str>),
    ExpectedDifferentMessageType {
        expected: MessageType,
        got: MessageType,
    },
    UnknownMessageType {
        got: Box<str>,
    },
    ParseError(Box<str>),
    Utf8Error(Utf8Error),
    InvalidVersion(Box<str>),
    MissingField(Box<str>),
    MissingPayload,
    UnexpectedEOF,
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::TokioIoError(inner) => Display::fmt(inner, f),
            DecodeError::ExpectedDifferentMessageType { expected, got } => {
                write!(
                    f,
                    "Expected '{}' but got '{}' as FCP message type while decoding.",
                    expected.name(),
                    got.name()
                )
            }
            DecodeError::UnknownMessageType { got } => {
                write!(f, "Could not parse '{got}' as MessageType")
            }
            DecodeError::ParseError(inner) => {
                write!(f, "Parse error: {inner}")
            }
            DecodeError::InvalidVersion(inner) => {
                write!(f, "Version {inner} is unknown.")
            }
            DecodeError::MissingField(inner) => {
                write!(f, "Field {inner} is missing in message.")
            }
            DecodeError::ProtocolBreak(inner) => {
                write!(f, "Protocol Break: {inner}")
            }
            DecodeError::Utf8Error(inner) => {
                write!(f, "Utf8Error: {inner}")
            }
            DecodeError::UnexpectedEOF => {
                write!(f, "Unexpected EOF")
            }
            DecodeError::MissingPayload => {
                write!(f, "Message is missing Payload")
            }
        }
    }
}

impl Error for DecodeError {}

impl From<tokio::io::Error> for DecodeError {
    fn from(value: std::io::Error) -> Self {
        DecodeError::TokioIoError(value)
    }
}

impl From<Utf8Error> for DecodeError {
    fn from(value: Utf8Error) -> Self {
        DecodeError::Utf8Error(value)
    }
}

impl From<ParseIntError> for DecodeError {
    fn from(value: ParseIntError) -> Self {
        DecodeError::ParseError(
            format!("Failed to parse integer (kind: {:?})", value.kind()).into(),
        )
    }
}
