use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SignatureKeyPair<P: SignatureProvider> {
    pub public_key: P::PublicKey,
    pub private_key: P::PrivateKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionKeyPair<P: AsymmetricEncryptionProvider> {
    pub public_key: P::PublicKey,
    pub private_key: P::PrivateKey,
}
