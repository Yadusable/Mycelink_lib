use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicEncryptionKey {
    X25519(#[serde(with = "hex::serde")] <X25519 as AsymmetricEncryptionProvider>::PublicKey),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicSigningKey {}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrivateSigningKey {}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrivateEncryptionKey {
    X25519(#[serde(with = "hex::serde")] <X25519 as AsymmetricEncryptionProvider>::PrivateKey),
}
