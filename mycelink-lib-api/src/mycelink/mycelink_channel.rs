use crate::crypto::hash_provider::HashProvider;
use crate::crypto::rachet::Ratchet;
use crate::crypto::secret_box::SecretBoxError;
use crate::crypto::signed_box::SignedBoxError;
use crate::model::keys::{PublicEncryptionKey, PublicSigningKey};
use crate::model::ratchet_hash_tag::RatchetHashTag;
use crate::model::tagged_key_exchange::TaggedAnswerKeyExchange;
use crate::model::tagged_keypair::TaggedEncryptionKeyPair;
use crate::model::tagged_secret_box::TaggedSecretBox;
use crate::model::tagged_signed_box::TaggedSignedBox;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChannel<H: HashProvider> {
    send_ratchet: Ratchet<H>,
    receive_ratchet: Ratchet<H>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChannelRequest {
    keys: TaggedAnswerKeyExchange,
    ratchet_hash: RatchetHashTag,
}

impl MycelinkChannelRequest {
    pub fn accept(self, keypair_candidates: &[&TaggedEncryptionKeyPair]) {
        for candidate in keypair_candidates {
            if let Ok(keys) = self.keys.try_complete(candidate) {}
        }

        todo!()
    }

    pub fn used_public_component(&self) -> PublicEncryptionKey {
        self.keys.public_component()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedMycelinkChannelRequest(TaggedSignedBox);

impl SignedMycelinkChannelRequest {
    pub fn verify(self) -> Result<(MycelinkChannelRequest, PublicSigningKey), SignedBoxError> {
        let public_key = self.0.public_key();
        let request = self.0.verify()?;

        Ok((request, public_key))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedSignedMycelinkChannelRequest {
    data: TaggedSecretBox,
    encryption_keys: TaggedAnswerKeyExchange,
}

impl EncryptedSignedMycelinkChannelRequest {
    pub fn try_open(
        self,
        keypair_candidates: &[&TaggedEncryptionKeyPair],
    ) -> Result<(MycelinkChannelRequest, PublicSigningKey), OpenChannelError> {
        let signed_request = self.try_decrypt(keypair_candidates)?;

        Ok(signed_request.verify()?)
    }

    fn try_decrypt(
        self,
        keypair_candidates: &[&TaggedEncryptionKeyPair],
    ) -> Result<SignedMycelinkChannelRequest, OpenChannelError> {
        for candidate in keypair_candidates {
            if let Ok(material) = self.encryption_keys.try_complete(candidate) {
                return Ok(self.data.try_decrypt(material)?);
            }
        }
        Err(OpenChannelError::NoMatchingKey)
    }
}

#[derive(Debug)]
pub enum OpenChannelError {
    NoMatchingKey,
    DecryptionError(SecretBoxError),
    SignatureError(SignedBoxError),
}

impl From<SecretBoxError> for OpenChannelError {
    fn from(value: SecretBoxError) -> Self {
        OpenChannelError::DecryptionError(value)
    }
}

impl From<SignedBoxError> for OpenChannelError {
    fn from(value: SignedBoxError) -> Self {
        OpenChannelError::SignatureError(value)
    }
}
