use crate::crypto::hash_provider::HashProvider;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::key_material::KeyMaterial::KM256;

pub struct Blake3 {}
impl HashProvider for Blake3 {
    type Hash = [u8; 32];

    fn hash(data: &[u8]) -> Self::Hash {
        blake3::hash(data).into()
    }

    fn derive_key(key_material: &KeyMaterial, purpose: &'static str) -> KeyMaterial {
        KM256(blake3::derive_key(purpose, key_material.as_ref()))
    }
}
