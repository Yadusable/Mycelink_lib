use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::ParseError;
use base64::Engine;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const PREFIX: &str = "[Mycelink] ";
const NAME_NONCE_SEPARATOR: &str = " - ";

const NONCE_LENGTH: usize = 32;
const ENCODED_NONCE_LENGTH: usize = NONCE_LENGTH.div_ceil(3) * 4;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UniqueIdentifier {
    name: Box<str>,
    nonce: Box<str>,
}

impl UniqueIdentifier {
    pub fn new(name: impl Into<Box<str>>) -> Self {
        Self {
            name: name.into(),
            nonce: base64::engine::general_purpose::STANDARD
                .encode(rand::random::<[u8; NONCE_LENGTH]>())
                .into(),
        }
    }
}

impl From<&UniqueIdentifier> for Box<str> {
    fn from(value: &UniqueIdentifier) -> Self {
        let mut builder = String::new();
        builder.push_str(PREFIX);
        builder.push_str(&value.name);
        builder.push_str(NAME_NONCE_SEPARATOR);
        builder.push_str(&value.nonce);

        builder.into_boxed_str()
    }
}

impl FromStr for UniqueIdentifier {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for UniqueIdentifier {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value.starts_with(PREFIX) {
            return Err(ParseError(
                format!("{value} is not a Mycelink identifier").into(),
            ));
        }

        let identifier = value.split_at(PREFIX.len()).1;

        let (name, nonce) = identifier.split_once(NAME_NONCE_SEPARATOR)
            .ok_or(ParseError(format!("{value} doesn't contain the separator for nonce and name ('{NAME_NONCE_SEPARATOR}')").into()))?;

        if nonce.len() != ENCODED_NONCE_LENGTH {
            return Err(ParseError(
                format!("Expected encoded nonce to be {ENCODED_NONCE_LENGTH} bytes long, but is instead {} bytes long.
                Read '{nonce}' as nonce from identifier '{value}'", nonce.len()).into()
            ));
        }

        Ok(Self {
            name: name.into(),
            nonce: nonce.into(),
        })
    }
}

impl Display for UniqueIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Box::<str>::from(self))
    }
}
