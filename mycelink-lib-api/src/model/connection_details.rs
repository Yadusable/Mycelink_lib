use crate::crypto::tagged_types::keys::PublicSigningKey;
use crate::crypto::tagged_types::tagged_key_exchange::TaggedInitiateKeyExchange;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PublicConnectionDetails {
    Mycelink(PublicMycelinkConnectionDetails),
}

impl TryFrom<PublicConnectionDetails> for PublicMycelinkConnectionDetails {
    type Error = ();

    fn try_from(value: PublicConnectionDetails) -> Result<Self, Self::Error> {
        if let PublicConnectionDetails::Mycelink(val) = value {
            Ok(val)
        } else {
            Err(())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMycelinkConnectionDetails {
    account_request_key: Box<str>,
    display_name: Box<str>,
    profile_picture: Option<Box<[u8]>>,
    public_signing_keys: Box<[PublicSigningKey]>,
    public_encryption_keys: Box<[TaggedInitiateKeyExchange]>,
    channel_request_droppoint: Box<str>,
}

impl PublicMycelinkConnectionDetails {
    pub fn new(
        account_request_key: Box<str>,
        display_name: impl Into<Box<str>>,
        public_signing_keys: Box<[PublicSigningKey]>,
        public_encryption_keys: Box<[TaggedInitiateKeyExchange]>,
        channel_request_droppoint: Box<str>,
    ) -> Self {
        Self {
            account_request_key,
            display_name: display_name.into(),
            profile_picture: None,
            public_signing_keys,
            public_encryption_keys,
            channel_request_droppoint,
        }
    }

    pub fn account_request_key(&self) -> &Box<str> {
        &self.account_request_key
    }
    pub fn display_name(&self) -> &Box<str> {
        &self.display_name
    }
    pub fn public_signing_keys(&self) -> &Box<[PublicSigningKey]> {
        &self.public_signing_keys
    }
    pub fn public_encryption_keys(&self) -> &Box<[TaggedInitiateKeyExchange]> {
        &self.public_encryption_keys
    }
    pub fn channel_request_droppoint(&self) -> &Box<str> {
        &self.channel_request_droppoint
    }
}
