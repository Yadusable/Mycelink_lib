use hex::{FromHex, FromHexError};
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct ByteArray64(pub [u8; 64]);

impl Serialize for ByteArray64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.0)
    }
}

impl AsRef<[u8]> for ByteArray64 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl AsMut<[u8]> for ByteArray64 {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Deref for ByteArray64 {
    type Target = [u8; 64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for ByteArray64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'a> Visitor<'a> for BytesVisitor {
    type Value = ByteArray64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 64 bytes long array")
    }

    fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v.try_into() {
            Ok(v) => Ok(ByteArray64(v)),
            Err(err) => Err(E::invalid_length(v.len(), &"a 64 bytes long array")),
        }
    }
}

impl FromHex for ByteArray64 {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        match hex.as_ref().try_into() {
            Ok(hex) => Ok(ByteArray64(hex)),
            Err(_) => Err(FromHexError::InvalidStringLength),
        }
    }
}
