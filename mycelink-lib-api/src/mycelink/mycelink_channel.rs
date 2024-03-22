use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::key_exchange::InitiateKeyExchange;
use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::ratchet::Ratchet;
use crate::crypto::secret_box::{DefaultSecretBox, SecretBoxError};
use crate::crypto::symmetrical_providers::{
    DefaultSymmetricEncryptionProvider, SymmetricEncryptionProvider,
};
use crate::fcp_tools::fcp_get::{fcp_get_inline, FcpGetError};
use crate::fcp_tools::fcp_put::{fcp_put_inline, FcpPutError};
use crate::model::tagged_key_exchange::TaggedInitiateKeyExchange;
use crate::model::tagged_keypair::TaggedEncryptionKeyPair;
use crate::model::tagged_secret_box::TaggedSecretBox;
use crate::mycelink::compressed_box::{CompressedBox, CompressionHint};
use crate::mycelink::mycelink_channel_message::{
    InitialResponderChannelMessage, MycelinkChannelMessage,
};
use crate::mycelink::mycelink_channel_request::OpenChannelError;
use crate::mycelink::mycelink_ratchet_key_generator::MycelinkRatchetKeyGenerator;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChannelRole {
    Initiator,
    Responder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChannel {
    role: MycelinkChannelRole,
    send_ratchet: Ratchet,
    receive_ratchet: Ratchet,
    pending_keys: Vec<TaggedEncryptionKeyPair>,
    pending_rekey: Box<[TaggedInitiateKeyExchange]>,
}

impl MycelinkChannel {
    fn new(
        send_secret: KeyMaterial,
        receive_secret: KeyMaterial,
        role: MycelinkChannelRole,
        kdf: KdfProviderTag,
        pending_keys: Vec<TaggedEncryptionKeyPair>,
        pending_rekey: Box<[TaggedInitiateKeyExchange]>,
    ) -> Self {
        let send_ratchet = Ratchet::new(send_secret, kdf);
        let receive_ratchet = Ratchet::new(receive_secret, kdf);

        MycelinkChannel {
            role,
            send_ratchet,
            receive_ratchet,
            pending_keys,
            pending_rekey,
        }
    }

    pub async fn open_initiator(
        send_secret: KeyMaterial,
        receive_secret: KeyMaterial,
        kdf: KdfProviderTag,
        fcp_connector: &FCPConnector,
    ) -> Result<MycelinkChannel, OpenChannelError> {
        let mut channel = Self::new(
            send_secret,
            receive_secret,
            MycelinkChannelRole::Initiator,
            kdf,
            vec![],
            Box::new([]),
        );

        let initial_message: InitialResponderChannelMessage =
            channel.try_receive(fcp_connector).await?;

        channel.pending_rekey = initial_message.available_public_component;

        Ok(channel)
    }

    pub async fn open_responder(
        send_secret: KeyMaterial,
        receive_secret: KeyMaterial,
        kdf: KdfProviderTag,
        fcp_connector: &FCPConnector,
    ) -> Result<Self, OpenChannelError> {
        let (x25519_initiate, x25519_private) = InitiateKeyExchange::<X25519>::new();

        let pending_keys = vec![TaggedEncryptionKeyPair::X25519(x25519_private)];

        let initial_message = InitialResponderChannelMessage {
            available_public_component: Box::new([TaggedInitiateKeyExchange::X25519(
                x25519_initiate,
            )]),
        };

        let mut channel = Self::new(
            send_secret,
            receive_secret,
            MycelinkChannelRole::Responder,
            kdf,
            pending_keys,
            initial_message.available_public_component.clone(),
        );

        channel
            .send(&initial_message, CompressionHint::High, fcp_connector)
            .await?;

        Ok(channel)
    }

    async fn send(
        &mut self,
        payload: &impl Serialize,
        compression_hint: CompressionHint,
        fcp_connector: &FCPConnector,
    ) -> Result<(), FcpPutError> {
        let compressed = CompressedBox::compress(payload, compression_hint);

        let encryption_key = self.send_ratchet.generate_message_encryption_key();
        let encryption_key =
            DefaultSymmetricEncryptionProvider::generate_key_from_material(encryption_key);
        let encrypted = DefaultSecretBox::create(&compressed, &encryption_key);
        let encrypted: TaggedSecretBox = encrypted.into();
        let mut encoded_encrypted = Vec::new();
        ciborium::into_writer(&encrypted, &mut encoded_encrypted).unwrap();

        fcp_put_inline(
            encoded_encrypted.into(),
            self.send_ratchet.generate_send_message_ksk(),
            fcp_connector,
        )
        .await?;
        self.send_ratchet.advance();
        Ok(())
    }

    async fn try_receive<T: for<'de> Deserialize<'de>>(
        &mut self,
        fcp_connector: &FCPConnector,
    ) -> Result<T, ReceiveMessageError> {
        let ksk = self.receive_ratchet.generate_send_message_ksk();
        let message = fcp_get_inline(
            ksk,
            fcp_connector,
            "Receive Mycelink Message",
            PriorityClass::High,
        )
        .await?;
        let decryption_key = self.receive_ratchet.generate_message_encryption_key();
        self.receive_ratchet.advance();

        let secret_box: TaggedSecretBox = ciborium::from_reader(message.data.as_ref())?;
        let compressed: CompressedBox = secret_box.try_decrypt(decryption_key)?;
        let original: T = compressed.open()?;

        Ok(original)
    }

    pub async fn try_receive_message(
        &mut self,
        fcp_connector: &FCPConnector,
    ) -> Result<MycelinkChannelMessage, ReceiveMessageError> {
        self.try_receive(fcp_connector).await
    }
}

#[derive(Debug)]
pub enum ReceiveMessageError {
    SecretBox(SecretBoxError),
    Deserialize(ciborium::de::Error<std::io::Error>),
    FcpGet(FcpGetError),
}

impl From<SecretBoxError> for ReceiveMessageError {
    fn from(value: SecretBoxError) -> Self {
        Self::SecretBox(value)
    }
}

impl From<ciborium::de::Error<std::io::Error>> for ReceiveMessageError {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        Self::Deserialize(value)
    }
}

impl From<FcpGetError> for ReceiveMessageError {
    fn from(value: FcpGetError) -> Self {
        Self::FcpGet(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::key_exchange::InitiateKeyExchange;
    use crate::crypto::key_exchange_providers::DefaultAsymmetricEncryptionProvider;
    use crate::mycelink::mycelink_channel_request::MycelinkChannelRequest;
    use crate::test::create_test_fcp_connector;

    #[tokio::test]
    async fn test_open_channel() {
        let fcp_connector = create_test_fcp_connector("test_open_channel").await;

        let (public_responder_static_key, private_responder_static_keys) =
            InitiateKeyExchange::<DefaultAsymmetricEncryptionProvider>::new();

        let (incoming_request, pending_request) =
            MycelinkChannelRequest::create(public_responder_static_key.into());

        let channel_receiver = incoming_request
            .accept(&[&private_responder_static_keys.into()], &fcp_connector)
            .await
            .unwrap();

        let channel_initiator = pending_request.check(&fcp_connector).await.unwrap();
    }
}
