use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;
use crate::crypto::key_material::KeyMaterial;
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

impl<P: AsymmetricEncryptionProvider<Provider = P>> AnswerKeyExchange<P> {
    pub fn complete(
        self,
        private_part: PrivateInitiateKeyExchangePart<P>,
    ) -> CompletedKeyExchange<P> {
        CompletedKeyExchange {
            public_component: self.answer_public_key,
            private_component: private_part.own_private_key,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedKeyExchange<P: AsymmetricEncryptionProvider> {
    public_component: P::PublicKey,
    private_component: P::PrivateKey,
}

impl<P: AsymmetricEncryptionProvider<Provider = P>> CompletedKeyExchange<P> {
    pub(super) fn into_components(self) -> (P::PublicKey, P::PrivateKey) {
        (self.public_component, self.private_component)
    }

    pub fn derive_material(self) -> KeyMaterial {
        P::finish_key_exchange(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::key_exchange::InitiateKeyExchange;
    use crate::crypto::key_exchange_providers::x25519::X25519;
    use crate::crypto::key_exchange_providers::AsymmetricEncryptionProvider;

    fn test_full_exchange_generic<P: AsymmetricEncryptionProvider<Provider = P>>() {
        let (initial, p1) = InitiateKeyExchange::<P>::new();

        let (answer, c2) = initial.answer();

        let c1 = answer.complete(p1);

        let s1 = c1.derive_material();
        let s2 = c2.derive_material();

        assert_eq!(s1, s2)
    }

    #[test]
    fn test_full_exchange_x25519() {
        test_full_exchange_generic::<X25519>()
    }
}
