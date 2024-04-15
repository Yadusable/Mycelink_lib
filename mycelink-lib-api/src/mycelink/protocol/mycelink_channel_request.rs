use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::secret_box::{SecretBox, SecretBoxError};
use crate::crypto::signed_box::SignedBoxError;
use crate::crypto::tagged_types::keys::{PublicEncryptionKey, PublicSigningKey};
use crate::crypto::tagged_types::tagged_key_exchange::{
    TaggedAnswerKeyExchange, TaggedInitiateKeyExchange,
};
use crate::crypto::tagged_types::tagged_keypair::{
    TaggedEncryptionKeyPair, TaggedSignatureKeyPair,
};
use crate::crypto::tagged_types::tagged_secret_box::TaggedSecretBox;
use crate::crypto::tagged_types::tagged_signed_box::TaggedSignedBox;
use crate::fcp_tools::fcp_put::FcpPutError;
use crate::mycelink::mycelink_account::{MycelinkAccount, NoMatchingKeyError};
use crate::mycelink::protocol::mycelink_channel::{MycelinkChannel, ReceiveMessageError};
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

/// Basic Structure for creating a new [MycelinkChannel]
///
/// The workflow of opening a new channel between two parties (Alice, Bob) is a follows:
/// 1: Alice receives Bobs public key over a third channel.
/// 2: Alice creates a [MycelinkChannelRequest] with a new ephemeral key, signs it with her publicly known public key and encrypts the request with Bob's public key.
/// 3: Alice uploads the signed and encrypted request to a location know to Bob.
/// 4. Bob receives the request, decrypts it and then verifies the signature.
/// 5. If Bob accepts the request, he uses Alice's ephemeral public key and his static public key to create a new [MycelinkChannel] an Initiator
/// 6: Alice uses Bobs public key and her ephemeral key to receive the new [MycelinkChannel] as the Responder
///
/// This exchange is secure only if Alice can trust Bob's public key and Bob trusts Alice's signing key.
#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChannelRequest {
    contact_request_key: Box<str>,
    keys: TaggedAnswerKeyExchange,
    kdf: KdfProviderTag,
}

impl MycelinkChannelRequest {
    pub async fn accept(
        self,
        keypair_candidates: &[impl Borrow<TaggedEncryptionKeyPair>],
        fcp_connector: &FCPConnector,
    ) -> Result<MycelinkChannel, OpenChannelError> {
        if let Ok(key) = self.keys.try_complete_multiple(keypair_candidates) {
            return Ok(MycelinkChannel::open(
                &key,
                self.keys.initiate_public_key(),
                self.keys.answer_public_key(),
                self.kdf,
                fcp_connector,
            )
            .await?);
        }

        Err(OpenChannelError::NoMatchingKey)
    }
    pub async fn create(
        own_account: &MycelinkAccount,
        responder_public_key: TaggedInitiateKeyExchange,
        fcp_connector: &FCPConnector,
    ) -> Result<(Self, MycelinkChannel), OpenChannelError> {
        let (answer, shared_secret) = responder_public_key.answer();

        let kdf = KdfProviderTag::default();

        Ok((
            Self {
                contact_request_key: own_account.request_ssk_key().into(),
                keys: answer.clone(),
                kdf,
            },
            MycelinkChannel::open(
                &shared_secret,
                answer.answer_public_key(),
                answer.initiate_public_key(),
                kdf,
                fcp_connector,
            )
            .await?,
        ))
    }

    pub fn sign(self, account: &MycelinkAccount) -> SignedMycelinkChannelRequest {
        SignedMycelinkChannelRequest(account.sign(self))
    }

    pub fn contact_request_key(&self) -> &str {
        &self.contact_request_key
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

    pub fn encrypt(
        self,
        recipient_pub: &TaggedInitiateKeyExchange,
    ) -> EncryptedSignedMycelinkChannelRequest {
        let (answer, key) = recipient_pub.answer();
        let encrypted = SecretBox::create(&self, &key.into()).into();
        EncryptedSignedMycelinkChannelRequest {
            data: encrypted,
            encryption_keys: answer,
        }
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
        account: &MycelinkAccount,
    ) -> Result<(MycelinkChannelRequest, PublicSigningKey), OpenChannelError> {
        let key = account.try_complete(self.encryption_keys)?;
        let signed_request: SignedMycelinkChannelRequest = self.data.try_decrypt(key)?;

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

impl From<NoMatchingKeyError> for OpenChannelError {
    fn from(value: NoMatchingKeyError) -> Self {
        Self::NoMatchingKey
    }
}
