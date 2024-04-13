use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::signature_providers::ed25519::Ed25519;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicEncryptionKey {
    X25519(#[serde(with = "hex::serde")] <X25519 as AsymmetricEncryptionProvider>::PublicKey),
}

impl From<<X25519 as AsymmetricEncryptionProvider>::PublicKey> for PublicEncryptionKey {
    fn from(value: <X25519 as AsymmetricEncryptionProvider>::PublicKey) -> Self {
        PublicEncryptionKey::X25519(value)
    }
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

pub trait KeyOrder {
    /// Assigns each key variant a value. A higher value prioritizes a key over others with a lower value.
    /// A priority of smaller than zero indicates that the key algorithm should not be used for future encryption.
    fn order(&self) -> i8;
}

impl KeyOrder for &PublicEncryptionKey {
    fn order(&self) -> i8 {
        match self {
            PublicEncryptionKey::X25519(_) => 1,
        }
    }
}

impl KeyOrder for &PublicSigningKey {
    fn order(&self) -> i8 {
        match self {
            PublicSigningKey::Ed25519(_) => 1,
        }
    }
}

pub trait KeyOrderExt: Sized {
    type Item: KeyOrder;

    /// Chooses a key from an iterator of keys.
    /// The key with the highest [KeyOrder] will be returned
    /// Keys with an order smaller than zero will never be returned
    fn get_recommended_key(self) -> Option<Self::Item>;
}

impl<T: Iterator<Item = K>, K: KeyOrder> KeyOrderExt for T {
    type Item = K;

    fn get_recommended_key(self) -> Option<Self::Item> {
        self.filter(|e| e.order() >= 0).max_by_key(|e| e.order())
    }
}
