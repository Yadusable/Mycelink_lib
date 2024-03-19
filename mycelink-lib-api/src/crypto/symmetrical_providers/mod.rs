pub mod xchacha20poly1305;

use crate::crypto::key_material::KeyMaterial;
use hex::FromHexError;
use serde::{Deserialize, Serialize};

pub trait SymmetricEncryptionProvider {
    type Key: Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + hex::FromHex<Error = FromHexError>
        + hex::ToHex
        + AsRef<[u8]>;

    type Encrypted: Serialize + for<'de> Deserialize<'de>;

    fn generate_random_key() -> Self::Key;
    fn generate_key_from_material(material: KeyMaterial) -> Self::Key;

    fn encrypt(data: Box<[u8]>, key: &Self::Key) -> Self::Encrypted;
    fn decrypt(data: Self::Encrypted, key: &Self::Key) -> Result<Box<[u8]>, ()>;
}
