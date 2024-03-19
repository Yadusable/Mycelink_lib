use crate::crypto::key_material::KeyMaterial;
use serde::{Deserialize, Serialize};

pub mod blake3;
pub mod sha512;

pub trait Hash: Clone + Serialize + for<'de> Deserialize<'de> + AsRef<[u8]> {}
impl<T: Clone + Serialize + for<'de> Deserialize<'de> + AsRef<[u8]>> Hash for T {}

pub trait HashProvider {
    type Hash: Hash;

    fn hash(data: &[u8]) -> Self::Hash;
    fn derive_key(key_material: &KeyMaterial, purpose: &'static str) -> KeyMaterial;
}
