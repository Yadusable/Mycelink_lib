use std::error::Error;
use std::fmt::{write, Debug, Display, Formatter};

#[derive(Debug)]
pub enum DecodeError {
    TokioIoError(tokio::io::Error),
    ExpectedDifferentMessage {
        expected: &'static str,
        got: Box<str>,
    },
    ParseError(Box<str>),
    InvalidVersion(Box<str>),
}

impl DecodeError {
    pub fn expect_message_identifier(got: &[u8], expected: &'static str) -> Result<(), Self> {
        let got = String::from_utf8_lossy(got);

        if !got.starts_with(expected) {
            let got = got.split_once('\n').unwrap_or((&*got, "")).0;

            return Err(Self::ExpectedDifferentMessage {
                expected,
                got: got.into(),
            });
        }

        Ok(())
    }
}
impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodeError::TokioIoError(inner) => Display::fmt(inner, f),
            DecodeError::ExpectedDifferentMessage { expected, got } => {
                write!(
                    f,
                    "Expected '{expected}' but got '{got}' as FCP message type while decoding."
                )
            }
            DecodeError::ParseError(inner) => {
                write!(f, "{inner}")
            }
            DecodeError::InvalidVersion(inner) => {
                write!(f, "Version {inner} is unknown")
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
