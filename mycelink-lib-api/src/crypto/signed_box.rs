use crate::crypto::hash_provider::sha512::Sha512;
use crate::crypto::hash_provider::HashProvider;
use crate::crypto::keypairs::SignatureKeyPair;
use crate::crypto::signature_providers::ed25519::Ed25519;
use crate::crypto::signature_providers::SignatureProvider;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub type DefaultSignature = SignedBox<Ed25519, Sha512>;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedBox<P: SignatureProvider, H: HashProvider> {
    hasher: PhantomData<H>,
    public_key: P::PublicKey,
    signature: P::Signature,
    data: Box<[u8]>,
}

impl<P: SignatureProvider<Provider = P>, H: HashProvider<Hash = P::Hash>> SignedBox<P, H> {
    pub fn sign(data: Box<[u8]>, keys: &SignatureKeyPair<P>) -> SignedBox<P, H> {
        let hash = H::hash(data.as_ref());
        let signature = P::sign::<H>(&hash, keys);

        SignedBox {
            hasher: PhantomData,
            public_key: keys.public_key.clone(),
            signature,
            data,
        }
    }

    pub fn verify(self) -> Result<Box<[u8]>, ()> {
        let hash = H::hash(self.data.as_ref());

        if P::verify::<H>(&self.signature, &hash, &self.public_key) {
            Ok(self.data)
        } else {
            Err(())
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

    fn test_sign_verify_generic<
        P: SignatureProvider<Provider = P>,
        H: HashProvider<Hash = P::Hash>,
    >() {
        let data = "Hello World".as_bytes();
        let signer = P::generate_signing_keypair();

        let signed_box = SignedBox::<P, H>::sign(data.into(), &signer);

        assert_eq!(signed_box.verify().unwrap().as_ref(), data);

        let other_data = "Live Well".as_bytes();

        let mut manipulate_box = SignedBox::<P, H>::sign(data.into(), &signer);
        manipulate_box.data = other_data.into();

        assert!(manipulate_box.verify().is_err())
    }

    #[test]
    fn test_sign_verify_ed25519_sha512() {
        test_sign_verify_generic::<Ed25519, Sha512>()
    }
}
