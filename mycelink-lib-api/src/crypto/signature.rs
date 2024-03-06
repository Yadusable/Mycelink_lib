use crate::crypto::hash_provider::HashProvider;
use crate::crypto::keypairs::SignatureKeyPair;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature<P: SignatureProvider, H: HashProvider> {
    public_key: P::PublicKey,

    hash: H::Hash,
}

impl<P: SignatureProvider, H: HashProvider> Signature<P, H> {
    pub fn sign(data: &[u8], keys: SignatureKeyPair<P>) -> Signature<P, H> {
        todo!()
    }
}
