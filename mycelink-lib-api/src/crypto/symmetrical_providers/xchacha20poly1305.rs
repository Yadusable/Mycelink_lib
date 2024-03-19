use crate::crypto::symmetrical_providers::SymmetricEncryptionProvider;
use chacha20poly1305::{AeadCore, AeadInPlace, KeyInit};
use std::ops::DerefMut;

use crate::crypto::key_material::KeyMaterial;
use rand::RngCore;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct XChaCha20Poly1305 {}
type Cipher = chacha20poly1305::XChaCha20Poly1305;

impl SymmetricEncryptionProvider for XChaCha20Poly1305 {
    type Key = [u8; 32];
    type Encrypted = Encrypted;

    fn generate_random_key() -> Self::Key {
        let mut key = [0; 32];
        rand::rngs::OsRng.fill_bytes(&mut key);
        key
    }

    fn generate_key_from_material(material: KeyMaterial) -> Self::Key {
        material.into()
    }

    fn encrypt(mut data: Box<[u8]>, key: &Self::Key) -> Self::Encrypted {
        let cipher = chacha20poly1305::XChaCha20Poly1305::new(key.into());
        let nonce = chacha20poly1305::XChaCha20Poly1305::generate_nonce(rand::rngs::OsRng);

        let tag = cipher
            .encrypt_in_place_detached(&nonce, &[], data.deref_mut())
            .unwrap();

        Encrypted {
            nonce: nonce.as_slice().try_into().unwrap(),
            mac: tag.as_slice().try_into().unwrap(),
            ciphertext: data,
        }
    }

    fn decrypt(mut data: Self::Encrypted, key: &Self::Key) -> Result<Box<[u8]>, ()> {
        let cipher = chacha20poly1305::XChaCha20Poly1305::new(key.into());

        match cipher.decrypt_in_place_detached(
            (&data.nonce).into(),
            &[],
            data.ciphertext.deref_mut(),
            (&data.mac).into(),
        ) {
            Ok(_) => Ok(data.ciphertext),
            Err(_) => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Encrypted {
    nonce: [u8; 24],
    mac: [u8; 16],
    ciphertext: Box<[u8]>,
}
