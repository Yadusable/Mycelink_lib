use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateInitiateKeyExchangePart<P: AsymmetricEncryptionProvider> {
    #[serde(with = "hex::serde")]
    own_public_key: P::PublicKey,
    #[serde(with = "hex::serde")]
    own_private_key: P::PrivateKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitiateKeyExchange<P: AsymmetricEncryptionProvider> {
    #[serde(with = "hex::serde")]
    public_key: P::PublicKey,
}

impl<P: AsymmetricEncryptionProvider<Provider = P>> InitiateKeyExchange<P> {
    pub fn new() -> (Self, PrivateInitiateKeyExchangePart<P>) {
        let keypair = P::generate_encryption_keypair();

        let initiate_part = InitiateKeyExchange {
            public_key: keypair.public_key.clone(),
        };
        let private_part = PrivateInitiateKeyExchangePart {
            own_public_key: keypair.public_key,
            own_private_key: keypair.private_key,
        };

        (initiate_part, private_part)
    }

    pub fn answer(self) -> (AnswerKeyExchange<P>, CompletedKeyExchange<P>) {
        let keypair = P::generate_encryption_keypair();

        let answer = AnswerKeyExchange {
            initiate_public_key: self.public_key.clone(),
            answer_public_key: keypair.public_key,
        };

        let completed = CompletedKeyExchange {
            public_component: self.public_key,
            private_component: keypair.private_key,
        };

        (answer, completed)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnswerKeyExchange<P: AsymmetricEncryptionProvider> {
    #[serde(with = "hex::serde")]
    initiate_public_key: P::PublicKey,
    #[serde(with = "hex::serde")]
    answer_public_key: P::PublicKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedKeyExchange<P: AsymmetricEncryptionProvider> {
    public_component: P::PublicKey,
    private_component: P::PrivateKey,
}

impl<P: AsymmetricEncryptionProvider> CompletedKeyExchange<P> {
    pub(super) fn into_components(self) -> (P::PublicKey, P::PrivateKey) {
        (self.public_component, self.private_component)
    }
}
