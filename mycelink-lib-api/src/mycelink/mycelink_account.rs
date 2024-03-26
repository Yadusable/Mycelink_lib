use crate::crypto::tagged_types::tagged_keypair::{
    TaggedEncryptionKeyPair, TaggedSignatureKeyPair,
};
use crate::model::connection_details::PublicMycelinkConnectionDetails;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkAccount {
    request_ssk_key: Box<str>,
    insert_ssk_key: Box<str>,
    encryption_keys: Vec<TaggedEncryptionKeyPair>,

    signing_keys: Vec<TaggedSignatureKeyPair>,
}

impl MycelinkAccount {
    pub fn new(
        request_ssk_key: Box<str>,
        insert_ssk_key: Box<str>,
        encryption_keys: Vec<TaggedEncryptionKeyPair>,
        signing_keys: Vec<TaggedSignatureKeyPair>,
    ) -> Self {
        Self {
            request_ssk_key,
            insert_ssk_key,
            encryption_keys,
            signing_keys,
        }
    }

    pub fn request_ssk_key(&self) -> &str {
        &self.request_ssk_key
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

        todo!();
    }

    pub fn generate_contact_info(
        &self,
        display_name: impl Into<Box<str>>,
    ) -> PublicMycelinkConnectionDetails {
        PublicMycelinkConnectionDetails::new(
            self.request_ssk_key.clone(),
            display_name,
            self.signing_keys.iter().map(|e| e.public_key()).collect(),
            self.encryption_keys
                .iter()
                .map(|e| e.public_key())
                .collect(),
        )
    }
    pub(crate) fn insert_ssk_key(&self) -> &str {
        &self.insert_ssk_key
    }
}

impl PartialEq for MycelinkAccount {
    fn eq(&self, other: &Self) -> bool {
        self.request_ssk_key == other.request_ssk_key
    }
}

impl Eq for MycelinkAccount {}
