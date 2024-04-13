use crate::crypto::key_material::KeyMaterial;
use crate::crypto::secret_box::{SecretBox, SecretBoxError};
use crate::crypto::symmetrical_providers::xchacha20poly1305::XChaCha20Poly1305;
use crate::crypto::symmetrical_providers::SymmetricEncryptionProvider;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaggedSecretBox {
    XChaCha20(SecretBox<XChaCha20Poly1305>),
}

impl TaggedSecretBox {
    pub fn try_decrypt<T: for<'d> Deserialize<'d>>(
        self,
        key_material: KeyMaterial,
    ) -> Result<T, SecretBoxError> {
        match self {
            TaggedSecretBox::XChaCha20(inner) => {
                let key = XChaCha20Poly1305::generate_key_from_material(key_material);
                inner.open(&key)
            }
        }
    }
}

impl From<SecretBox<XChaCha20Poly1305>> for TaggedSecretBox {
    fn from(value: SecretBox<XChaCha20Poly1305>) -> Self {
        Self::XChaCha20(value)
    }
}
