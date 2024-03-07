use crate::crypto::key_material::KeyMaterial::{KeyMaterial256, KeyMaterial512};

pub enum KeyMaterial {
    KeyMaterial256([u8; 32]),
    KeyMaterial512([u8; 64]),
}

impl AsRef<[u8]> for KeyMaterial {
    fn as_ref(&self) -> &[u8] {
        match self {
            KeyMaterial256(data) => data,
            KeyMaterial512(data) => data,
        }
    }
}

impl From<[u8; 32]> for KeyMaterial {
    fn from(value: [u8; 32]) -> Self {
        KeyMaterial256(value)
    }
}

impl From<[u8; 64]> for KeyMaterial {
    fn from(value: [u8; 64]) -> Self {
        KeyMaterial512(value)
    }
}

impl From<KeyMaterial> for [u8; 32] {
    fn from(value: KeyMaterial) -> Self {
        match value {
            KeyMaterial256(inner) => inner,
            KeyMaterial512(inner) => inner.as_slice().split_at(32).0.try_into().unwrap(),
        }
    }
}
