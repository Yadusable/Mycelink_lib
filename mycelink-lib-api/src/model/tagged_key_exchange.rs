use crate::crypto::key_exchange::{AnswerKeyExchange, InitiateKeyExchange};
use crate::crypto::key_exchange_providers::x25519::X25519;
use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::keypairs::EncryptionKeyPair;
use crate::model::keys::PublicEncryptionKey;
use crate::model::tagged_keypair::TaggedEncryptionKeyPair;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TaggedInitiateKeyExchange {
    X25519(InitiateKeyExchange<X25519>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaggedAnswerKeyExchange {
    X25519(AnswerKeyExchange<X25519>),
}

impl TaggedInitiateKeyExchange {
    pub fn new_default() -> (TaggedInitiateKeyExchange, TaggedEncryptionKeyPair) {
        let (initial, private_part) = InitiateKeyExchange::new();
        (
            TaggedInitiateKeyExchange::X25519(initial),
            TaggedEncryptionKeyPair::X25519(private_part),
        )
    }

    pub fn answer(self) -> (TaggedAnswerKeyExchange, KeyMaterial) {
        match self {
            TaggedInitiateKeyExchange::X25519(inner) => {
                let (answer, private_part) = inner.answer();
                (
                    TaggedAnswerKeyExchange::X25519(answer),
                    private_part.derive_material(),
                )
            }
        }
    }
}

impl TaggedAnswerKeyExchange {
    pub fn try_complete(&self, possible_part: &TaggedEncryptionKeyPair) -> Result<KeyMaterial, ()> {
        match self {
            TaggedAnswerKeyExchange::X25519(inner) => {
                let part: &EncryptionKeyPair<X25519> = possible_part.try_into()?;
                let completed = inner.complete(part.clone())?;
                Ok(X25519::finish_key_exchange(completed))
            }
        }
    }

    pub fn initiate_public_key(&self) -> PublicEncryptionKey {
        match self {
            TaggedAnswerKeyExchange::X25519(inner) => {
                PublicEncryptionKey::X25519(*inner.initiate_public_key())
            }
        }
    }

    pub fn answer_public_key(&self) -> PublicEncryptionKey {
        match self {
            TaggedAnswerKeyExchange::X25519(inner) => {
                PublicEncryptionKey::X25519(*inner.answer_public_key())
            }
        }
    }
}

impl TryFrom<TaggedAnswerKeyExchange> for AnswerKeyExchange<X25519> {
    type Error = ();

    fn try_from(value: TaggedAnswerKeyExchange) -> Result<Self, Self::Error> {
        let TaggedAnswerKeyExchange::X25519(value) = value;
        Ok(value)
    }
}
