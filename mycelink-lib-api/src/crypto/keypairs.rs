use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignatureKeyPair<P: SignatureProvider + ?Sized> {
    pub public_key: P::PublicKey,
    pub private_key: P::PrivateKey,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionKeyPair<P: AsymmetricEncryptionProvider + ?Sized> {
    pub public_key: P::PublicKey,
    pub private_key: P::PrivateKey,
}
