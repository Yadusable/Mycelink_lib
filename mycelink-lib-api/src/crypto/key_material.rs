use crate::crypto::key_material::KeyMaterial::{KM256, KM512};
use crate::crypto::types::byte_array_64::ByteArray64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum KeyMaterial {
    KM256([u8; 32]),
    KM512(ByteArray64),
}

impl AsRef<[u8]> for KeyMaterial {
    fn as_ref(&self) -> &[u8] {
        match self {
            KM256(data) => data,
            KM512(data) => data.as_ref(),
        }
    }
}

impl From<[u8; 32]> for KeyMaterial {
    fn from(value: [u8; 32]) -> Self {
        KM256(value)
    }
}

impl From<[u8; 64]> for KeyMaterial {
    fn from(value: [u8; 64]) -> Self {
        KM512(ByteArray64(value))
    }
}

impl From<KeyMaterial> for [u8; 32] {
    fn from(value: KeyMaterial) -> Self {
        match value {
            KM256(inner) => inner,
            KM512(inner) => inner.as_slice().split_at(32).0.try_into().unwrap(),
        }
    }
}
