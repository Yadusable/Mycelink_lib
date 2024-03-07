use crate::decode_error::DecodeError;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct URI {
    uri: Box<str>,
}

impl FromStr for URI {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for URI {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self { uri: value.into() })
    }
}

impl From<&URI> for Box<str> {
    fn from(value: &URI) -> Self {
        value.uri.clone()
    }
}
