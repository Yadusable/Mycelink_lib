use crate::crypto::secret_box::SecretBox;
use crate::crypto::symmetrical_providers::xchacha20poly1305::XChaCha20Poly1305;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug)]
pub enum TaggedSecretBox {
    XChaCha20(SecretBox<XChaCha20Poly1305>),
}

impl From<TaggedSecretBox> for Box<[u8]> {
    fn from(value: TaggedSecretBox) -> Self {
        let mut buffer = Vec::new();
        ciborium::into_writer(&value, &mut buffer).unwrap();
        buffer.into()
    }
}

impl TryFrom<Box<[u8]>> for TaggedSecretBox {
    type Error = ciborium::de::Error<std::io::Error>;

    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        ciborium::from_reader(value.deref())
    }
}
