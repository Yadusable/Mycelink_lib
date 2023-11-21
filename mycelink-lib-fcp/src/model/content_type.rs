use crate::decode_error::DecodeError;
use mime::Mime;
use std::str::FromStr;

pub struct ContentType {
    inner: Mime,
}

impl From<&ContentType> for Box<str> {
    fn from(value: &ContentType) -> Self {
        value.inner.to_string().into()
    }
}

impl FromStr for ContentType {
    type Err = DecodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            inner: Mime::from_str(s).map_err(|err| {
                DecodeError::ParseError(format!("Failed to parse {s} to MIME ({err})").into())
            })?,
        })
    }
}
