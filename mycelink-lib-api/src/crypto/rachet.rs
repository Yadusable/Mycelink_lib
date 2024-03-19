use crate::crypto::hash_provider::HashProvider;
use crate::crypto::key_material::KeyMaterial;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ratchet<H: HashProvider + ?Sized> {
    hasher: PhantomData<H>,
    current_iteration: u32,
    current_state: KeyMaterial,
    purpose: RatchetPurpose,
}

impl<H: HashProvider> Clone for Ratchet<H> {
    fn clone(&self) -> Self {
        Self {
            hasher: PhantomData,
            current_iteration: self.current_iteration,
            current_state: self.current_state.clone(),
            purpose: self.purpose,
        }
    }
}

impl<H: HashProvider> Ratchet<H> {
    pub fn new(key_material: KeyMaterial, purpose: RatchetPurpose) -> Self {
        Self {
            hasher: PhantomData,
            current_iteration: 0,
            current_state: key_material,
            purpose,
        }
    }

    pub fn advance(&mut self) {
        self.current_iteration += 1;
        self.current_state = H::derive_key(&self.current_state, "Mycelink ratchet advance")
    }

    pub fn current_key(&self) -> KeyMaterial {
        H::derive_key(&self.current_state, self.purpose.as_str())
    }

    pub fn get_key(&self, iteration: u32) -> Result<KeyMaterial, ()> {
        if iteration < self.current_iteration {
            return Err(());
        }

        let mut ratchet: Cow<Ratchet<H>> = Cow::Borrowed(self);
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
