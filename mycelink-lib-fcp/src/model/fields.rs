use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::ParseError;
use crate::peekable_reader_legacy::PeekableReaderLegacy;
use std::slice::Iter;
use std::str::from_utf8;
use tokio::io::{AsyncRead, BufReader};

const END_MESSAGE_LIT: &str = "EndMessage\n";
const DATA_LIT: &str = "Data\n";

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

    pub async fn decode(
        encoded: &mut PeekableReaderLegacy<BufReader<impl AsyncRead + Unpin + Send>>,
    ) -> Result<Self, DecodeError> {
        let mut buf = Vec::new();
        let mut fields: Vec<Field> = Vec::new();
        let mut read_len;
        let mut line;

        loop {
            read_len = encoded.peek_line(&mut buf).await?;
            line = from_utf8(buf.as_slice())?.trim_end();
            if Field::is_field(line) {
                fields.push(line.try_into()?);
                encoded.consume(read_len);
                buf.clear();
            } else {
                break;
            }
        }

        if line != END_MESSAGE_LIT && line != DATA_LIT {
            return Err(ParseError(
                format!("'{line}' neither indicates the end of a Fields nor is a field itself.")
                    .into(),
            ));
        }

        Ok(fields.into())
    }
}

pub struct Field {
    key: Box<str>,
    value: Box<str>,
}

impl Field {
    pub fn new(key: Box<str>, value: Box<str>) -> Self {
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
                format!("{encoded} cannot be parsed as field as it contains no '='").into(),
            )
        })?;

        Ok(Self {
            key: key.into(),
            value: value.into(),
        })
    }
}
