use crate::crypto::key_exchange_providers::x25519;
use crate::crypto::keypairs::{EncryptionKeyPair, SignatureKeyPair};
use crate::crypto::signature_providers::ed25519;
use crate::crypto::tagged_types::keys::{PublicEncryptionKey, PublicSigningKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaggedEncryptionKeyPair {
    X25519(EncryptionKeyPair<x25519::X25519>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaggedSignatureKeyPair {
    Ed25519(SignatureKeyPair<ed25519::Ed25519>),
}

impl TaggedSignatureKeyPair {
    pub fn public_key(&self) -> PublicSigningKey {
        match self {
            TaggedSignatureKeyPair::Ed25519(inner) => PublicSigningKey::Ed25519(inner.public_key),
        }
    }
}

impl TaggedEncryptionKeyPair {
    pub fn public_key(&self) -> PublicEncryptionKey {
        match self {
            TaggedEncryptionKeyPair::X25519(inner) => PublicEncryptionKey::X25519(inner.public_key),
        }
    }
}

impl<'a> TryFrom<&'a TaggedEncryptionKeyPair> for &'a EncryptionKeyPair<x25519::X25519> {
    type Error = ();

    fn try_from(value: &'a TaggedEncryptionKeyPair) -> Result<Self, Self::Error> {
        let TaggedEncryptionKeyPair::X25519(value) = value;
        Ok(value)
    }
}

impl From<EncryptionKeyPair<x25519::X25519>> for TaggedEncryptionKeyPair {
    fn from(value: EncryptionKeyPair<x25519::X25519>) -> Self {
        Self::X25519(value)
    }
}

impl TryFrom<TaggedSignatureKeyPair> for SignatureKeyPair<ed25519::Ed25519> {
    type Error = ();

    fn try_from(value: TaggedSignatureKeyPair) -> Result<Self, Self::Error> {
        let TaggedSignatureKeyPair::Ed25519(value) = value;
        Ok(value)
    }
}
