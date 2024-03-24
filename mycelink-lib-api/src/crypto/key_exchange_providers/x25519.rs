use crate::crypto::key_exchange::CompletedKeyExchange;
use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::keypairs::EncryptionKeyPair;
use x25519_dalek::{x25519, PublicKey, StaticSecret};

#[derive(Debug)]
pub struct X25519 {}
impl AsymmetricEncryptionProvider for X25519 {
    type Provider = X25519;

    type PublicKey = [u8; 32];
    type PrivateKey = [u8; 32];

    fn generate_encryption_keypair() -> EncryptionKeyPair<Self> {
        let secret = StaticSecret::random();
        let public = PublicKey::from(&secret);

        EncryptionKeyPair {
            public_key: public.to_bytes(),
            private_key: secret.to_bytes(),
        }
    }

    fn finish_key_exchange(exchange: CompletedKeyExchange<Self>) -> KeyMaterial {
        let (public, private) = exchange.into_components();
        x25519(private, public).into()
    }
}
