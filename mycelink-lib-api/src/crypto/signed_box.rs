use crate::crypto::hash_provider::sha512::Sha512;
use crate::crypto::hash_provider::HashProvider;
use crate::crypto::keypairs::SignatureKeyPair;
use crate::crypto::signature_providers::ed25519::Ed25519;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

pub type DefaultSignatureBox = SignedBox<Ed25519, Sha512>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedBox<P: SignatureProvider, H: HashProvider> {
    hasher: PhantomData<H>,
    public_key: P::PublicKey,
    signature: P::Signature,
    data: Box<[u8]>,
}

impl<P: SignatureProvider, H: HashProvider<Hash = P::Hash>> SignedBox<P, H> {
    pub fn sign<T: Serialize>(item: T, keys: &SignatureKeyPair<P>) -> SignedBox<P, H> {
        let mut data = Vec::new();
        ciborium::into_writer(&item, &mut data).unwrap();
        let hash = H::hash(data.as_ref());
        let signature = P::sign::<H>(&hash, keys);

        SignedBox {
            hasher: PhantomData,
            public_key: keys.public_key.clone(),
            signature,
            data: data.into(),
        }
    }

    pub fn verify<T: for<'d> Deserialize<'d>>(self) -> Result<T, SignedBoxError> {
        let hash = H::hash(self.data.as_ref());

        if P::verify::<H>(&self.signature, &hash, &self.public_key) {
            let item = ciborium::from_reader(self.data.as_ref())?;
            Ok(item)
        } else {
            Err(SignedBoxError::InvalidSignature)
        }
    }

    pub fn public_key(&self) -> &P::PublicKey {
        &self.public_key
    }
}

#[derive(Debug)]
pub enum SignedBoxError {
    InvalidSignature,
    CBOR(ciborium::de::Error<std::io::Error>),
}

impl From<ciborium::de::Error<std::io::Error>> for SignedBoxError {
    fn from(value: ciborium::de::Error<std::io::Error>) -> Self {
        SignedBoxError::CBOR(value)
    }
}

impl Error for SignedBoxError {}
impl Display for SignedBoxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SignedBoxError::InvalidSignature => write!(f, "Failed to verify signature"),
            SignedBoxError::CBOR(inner) => write!(f, "Failed to parse data with error {inner}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::hash_provider::sha512::Sha512;
    use crate::crypto::hash_provider::HashProvider;
    use crate::crypto::signature_providers::ed25519::Ed25519;
    use crate::crypto::signature_providers::SignatureProvider;
    use crate::crypto::signed_box::SignedBox;

    fn test_sign_verify_generic<P: SignatureProvider, H: HashProvider<Hash = P::Hash>>() {
        let data: Box<[u8]> = "Hello World".as_bytes().into();
        let signer = P::generate_signing_keypair();

        let signed_box = SignedBox::<P, H>::sign(data.as_ref(), &signer);

        assert_eq!(&signed_box.verify::<Box<[u8]>>().unwrap(), &data);

        let other_data = "Live Well".as_bytes();

        let mut manipulate_box = SignedBox::<P, H>::sign(data, &signer);
        manipulate_box.data = other_data.into();

        assert!(manipulate_box.verify::<Box<[u8]>>().is_err())
    }

    #[test]
    fn test_sign_verify_ed25519_sha512() {
        test_sign_verify_generic::<Ed25519, Sha512>()
    }
}
