use crate::crypto::hash_provider::blake3::BLAKE3;
use crate::crypto::hash_provider::sha512::SHA512;
use crate::crypto::key_material::KeyMaterial;
use serde::{Deserialize, Serialize};

pub trait KdfProvider: 'static {
    fn derive_key(&self, key_material: &KeyMaterial, purpose: &str) -> KeyMaterial;
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum KdfProviderTag {
    Sha512,
    Blake3,
}

impl KdfProviderTag {
    pub fn as_provider(&self) -> &'static dyn KdfProvider {
        (*self).into()
    }
}

impl From<KdfProviderTag> for &'static dyn KdfProvider {
    fn from(value: KdfProviderTag) -> Self {
        match value {
            KdfProviderTag::Sha512 => &SHA512,
            KdfProviderTag::Blake3 => &BLAKE3,
        }
    }
}

impl Default for KdfProviderTag {
    fn default() -> Self {
        KdfProviderTag::Blake3
    }
}
