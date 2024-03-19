use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::signature_providers::ed25519::Ed25519;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicEncryptionKey {
    X25519(#[serde(with = "hex::serde")] <X25519 as AsymmetricEncryptionProvider>::PublicKey),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicSigningKey {
    Ed25519(#[serde(with = "hex::serde")] <Ed25519 as SignatureProvider>::PublicKey),
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrivateSigningKey {}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrivateEncryptionKey {
    X25519(#[serde(with = "hex::serde")] <X25519 as AsymmetricEncryptionProvider>::PrivateKey),
}

impl AsRef<[u8]> for PublicEncryptionKey {
    fn as_ref(&self) -> &[u8] {
        match self {
            PublicEncryptionKey::X25519(inner) => inner,
        }
    }
}
