use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::{ParseError, UnexpectedEOF};
use crate::peekable_reader::Peeker;
use std::borrow::Cow;
use std::ops::Deref;
use std::slice::Iter;
use tokio::io::AsyncRead;

pub const END_MESSAGE_LIT: &str = "EndMessage";
pub const DATA_LIT: &str = "Data";

const PAYLOAD_LENGTH_HINT_KEYS: &[&str] = &["DataLength"];

pub struct Fields {
    fields: Vec<Field>,
}

impl Fields {
    pub fn iter(&self) -> Iter<Field> {
        self.fields.iter()
    }

    pub fn get(&self, key: &str) -> Result<&Field, DecodeError> {
        self.fields
            .iter()
            .find(|e| &*e.key == key)
            .ok_or(DecodeError::MissingField(key.into()))
    }

    pub fn get_payload_size_hint(&self) -> Result<&Field, DecodeError> {
        let mut iter = self
            .iter()
            .filter(|e| PAYLOAD_LENGTH_HINT_KEYS.contains(&e.key()));
        let first = iter.next();

        match first {
            None => Err(DecodeError::MissingField("PAYLOAD LENGTH HINT".into())),
            Some(hint) => {
                if iter.next().is_some() {
                    todo!("How to handle if multiple hints match?")
                } else {
                    Ok(hint)
                }
            }
        }
    }

    pub async fn decode(
        peeker: &mut Peeker<'_, impl AsyncRead + Unpin>,
    ) -> Result<Self, DecodeError> {
        let mut fields: Vec<Field> = Vec::new();

        let mut line = peeker.next_contentful_line().await?.ok_or(UnexpectedEOF)?;

        while Field::is_field(line.deref()) {
            fields.push(line.as_ref().try_into()?);
            line = peeker.next_contentful_line().await?.ok_or(UnexpectedEOF)?;
        }

        if line.as_ref() != END_MESSAGE_LIT && line.as_ref() != DATA_LIT {
            return Err(ParseError(
                format!("'{line}' neither indicates the end of a Fields nor is a field itself.")
                    .into(),
            ));
        }

        Ok(fields.into())
    }
}

pub struct Field {
    key: Cow<'static, str>,
    value: Box<str>,
}

impl Field {
    pub fn new(key: Cow<'static, str>, value: Box<str>) -> Self {
        Self { key, value }
    }
    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn is_field(line: &str) -> bool {
        line.contains('=')
    }
}

impl From<Vec<Field>> for Fields {
    fn from(value: Vec<Field>) -> Self {
        Fields { fields: value }
    }
}

impl TryFrom<&str> for Field {
    type Error = DecodeError;

    fn try_from(encoded: &str) -> Result<Self, Self::Error> {
        let (key, value) = encoded.split_once('=').ok_or_else(|| {
            DecodeError::ParseError(
                format!("'{encoded}' cannot be parsed as field as it contains no '='").into(),
            )
        })?;

        Ok(Self {
            key: key.to_string().into(),
            value: value.into(),
        })
    }
}
