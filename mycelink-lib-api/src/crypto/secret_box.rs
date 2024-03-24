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

#[cfg(test)]
mod tests {
    use crate::crypto::secret_box::SecretBox;
    use crate::crypto::symmetrical_providers::xchacha20poly1305::XChaCha20Poly1305;
    use crate::crypto::symmetrical_providers::SymmetricEncryptionProvider;

    fn test_encrypt_decrypt_generic<P: SymmetricEncryptionProvider>() {
        let data = "Hello World".as_bytes();
        let key = P::generate_random_key();

        let encrypted = SecretBox::<P>::create(data.into(), &key);

        let decrypted = encrypted.open(&key).unwrap();

        assert_eq!(data, decrypted.as_ref())
    }

    #[test]
    fn test_encrypt_decrypt_xchacha20() {
        test_encrypt_decrypt_generic::<XChaCha20Poly1305>()
    }
}
