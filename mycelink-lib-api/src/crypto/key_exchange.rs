use crate::crypto::key_exchange_providers::{x25519, AsymmetricEncryptionProvider};
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::keypairs::EncryptionKeyPair;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InitiateKeyExchange<P: AsymmetricEncryptionProvider> {
    #[serde(with = "hex::serde")]
    public_key: P::PublicKey,
}

impl<P: AsymmetricEncryptionProvider> InitiateKeyExchange<P> {
    pub fn new() -> (Self, EncryptionKeyPair<P>) {
        let keypair = P::generate_encryption_keypair();

        let initiate_part = InitiateKeyExchange {
            public_key: keypair.public_key.clone(),
        };

        (initiate_part, keypair)
    }

    pub fn answer(&self) -> (AnswerKeyExchange<P>, CompletedKeyExchange<P>) {
        let keypair = P::generate_encryption_keypair();

        let answer = AnswerKeyExchange {
            initiate_public_key: self.public_key.clone(),
            answer_public_key: keypair.public_key,
        };

        let completed = CompletedKeyExchange {
            public_component: self.public_key.clone(),
            private_component: keypair.private_key,
        };

        (answer, completed)
    }

    pub fn into_public_key(self) -> P::PublicKey {
        self.public_key
    }
}

impl From<EncryptionKeyPair<x25519::X25519>> for InitiateKeyExchange<x25519::X25519> {
    fn from(value: EncryptionKeyPair<x25519::X25519>) -> Self {
        Self {
            public_key: value.public_key,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnswerKeyExchange<P: AsymmetricEncryptionProvider> {
    #[serde(with = "hex::serde")]
    initiate_public_key: P::PublicKey,
    #[serde(with = "hex::serde")]
    answer_public_key: P::PublicKey,
}

impl<P: AsymmetricEncryptionProvider> AnswerKeyExchange<P> {
    pub fn complete(
        &self,
        private_part: EncryptionKeyPair<P>,
    ) -> Result<CompletedKeyExchange<P>, ()> {
        let public_component = if private_part.public_key == self.initiate_public_key {
            &self.answer_public_key
        } else if private_part.public_key == self.answer_public_key {
            &self.initiate_public_key
        } else {
            return Err(());
        };

        Ok(CompletedKeyExchange {
            public_component: public_component.clone(),
            private_component: private_part.private_key,
        })
    }

    pub fn initiate_public_key(&self) -> &P::PublicKey {
        &self.initiate_public_key
    }
    pub fn answer_public_key(&self) -> &P::PublicKey {
        &self.answer_public_key
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedKeyExchange<P: AsymmetricEncryptionProvider + ?Sized> {
    public_component: P::PublicKey,
    private_component: P::PrivateKey,
}

impl<P: AsymmetricEncryptionProvider> CompletedKeyExchange<P> {
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

    fn test_full_exchange_generic<P: AsymmetricEncryptionProvider>() {
        let (initial, p1) = InitiateKeyExchange::<P>::new();

        let (answer, c2) = initial.answer();

        let c1 = answer.complete(p1).unwrap();

        let s1 = c1.derive_material();
        let s2 = c2.derive_material();

        assert_eq!(s1, s2)
    }

    #[test]
    fn test_full_exchange_x25519() {
        test_full_exchange_generic::<X25519>()
    }
}
