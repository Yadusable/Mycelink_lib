use crate::model::keys::{PublicEncryptionKey, PublicSigningKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PublicConnectionDetails {
    Mycelink {
        inner: PublicMycelinkConnectionDetails,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicMycelinkConnectionDetails {
    account_request_key: Box<str>,
    display_name: Box<str>,
    public_signing_key: PublicSigningKey,
    public_encryption_key: PublicEncryptionKey,
}

impl PublicMycelinkConnectionDetails {
    pub fn new(
        account_request_key: Box<str>,
        display_name: impl Into<Box<str>>,
        public_signing_key: PublicSigningKey,
        public_encryption_key: PublicEncryptionKey,
    ) -> Self {
        Self {
            account_request_key,
            display_name: display_name.into(),
            public_signing_key,
            public_encryption_key,
        }
    }
}
