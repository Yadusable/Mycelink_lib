use crate::crypto::hash_provider::HashProvider;
use crate::crypto::keypairs::SignatureKeyPair;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature<P: SignatureProvider, H: HashProvider> {
    public_key: P::PublicKey,
    signature: P::Signature,
    hash: H::Hash,
}

impl<P: SignatureProvider<Provider=P>, H: HashProvider<Hash=P::Hash>> Signature<P, H> {
    pub fn sign(data: &[u8], keys: &SignatureKeyPair<P>) -> Signature<P, H> {
        let hash = H::hash(data);
        let signature = P::sign(&hash, keys);
        
        Signature {
            public_key: keys.public_key.clone(),
            signature,
            hash,
        }
    }
}
