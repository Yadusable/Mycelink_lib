use crate::crypto::key_material::KeyMaterial;
use serde::{Deserialize, Serialize};

pub mod blake3;
pub mod sha512;

pub trait HashProvider {
    type Hash: Clone + Serialize + for<'de> Deserialize<'de> + AsRef<[u8]>;

    fn hash(data: &[u8]) -> Self::Hash;
    fn derive_key(key_material: &KeyMaterial, purpose: &'static str) -> KeyMaterial;
}
