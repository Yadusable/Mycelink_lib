pub mod ed25519;

use crate::crypto::hash_provider::HashProvider;
use crate::crypto::keypairs::SignatureKeyPair;
use hex::FromHexError;
use serde::{Deserialize, Serialize};

pub trait SignatureProvider {
    type PublicKey: Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + hex::FromHex<Error = FromHexError>
        + hex::ToHex
        + AsRef<[u8]>;
    type PrivateKey: Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + hex::FromHex<Error = FromHexError>
        + hex::ToHex
        + AsRef<[u8]>;
    type Signature: Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + hex::FromHex<Error = FromHexError>
        + hex::ToHex
        + AsRef<[u8]>;

    type Hash;

    fn generate_signing_keypair() -> SignatureKeyPair<Self>;
    fn sign<H: HashProvider<Hash = Self::Hash>>(
        hash: &Self::Hash,
        keypair: &SignatureKeyPair<Self>,
    ) -> Self::Signature;
    fn verify<H: HashProvider<Hash = Self::Hash>>(
        signature: &Self::Signature,
        hash: &H::Hash,
        public_key: &Self::PublicKey,
    ) -> bool;
}
