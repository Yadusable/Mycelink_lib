use crate::model::account::Account;
use crate::model::asymmetric_keys::{PublicEncryptionKey, PublicSigningKey};
use base64::Engine;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    #[serde(flatten)]
    identifier: ContactIdentifier,
    display_name: Box<str>,
    public_signing_key: PublicSigningKey,
    public_encryption_key: PublicEncryptionKey,
}

impl Contact {
    pub fn new(
        identifier: ContactIdentifier,
        display_name: impl Into<Box<str>>,
        public_signing_key: PublicSigningKey,
        public_encryption_key: PublicEncryptionKey,
    ) -> Self {
        Self {
            identifier,
            display_name: display_name.into(),
            public_signing_key,
            public_encryption_key,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ContactIdentifier {
    #[serde(rename = "identifier")]
    pub_key: Box<str>,
}

impl ContactIdentifier {
    pub fn new(identifier: impl Into<Box<str>>) -> Self {
        Self {
            pub_key: identifier.into(),
        }
    }

    pub fn get_postbox_ksk_key(&self) -> String {
        let key_material = self.pub_key.as_bytes();
        let context = "mycelink postbox";

        let mut hasher = blake3::Hasher::new_derive_key(context);
        hasher.update(key_material);
        let mut hash_reader = hasher.finalize_xof();

        let mut key = [0; 64];
        hash_reader.fill(&mut key);

        //let key = blake3::derive_key(context, key_material);

        base64::engine::general_purpose::URL_SAFE.encode(key)
    }
}

impl From<Account> for ContactIdentifier {
    fn from(value: Account) -> Self {
        Self {
            pub_key: value.request_ssk_key().into(),
        }
    }
}
