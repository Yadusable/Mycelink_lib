use crate::crypto::key_material::KeyMaterial::KeyMaterial256;

pub enum KeyMaterial {
    KeyMaterial256([u8; 32]),
}

impl From<[u8; 32]> for KeyMaterial {
    fn from(value: [u8; 32]) -> Self {
        KeyMaterial256(value)
    }
}

impl From<KeyMaterial> for [u8; 32] {
    fn from(value: KeyMaterial) -> Self {
        match value {
            KeyMaterial256(inner) => inner,
        }
    }
}
