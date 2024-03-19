use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::key_material::KeyMaterial;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ratchet {
    kdf: KdfProviderTag,
    current_iteration: u32,
    current_state: KeyMaterial,
    purpose: RatchetPurpose,
}

impl Ratchet {
    pub fn new(key_material: KeyMaterial, purpose: RatchetPurpose, kdf: KdfProviderTag) -> Self {
        Self {
            kdf,
            current_iteration: 0,
            current_state: key_material,
            purpose,
        }
    }

    pub fn advance(&mut self) {
        self.current_iteration += 1;
        self.current_state = self
            .kdf
            .as_provider()
            .derive_key(&self.current_state, "Mycelink ratchet advance")
    }

    pub fn current_key(&self) -> KeyMaterial {
        self.kdf
            .as_provider()
            .derive_key(&self.current_state, self.purpose.as_str())
    }

    pub fn get_key(&self, iteration: u32) -> Result<KeyMaterial, ()> {
        if iteration < self.current_iteration {
            return Err(());
        }

        let mut ratchet = Cow::Borrowed(self);
        if ratchet.current_iteration < iteration {
            let mut cloned = self.clone();
            while cloned.current_iteration < iteration {
                cloned.advance()
            }
            ratchet = Cow::Owned(cloned)
        }

        Ok(ratchet.current_key())
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum RatchetPurpose {
    MycelinkChannel,
}

impl RatchetPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            RatchetPurpose::MycelinkChannel => "MycelinkChannel",
        }
    }
}
