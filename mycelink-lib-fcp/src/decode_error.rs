use crate::model::message_identifier::{MessageIdentifier, NodeMessageIdentifier};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum DecodeError {
    TokioIoError(tokio::io::Error),
    ProtocolBreak(Box<str>),
    ExpectedDifferentMessage {
        expected: NodeMessageIdentifier,
        got: NodeMessageIdentifier,
    },
    UnknownMessageIdentifier {
        got: Box<str>,
    },
    ParseError(Box<str>),
    InvalidVersion(Box<str>),
    MissingField(Box<str>),
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::TokioIoError(inner) => Display::fmt(inner, f),
            DecodeError::ExpectedDifferentMessage { expected, got } => {
                write!(
                    f,
                    "Expected '{}' but got '{}' as FCP message type while decoding.",
                    expected.name(),
                    got.name()
                )
            }
            DecodeError::UnknownMessageIdentifier { got } => {
                write!(f, "Could not parse '{got}' as MessageIdentifier")
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
        }
    }
}

impl Error for DecodeError {}

impl From<tokio::io::Error> for DecodeError {
    fn from(value: std::io::Error) -> Self {
        DecodeError::TokioIoError(value)
    }
}
