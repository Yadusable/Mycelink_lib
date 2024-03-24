use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicEncryptionKey {
    X25519 {
        #[serde(with = "hex::serde")]
        raw_key: [u8; 32],
    },
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum PublicSigningKey {
    Ed25519 {
        #[serde(with = "hex::serde")]
        raw_key: [u8; 32],
    },
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrivateSigningKey {
    Ed25519 {
        #[serde(with = "hex::serde")]
        raw_key: [u8; 32],
    },
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum PrivateEncryptionKey {
    X25519 {
        #[serde(with = "hex::serde")]
        raw_key: [u8; 32],
    },
}
