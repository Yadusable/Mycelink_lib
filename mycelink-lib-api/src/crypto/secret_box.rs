use crate::crypto::symmetrical_providers::xchacha20poly1305::XChaCha20Poly1305;
use crate::crypto::symmetrical_providers::SymmetricEncryptionProvider;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};

pub type DefaultSecretBox = SecretBox<XChaCha20Poly1305>;

#[derive(Serialize, Deserialize, Debug)]
pub struct SecretBox<P: SymmetricEncryptionProvider>(P::Encrypted);

impl<P: SymmetricEncryptionProvider> SecretBox<P> {
    pub fn create<T: Serialize>(item: T, key: &P::Key) -> SecretBox<P> {
        let mut plaintext = Vec::new();
        ciborium::into_writer(&item, &mut plaintext).unwrap();
        let payload = P::encrypt(plaintext.into(), key);
        SecretBox(payload)
    }

    pub fn open<T: for<'d> Deserialize<'d>>(self, key: &P::Key) -> Result<T, SecretBoxError> {
        let plaintext = P::decrypt(self.0, key).map_err(|_| SecretBoxError::DecryptionFailed)?;
        Ok(ciborium::from_reader(plaintext.as_ref())?)
    }
}

#[derive(Debug)]
pub enum SecretBoxError {
    DecryptionFailed,
    CBOR(ciborium::de::Error<std::io::Error>),
}

impl From<ciborium::de::Error<std::io::Error>> for SecretBoxError {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        SecretBoxError::CBOR(value)
    }
}

impl Error for SecretBoxError {}
impl Display for SecretBoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretBoxError::DecryptionFailed => write!(f, "Failed to decrypt content"),
            SecretBoxError::CBOR(inner) => {
                write!(f, "Failed to parse decrypted content with err {inner}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::secret_box::SecretBox;
    use crate::crypto::symmetrical_providers::xchacha20poly1305::XChaCha20Poly1305;
    use crate::crypto::symmetrical_providers::SymmetricEncryptionProvider;

    fn test_encrypt_decrypt_generic<P: SymmetricEncryptionProvider>() {
        let data: Box<[u8]> = "Hello World".as_bytes().into();
        let key = P::generate_random_key();

        let encrypted = SecretBox::<P>::create(&data, &key);

        let decrypted: Box<[u8]> = encrypted.open(&key).unwrap();

        assert_eq!(data, decrypted)
    }

    #[test]
    fn test_encrypt_decrypt_xchacha20() {
        test_encrypt_decrypt_generic::<XChaCha20Poly1305>()
    }
}
