use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::secret_box::SecretBoxError;
use crate::crypto::signed_box::SignedBoxError;
use crate::fcp_tools::fcp_put::FcpPutError;
use crate::model::keys::{PublicEncryptionKey, PublicSigningKey};
use crate::model::tagged_key_exchange::{TaggedAnswerKeyExchange, TaggedInitiateKeyExchange};
use crate::model::tagged_keypair::TaggedEncryptionKeyPair;
use crate::model::tagged_secret_box::TaggedSecretBox;
use crate::model::tagged_signed_box::TaggedSignedBox;
use crate::mycelink::mycelink_channel::{MycelinkChannel, ReceiveMessageError};
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChannelRequest {
    keys: TaggedAnswerKeyExchange,
    kdf: KdfProviderTag,
}

impl MycelinkChannelRequest {
    pub async fn accept(
        self,
        keypair_candidates: &[&TaggedEncryptionKeyPair],
        fcp_connector: &FCPConnector,
    ) -> Result<MycelinkChannel, OpenChannelError> {
        for candidate in keypair_candidates {
            if let Ok(key) = self.keys.try_complete(candidate) {
                let (send_key, receive_key) = derive_send_request_keys(
                    key,
                    self.keys.initiate_public_key(),
                    self.keys.answer_public_key(),
                    self.kdf,
                );

                return MycelinkChannel::open_responder(
                    send_key,
                    receive_key,
                    self.kdf,
                    fcp_connector,
                )
                .await;
            }
        }

        Err(OpenChannelError::NoMatchingKey)
    }
    pub fn create(
        responder_public_key: TaggedInitiateKeyExchange,
    ) -> (Self, PendingMycelinkChannelRequest) {
        let (answer, shared_secret) = responder_public_key.answer();

        let kdf = KdfProviderTag::default();
        (
            Self {
                keys: answer.clone(),
                kdf,
            },
            PendingMycelinkChannelRequest {
                shared_secret,
                exchange: answer,
                kdf,
            },
        )
    }
}

fn derive_send_request_keys(
    material: KeyMaterial,
    sender_public_key: PublicEncryptionKey,
    recipient_public_key: PublicEncryptionKey,
    kdf: KdfProviderTag,
) -> (KeyMaterial, KeyMaterial) {
    let send_key = kdf.as_provider().derive_key(
        &material,
        &format!("Mycelink open-channel {}", hex::encode(sender_public_key)),
    );

    let receive_key = kdf.as_provider().derive_key(
        &material,
        &format!(
            "Mycelink open-channel {}",
            hex::encode(recipient_public_key)
        ),
    );

    (send_key, receive_key)
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingMycelinkChannelRequest {
    shared_secret: KeyMaterial,
    exchange: TaggedAnswerKeyExchange,
    kdf: KdfProviderTag,
}

impl PendingMycelinkChannelRequest {
    pub async fn check(
        &self,
        fcp_connector: &FCPConnector,
    ) -> Result<MycelinkChannel, OpenChannelError> {
        let (send_key, receive_key) = derive_send_request_keys(
            self.shared_secret.clone(),
            self.exchange.answer_public_key(),
            self.exchange.initiate_public_key(),
            self.kdf,
        );

        MycelinkChannel::open_initiator(send_key, receive_key, self.kdf, fcp_connector).await
    }
}

#[derive(Debug)]
pub enum OpenChannelError {
    NoMatchingKey,
    DecryptionError(SecretBoxError),
    SignatureError(SignedBoxError),
    FcpPutError(FcpPutError),
    ReceiveMessageError(ReceiveMessageError),
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

impl From<FcpPutError> for OpenChannelError {
    fn from(value: FcpPutError) -> Self {
        OpenChannelError::FcpPutError(value)
    }
}

impl From<ReceiveMessageError> for OpenChannelError {
    fn from(value: ReceiveMessageError) -> Self {
        Self::ReceiveMessageError(value)
    }
}
