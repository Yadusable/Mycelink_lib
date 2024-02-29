use crate::model::asymmetric_keys::{
    PrivateEncryptionKey, PrivateSigningKey, PublicEncryptionKey, PublicSigningKey,
};
use crate::model::connection_details::PublicMycelinkConnectionDetails;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MycelinkAccount {
    request_ssk_key: Box<str>,
    insert_ssk_key: Box<str>,

    public_encryption_key: PublicEncryptionKey,
    private_encryption_key: PrivateEncryptionKey,

    public_signing_key: PublicSigningKey,
    private_signing_key: PrivateSigningKey,
}

impl MycelinkAccount {
    pub fn request_ssk_key(&self) -> &str {
        &self.request_ssk_key
    }

    pub fn new(
        request_ssk_key: Box<str>,
        insert_ssk_key: Box<str>,
        public_encryption_key: PublicEncryptionKey,
        private_encryption_key: PrivateEncryptionKey,
        public_signing_key: PublicSigningKey,
        private_signing_key: PrivateSigningKey,
    ) -> Self {
        Self {
            request_ssk_key,
            insert_ssk_key,
            public_encryption_key,
            private_encryption_key,
            public_signing_key,
            private_signing_key,
        }
    }

    pub fn create_new(request_ssk_key: Box<str>, insert_ssk_key: Box<str>) -> Self {
        let encryption_keys = x25519_dalek::StaticSecret::random_from_rng(rand::rngs::OsRng);
        let mut signing_key_secret = [0; 32];
        rand::rngs::OsRng::default().fill_bytes(&mut signing_key_secret);
        let signing_keys = ed25519_dalek::SigningKey::from_bytes(&signing_key_secret);

        let public_encryption_key = x25519_dalek::PublicKey::from(&encryption_keys);
        let private_encryption_key = encryption_keys.as_bytes();

        let public_signing_key = signing_keys.verifying_key();
        let private_signing_key = signing_keys.as_bytes();

        Self::new(
            request_ssk_key,
            insert_ssk_key,
            PublicEncryptionKey::X25519 {
                raw_key: *public_encryption_key.as_bytes(),
            },
            PrivateEncryptionKey::X25519 {
                raw_key: *private_encryption_key,
            },
            PublicSigningKey::Ed25519 {
                raw_key: *public_signing_key.as_bytes(),
            },
            PrivateSigningKey::Ed25519 {
                raw_key: *private_signing_key,
            },
        )
    }

    pub fn generate_contact_info(
        &self,
        display_name: impl Into<Box<str>>,
    ) -> PublicMycelinkConnectionDetails {
        PublicMycelinkConnectionDetails::new(
            self.request_ssk_key.clone(),
            display_name,
            self.public_signing_key.clone(),
            self.public_encryption_key.clone(),
        )
    }

    pub(crate) fn insert_ssk_key(&self) -> &str {
        &self.insert_ssk_key
    }
}
