use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::key_exchange::InitiateKeyExchange;
use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::ratchet::Ratchet;
use crate::crypto::secret_box::{DefaultSecretBox, SecretBoxError};
use crate::crypto::symmetrical_providers::{
    DefaultSymmetricEncryptionProvider, SymmetricEncryptionProvider,
};
use crate::crypto::tagged_types::keys::PublicEncryptionKey;
use crate::crypto::tagged_types::tagged_key_exchange::TaggedInitiateKeyExchange;
use crate::crypto::tagged_types::tagged_keypair::TaggedEncryptionKeyPair;
use crate::crypto::tagged_types::tagged_secret_box::TaggedSecretBox;
use crate::fcp_tools::fcp_get::{fcp_get_inline, FcpGetError};
use crate::fcp_tools::fcp_put::{fcp_put_inline, FcpPutError};
use crate::mycelink::protocol::compressed_box::{
    CompressedBox, CompressionHint, CompressionHinting,
};
use crate::mycelink::protocol::mycelink_channel::ReceiveMessageError::{
    FailedRekey, NotInitialized,
};
use crate::mycelink::protocol::mycelink_channel_message::MycelinkChannelMessage::FinalMessage;
use crate::mycelink::protocol::mycelink_channel_message::{
    InitialChannelMessage, MycelinkChannelMessage,
};
use crate::mycelink::protocol::mycelink_chat_message::{
    MycelinkChatMessage, MycelinkChatMessageId, MycelinkChatMessageType,
};
use crate::mycelink::protocol::mycelink_ratchet_key_generator::MycelinkRatchetKeyGenerator;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::messages::get_failed::DATA_NOT_FOUND_CODE;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use serde::{Deserialize, Serialize};
use std::cmp::max_by;
use std::time::UNIX_EPOCH;

/// Basic structure for secret communication
///
/// A Mycelink Channel provides an encrypted communication channel over a distributed Hashtable
/// as provided by Hyphanet.
/// There are always exactly two parties in a channel with exactly one Initiator and one Responder.
///
/// The connection workflow is a follows:
/// 1: Both parties receive some common secret over a prior key exchange
/// 2: Both parties send a [InitialChannelMessage] with newly generated ephemeral public keys in all supported schemas.
/// 3: Both parties may send as many non-final [MycelinkChannelMessage] as they like.
/// 4: Any party can at any point rekey its sending ratchet by sending a [MycelinkChannelMessage::FinalMessage] provided it has unused pending public components from the receiver
///
/// Forward secrecy is provided for following channels, as they are secrets depend on a fully ephemeral key exchange.
/// Deniability is provided as no messages are signed, meaning that any party able to read or verify any message is also able of forging a message.
///
/// A new [MycelinkChannel] can be created using a [super::mycelink_channel_request::MycelinkChannelRequest]
#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChannel {
    send_ratchet: Ratchet,
    receive_ratchet: Ratchet,

    received_initial_message: bool,
    pending_public_components: Option<Box<[TaggedInitiateKeyExchange]>>,
    own_private_component: Vec<Box<[TaggedEncryptionKeyPair]>>,
}

impl MycelinkChannel {
    pub async fn open(
        common_secret: &KeyMaterial,
        own_public_key: PublicEncryptionKey,
        recipient_public_key: PublicEncryptionKey,
        kdf: KdfProviderTag,
        fcp_connector: &FCPConnector,
    ) -> Result<Self, FcpPutError> {
        let send_key = kdf.as_provider().derive_key(
            common_secret,
            &format!("Mycelink open-channel {}", hex::encode(own_public_key)),
        );

        let receive_key = kdf.as_provider().derive_key(
            common_secret,
            &format!(
                "Mycelink open-channel {}",
                hex::encode(recipient_public_key)
            ),
        );

        let send_ratchet = Ratchet::new(send_key, kdf);
        let receive_ratchet = Ratchet::new(receive_key, kdf);

        let (public_components, private_components) = Self::prepare_rekey();

        let mut channel = MycelinkChannel {
            send_ratchet,
            receive_ratchet,

            received_initial_message: false,
            own_private_component: vec![private_components.into()],
            pending_public_components: None,
        };

        let initial_message = InitialChannelMessage {
            available_public_component: public_components.into(),
        };

        channel
            .send(&initial_message, CompressionHint::Fast, fcp_connector)
            .await?;

        Ok(channel)
    }

    fn prepare_rekey() -> ([TaggedInitiateKeyExchange; 1], [TaggedEncryptionKeyPair; 1]) {
        let (x25519_exchange, x25519_keys) = InitiateKeyExchange::<X25519>::new();
        let private_components = [x25519_keys.into()];
        let public_components = [x25519_exchange.into()];
        (public_components, private_components)
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
            "Send Mycelink Channel",
        )
        .await?;
        self.send_ratchet.advance();

        Ok(())
    }

    pub async fn send_channel_message(
        &mut self,
        message: &MycelinkChannelMessage,
        fcp_connector: &FCPConnector,
    ) -> Result<(), FcpPutError> {
        let rekeyed = self.rekey_send_if_possible(message, fcp_connector).await?;

        // If rekey_send wasn't possible, send message without rekeying
        if !rekeyed {
            self.send(message, message.compression_hint(), fcp_connector)
                .await?;
        }

        Ok(())
    }

    pub async fn send_chat_message(
        &mut self,
        message: MycelinkChatMessageType,
        fcp_connector: &FCPConnector,
    ) -> Result<MycelinkChatMessageId, FcpPutError> {
        let message_id = MycelinkChatMessageId::new();
        let message = MycelinkChatMessage::new(
            UNIX_EPOCH.elapsed().unwrap().as_secs(),
            message_id.clone(),
            message,
        );

        let message = MycelinkChannelMessage::DirectMessage(message);

        self.send_channel_message(&message, fcp_connector)
            .await
            .map(|_| message_id)
    }

    async fn rekey_send_if_possible(
        &mut self,
        attached_message: &MycelinkChannelMessage,
        fcp_connector: &FCPConnector,
    ) -> Result<bool, FcpPutError> {
        if let Some(pending_public_components) = &self.pending_public_components {
            if pending_public_components.len() > 0 {
                let public_component = pending_public_components
                    .iter()
                    .fold(&pending_public_components[0], |best, e| {
                        max_by(e, best, TaggedInitiateKeyExchange::preference)
                    });

                let (answer, new_secret) = public_component.answer();
                let (next_public_components, next_private_components) = Self::prepare_rekey();
                let new_kdf = KdfProviderTag::default();

                let final_message = FinalMessage {
                    new_key: answer,
                    next_public_components: next_public_components.into(),
                    new_kdf,

                    attached_message: attached_message.clone().into(),
                };

                log::debug!("Send Final Message message");
                self.send(
                    &final_message,
                    final_message.compression_hint(),
                    fcp_connector,
                )
                .await?;
                log::info!("Rekeyed send ratchet");
                self.own_private_component
                    .push(next_private_components.into());
                self.send_ratchet = Ratchet::new(new_secret, new_kdf);
                self.pending_public_components = None;
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn try_receive<T: for<'de> Deserialize<'de>>(
        &mut self,
        fcp_connector: &FCPConnector,
    ) -> Result<Option<T>, ReceiveMessageError> {
        let ksk = self.receive_ratchet.generate_send_message_ksk();
        let message = fcp_get_inline(
            ksk,
            fcp_connector,
            "Receive Mycelink Message",
            PriorityClass::High,
        )
        .await;

        if let Err(FcpGetError::GetFailed { inner }) = &message {
            if inner.code == DATA_NOT_FOUND_CODE {
                return Ok(None);
            }
        }

        let message = message?;

        let decryption_key = self.receive_ratchet.generate_message_encryption_key();
        self.receive_ratchet.advance();

        let secret_box: TaggedSecretBox = ciborium::from_reader(message.data.as_ref())?;
        let compressed: CompressedBox = secret_box.try_decrypt(decryption_key)?;
        let original: T = compressed.open()?;

        Ok(Some(original))
    }

    pub async fn try_receive_initial_message(
        &mut self,
        fcp_connector: &FCPConnector,
    ) -> Result<(), ReceiveMessageError> {
        let initial_message: InitialChannelMessage = self
            .try_receive(fcp_connector)
            .await?
            .ok_or(NotInitialized)?;
        self.received_initial_message = true;
        self.pending_public_components = Some(initial_message.available_public_component);
        Ok(())
    }

    pub async fn try_receive_message(
        &mut self,
        fcp_connector: &FCPConnector,
    ) -> Result<Option<MycelinkChannelMessage>, ReceiveMessageError> {
        if !self.received_initial_message {
            self.try_receive_initial_message(fcp_connector).await?;
        }

        match self.try_receive(fcp_connector).await? {
            Some(message) => {
                if let FinalMessage {
                    new_key,
                    new_kdf,
                    next_public_components,
                    attached_message,
                } = message
                {
                    log::debug!("Received Final Message message");
                    for i in 0..self.own_private_component.len() {
                        let material =
                            new_key.try_complete_multiple(&self.own_private_component[i]);
                        if material.is_err() {
                            continue;
                        }

                        log::info!("Rekeyed receive ratchet");
                        self.receive_ratchet = Ratchet::new(material.unwrap(), new_kdf);
                        self.pending_public_components = Some(next_public_components);

                        self.own_private_component.drain(0..i);
                        return Ok(Some(*attached_message));
                    }
                    Err(FailedRekey)
                } else {
                    Ok(Some(message))
                }
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug)]
pub enum ReceiveMessageError {
    SecretBox(SecretBoxError),
    Deserialize(ciborium::de::Error<std::io::Error>),
    FcpGet(FcpGetError),
    NotInitialized,
    FailedRekey,
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

    use crate::crypto::kdf_provider::KdfProviderTag;
    use crate::crypto::key_exchange::InitiateKeyExchange;
    use crate::crypto::key_exchange_providers::DefaultAsymmetricEncryptionProvider;
    use crate::crypto::tagged_types::tagged_key_exchange::TaggedAnswerKeyExchange;
    use crate::fcp_tools::fcp_put::FcpPutError;
    use crate::mycelink::mycelink_channel::MycelinkChannel;
    use crate::mycelink::mycelink_channel_message::MycelinkChannelMessage;
    use crate::mycelink::mycelink_chat_message::{
        MycelinkChatMessage, MycelinkChatMessageContent, MycelinkChatMessageType,
    };
    use crate::test::create_test_fcp_connector;
    use mycelink_lib_fcp::fcp_connector::FCPConnector;

    async fn open_channel(
        fcp_connector: &FCPConnector,
    ) -> Result<(MycelinkChannel, MycelinkChannel), FcpPutError> {
        let (start_exchange, _keypair_a) =
            InitiateKeyExchange::<DefaultAsymmetricEncryptionProvider>::new();
        let (answer, completed) = start_exchange.answer();
        let answer: TaggedAnswerKeyExchange = answer.into();
        let material = completed.derive_material();

        let channels = tokio::join!(
            MycelinkChannel::open(
                &material,
                answer.initiate_public_key(),
                answer.answer_public_key(),
                KdfProviderTag::default(),
                fcp_connector,
            ),
            MycelinkChannel::open(
                &material,
                answer.answer_public_key(),
                answer.initiate_public_key(),
                KdfProviderTag::default(),
                fcp_connector,
            ),
        );

        Ok((channels.0.unwrap(), channels.1.unwrap()))
    }

    #[tokio::test]
    async fn test_open_channel() {
        let _ = env_logger::try_init();
        let fcp_connector = create_test_fcp_connector("test_open_channel").await;

        let (mut channel_initiator, mut channel_receiver) =
            open_channel(&fcp_connector).await.unwrap();

        channel_initiator
            .try_receive_initial_message(&fcp_connector)
            .await
            .unwrap();
        channel_receiver
            .try_receive_initial_message(&fcp_connector)
            .await
            .unwrap();

        assert_eq!(
            channel_initiator.send_ratchet.current_key("unit test"),
            channel_receiver.receive_ratchet.current_key("unit test")
        );
        assert_eq!(
            channel_initiator.receive_ratchet.current_key("unit test"),
            channel_receiver.send_ratchet.current_key("unit test")
        );
    }

    #[tokio::test]
    async fn test_sending_channel_messages_initiator() {
        let _ = env_logger::try_init();
        let fcp_connector =
            create_test_fcp_connector("test_sending_channel_messages_initiator").await;

        let (mut channel_initiator, mut channel_receiver) =
            open_channel(&fcp_connector).await.unwrap();

        let message = MycelinkChatMessageType::Standard {
            content: MycelinkChatMessageContent::Text("Hello World".into()),
        };

        channel_initiator
            .send_chat_message(message.clone(), &fcp_connector)
            .await
            .unwrap();

        let received_message = channel_receiver
            .try_receive_message(&fcp_connector)
            .await
            .unwrap();

        let received_message: &MycelinkChatMessage =
            received_message.as_ref().unwrap().try_into().unwrap();

        assert_eq!(received_message.message_type(), &message)
    }

    #[tokio::test]
    async fn test_sending_channel_message_responder() {
        let _ = env_logger::try_init();
        let fcp_connector =
            create_test_fcp_connector("test_sending_channel_message_responder").await;

        let (mut channel_initiator, mut channel_receiver) =
            open_channel(&fcp_connector).await.unwrap();

        let message = MycelinkChatMessageType::Standard {
            content: MycelinkChatMessageContent::Text("Hello World".into()),
        };

        channel_receiver
            .send_chat_message(message.clone(), &fcp_connector)
            .await
            .unwrap();

        let received_message = channel_initiator
            .try_receive_message(&fcp_connector)
            .await
            .unwrap();

        let received_message: &MycelinkChatMessage =
            received_message.as_ref().unwrap().try_into().unwrap();

        assert_eq!(received_message.message_type(), &message);
    }

    #[tokio::test]
    async fn test_sending_rekeys() {
        let _ = env_logger::try_init();
        let fcp_connector = create_test_fcp_connector("test_sending_rekeys").await;

        let (mut channel_a, mut channel_b) = open_channel(&fcp_connector).await.unwrap();

        // Ensure a has b public components
        assert!(channel_a
            .try_receive_message(&fcp_connector)
            .await
            .unwrap()
            .is_none());

        // Send a message
        channel_a
            .send_chat_message(
                MycelinkChatMessageType::Standard {
                    content: MycelinkChatMessageContent::Text("Hello World".into()),
                },
                &fcp_connector,
            )
            .await
            .unwrap();

        // if a has rekeyed, its ratchet should be reset
        assert_eq!(channel_a.send_ratchet.current_iteration(), 0);

        // send message with new key
        channel_a
            .send_chat_message(
                MycelinkChatMessageType::Standard {
                    content: MycelinkChatMessageContent::Text("Hello World 2".into()),
                },
                &fcp_connector,
            )
            .await
            .unwrap();

        // as a has no new public components it should not be able to rekey
        assert_eq!(channel_a.send_ratchet.current_iteration(), 1);

        if let MycelinkChannelMessage::DirectMessage(message) = channel_b
            .try_receive_message(&fcp_connector)
            .await
            .unwrap()
            .unwrap()
        {
            if let MycelinkChatMessageType::Standard {
                content: MycelinkChatMessageContent::Text(text),
            } = message.message_type()
            {
                assert_eq!(&**text, "Hello World")
            } else {
                panic!()
            }
        } else {
            panic!()
        }

        if let MycelinkChannelMessage::DirectMessage(message) = channel_b
            .try_receive_message(&fcp_connector)
            .await
            .unwrap()
            .unwrap()
        {
            if let MycelinkChatMessageType::Standard {
                content: MycelinkChatMessageContent::Text(text),
            } = message.message_type()
            {
                assert_eq!(&**text, "Hello World 2")
            } else {
                panic!()
            }
        } else {
            panic!()
        }
    }
}
