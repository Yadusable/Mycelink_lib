use crate::crypto::symmetrical_providers::SymmetricEncryptionProvider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretBox<P: SymmetricEncryptionProvider>(P::Encrypted);

impl<P: SymmetricEncryptionProvider> SecretBox<P> {
    pub fn create(plaintext: Box<[u8]>, key: &P::Key) -> SecretBox<P> {
        let payload = P::encrypt(plaintext, key);
        SecretBox(payload)
    }

    pub fn open(self, key: &P::Key) -> Result<Box<[u8]>, DecryptionFailed> {
        P::decrypt(self.0, key)
    }
}

pub type DecryptionFailed = ();
