use crate::crypto::signed_box::SignedBoxError;
use crate::crypto::tagged_types::tagged_signed_box::TaggedSignedBox;
use crate::model::connection_details::PublicMycelinkConnectionDetails;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkContact {
    display_name: Box<str>,
    connection_details: PublicMycelinkConnectionDetails,
}

impl MycelinkContact {
    pub fn new(
        display_name: Box<str>,
        connection_details: PublicMycelinkConnectionDetails,
    ) -> Self {
        Self {
            display_name,
            connection_details,
        }
    }
    pub fn display_name(&self) -> &str {
        &self.display_name
    }
    pub fn connection_details(&self) -> &PublicMycelinkConnectionDetails {
        &self.connection_details
    }

    pub fn verify_signed_box<T: for<'d> Deserialize<'d>>(
        &self,
        signed: TaggedSignedBox,
    ) -> Result<T, SignatureVerificationError> {
        if self
            .connection_details
            .public_signing_keys()
            .iter()
            .find(|e| e == &&signed.public_key())
            .is_none()
        {
            return Err(SignatureVerificationError::NoMatchingKey);
        }

        Ok(signed.verify()?)
    }
}

impl From<MycelinkContact> for PublicMycelinkConnectionDetails {
    fn from(value: MycelinkContact) -> Self {
        value.connection_details
    }
}

#[derive(Debug)]
pub enum SignatureVerificationError {
    SignedBoxError(SignedBoxError),
    NoMatchingKey,
}

impl From<SignedBoxError> for SignatureVerificationError {
    fn from(value: SignedBoxError) -> Self {
        Self::SignedBoxError(value)
    }
}
