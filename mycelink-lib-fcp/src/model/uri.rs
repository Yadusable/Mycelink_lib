use crate::decode_error::DecodeError;

#[derive(Eq, PartialEq, Debug)]
pub struct URI {
    uri: Box<str>,
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
