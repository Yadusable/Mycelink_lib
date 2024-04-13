use crate::crypto::hash_provider::HashProvider;
use crate::crypto::kdf_provider::KdfProvider;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::key_material::KeyMaterial::KM256;

pub const BLAKE3: Blake3 = Blake3 {};

pub struct Blake3 {}
impl HashProvider for Blake3 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        blake3::hash(data).into()
    }
}

impl KdfProvider for Blake3 {
    fn derive_key(&self, key_material: &KeyMaterial, purpose: &str) -> KeyMaterial {
        KM256(blake3::derive_key(purpose, key_material.as_ref()))
    }
}
