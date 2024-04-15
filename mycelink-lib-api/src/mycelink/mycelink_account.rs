use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::signature_providers::ed25519::Ed25519;
use crate::crypto::signature_providers::SignatureProvider;
use crate::crypto::tagged_types::tagged_keypair::{
    TaggedEncryptionKeyPair, TaggedSignatureKeyPair,
};
use crate::db::actions::mycelink_account_actions::MycelinkAccountEntryError;
use crate::fcp_tools::fcp_put::{fcp_put_inline, FcpPutError};
use crate::fcp_tools::generate_ssk::{generate_ssk, GenerateSSKKeypairError};
use crate::model::connection_details::PublicMycelinkConnectionDetails;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub async fn create_new(
        display_name: impl Into<Box<str>>,
        fcp: &FCPConnector,
    ) -> Result<Self, CreateAccountError> {
        let encryption_keys = vec![X25519::generate_encryption_keypair().into()];
        let signing_keys = vec![Ed25519::generate_signing_keypair().into()];

        let ssk_keypair = generate_ssk(fcp).await?;

        let account = Self {
            request_ssk_key: ssk_keypair.request_uri,
            insert_ssk_key: ssk_keypair.insert_uri,
            encryption_keys,
            signing_keys,
        };

        account.publish(display_name, fcp).await?;
        Ok(account)
    }

    async fn publish(
        &self,
        display_name: impl Into<Box<str>>,
        fcp: &FCPConnector,
    ) -> Result<(), FcpPutError> {
        let public_details = self.generate_contact_info(display_name);
        let mut public_details_buf = Vec::new();
        ciborium::into_writer(&public_details, &mut public_details_buf).unwrap();

        fcp_put_inline(
            public_details_buf.into(),
            self.insert_ssk_key.deref().try_into().unwrap(),
            fcp,
            "publish account",
        )
        .await?;
        Ok(())
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

#[derive(Debug)]
pub enum CreateAccountError {
    GenerateSSK(GenerateSSKKeypairError),
    FcpPut(FcpPutError),
    AccountEntry(MycelinkAccountEntryError),
}

impl From<GenerateSSKKeypairError> for CreateAccountError {
    fn from(value: GenerateSSKKeypairError) -> Self {
        Self::GenerateSSK(value)
    }
}

impl From<FcpPutError> for CreateAccountError {
    fn from(value: FcpPutError) -> Self {
        Self::FcpPut(value)
    }
}

impl From<MycelinkAccountEntryError> for CreateAccountError {
    fn from(value: MycelinkAccountEntryError) -> Self {
        Self::AccountEntry(value)
    }
}

impl From<sqlx::Error> for CreateAccountError {
    fn from(value: sqlx::Error) -> Self {
        Self::AccountEntry(MycelinkAccountEntryError::SqlxError { inner: value })
    }
}
