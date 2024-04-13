use crate::crypto::hash_provider::sha512::Sha512;
use crate::crypto::signature_providers::ed25519::Ed25519;
use crate::crypto::signed_box::{SignedBox, SignedBoxError};
use crate::crypto::tagged_types::keys::PublicSigningKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaggedSignedBox {
    Ed25519(SignedBox<Ed25519, Sha512>),
}

impl TaggedSignedBox {
    pub fn verify<T: for<'d> Deserialize<'d>>(self) -> Result<T, SignedBoxError> {
        match self {
            TaggedSignedBox::Ed25519(inner) => inner.verify(),
        }
    }

    pub fn public_key(&self) -> PublicSigningKey {
        match self {
            TaggedSignedBox::Ed25519(inner) => PublicSigningKey::Ed25519(*inner.public_key()),
        }
    }
}
