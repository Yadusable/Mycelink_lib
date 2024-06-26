use crate::crypto::key_exchange::CompletedKeyExchange;
use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::keypairs::EncryptionKeyPair;
use hex::FromHexError;
use serde::{Deserialize, Serialize};

pub mod x25519;

pub type DefaultAsymmetricEncryptionProvider = X25519;

pub trait AsymmetricEncryptionProvider {
    type PublicKey: Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + hex::FromHex<Error = FromHexError>
        + hex::ToHex
        + AsRef<[u8]>
        + PartialEq
        + Eq;
    type PrivateKey: Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + hex::FromHex<Error = FromHexError>
        + hex::ToHex
        + AsRef<[u8]>;

    fn generate_encryption_keypair() -> EncryptionKeyPair<Self>;
    fn finish_key_exchange(exchange: CompletedKeyExchange<Self>) -> KeyMaterial;
}
