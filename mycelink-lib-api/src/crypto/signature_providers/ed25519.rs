use crate::crypto::hash_provider::HashProvider;
use crate::crypto::keypairs::SignatureKeyPair;
use crate::crypto::signature_providers::ed25519::dummy::DummyDigest;
use crate::crypto::signature_providers::SignatureProvider;
use crate::crypto::types::byte_array_64::ByteArray64;
use ed25519_dalek::{Signature, VerifyingKey};

pub struct Ed25519 {}
impl SignatureProvider for Ed25519 {
    type Provider = Ed25519;
    type PublicKey = [u8; 32];
    type PrivateKey = [u8; 32];
    type Signature = ByteArray64;
    type Hash = ByteArray64;

    fn generate_signing_keypair() -> SignatureKeyPair<Self::Provider> {
        let private_key = ed25519_dalek::SigningKey::generate(&mut rand::rngs::OsRng);
        let public_key = private_key.verifying_key();

        SignatureKeyPair {
            private_key: private_key.to_bytes(),
            public_key: public_key.to_bytes(),
        }
    }

    fn sign<H: HashProvider<Hash = ByteArray64>>(
        hash: &H::Hash,
        keypair: &SignatureKeyPair<Ed25519>,
    ) -> Self::Signature {
        let private = keypair.private_key;
        let private = ed25519_dalek::SigningKey::from_bytes(&private);

        let digest = DummyDigest(hash.0);

        let signature = private.sign_prehashed(digest, None).unwrap();

        ByteArray64(signature.to_bytes())
    }

    fn verify<H: HashProvider<Hash = Self::Hash>>(
        signature: &Self::Signature,
        hash: &H::Hash,
        public_key: &Self::PublicKey,
    ) -> bool {
        let public_key = match VerifyingKey::from_bytes(public_key) {
            Err(err) => {
                log::warn!(
                    "Failed parsing public_key {} with err {err}",
                    hex::encode(public_key)
                );
                return false;
            }
            Ok(v) => v,
        };

        let signature = Signature::from_bytes(&signature.0);

        if let Err(err) = public_key.verify_prehashed(DummyDigest(hash.0), None, &signature) {
            log::warn!("Failed to verify signature with err {err}");
            false
        } else {
            true
        }
    }
}

mod dummy {
    use chacha20poly1305::consts::U64;
    use ed25519_dalek::ed25519::signature::digest::{
        FixedOutput, FixedOutputReset, Output, OutputSizeUser, Reset, Update,
    };
    use ed25519_dalek::Digest;

    // Used because ed25519 api requires a digest object while only using the data returned from finalize...
    pub(super) struct DummyDigest(pub [u8; 64]);

    impl OutputSizeUser for DummyDigest {
        type OutputSize = U64;
    }

    impl Digest for DummyDigest {
        fn new() -> Self {
            panic!("new shouldn't be called on dummy digest")
        }

        fn new_with_prefix(_data: impl AsRef<[u8]>) -> Self {
            panic!("new_with_prefix shouldn't be called on dummy digest")
        }

        fn update(&mut self, _data: impl AsRef<[u8]>) {
            panic!("update shouldn't be called on dummy digest")
        }

        fn chain_update(self, _data: impl AsRef<[u8]>) -> Self {
            panic!("chain_update shouldn't be called on dummy digest")
        }

        fn finalize(self) -> Output<Self> {
            self.0.into()
        }

        fn finalize_into(self, _out: &mut Output<Self>) {
            panic!("finalize_into shouldn't be called on dummy digest")
        }

        fn finalize_reset(&mut self) -> Output<Self>
        where
            Self: FixedOutputReset,
        {
            panic!("finalize_reset shouldn't be called on dummy digest")
        }

        fn finalize_into_reset(&mut self, _out: &mut Output<Self>)
        where
            Self: FixedOutputReset,
        {
            panic!("finalize_into_reset shouldn't be called on dummy digest")
        }

        fn reset(&mut self)
        where
            Self: Reset,
        {
            panic!("reset shouldn't be called on dummy digest")
        }

        fn output_size() -> usize {
            panic!("output_size shouldn't be called on dummy digest")
        }

        fn digest(_data: impl AsRef<[u8]>) -> Output<Self> {
            panic!("digest shouldn't be called on dummy digest")
        }
    }

    impl Reset for DummyDigest {
        fn reset(&mut self) {
            panic!("reset shouldn't be called on dummy digest")
        }
    }

    impl FixedOutput for DummyDigest {
        fn finalize_into(self, _out: &mut Output<Self>) {
            panic!("finalize_into shouldn't be called on dummy digest")
        }
    }

    impl Update for DummyDigest {
        fn update(&mut self, _data: &[u8]) {
            panic!("update shouldn't be called on dummy digest")
        }
    }

    impl FixedOutputReset for DummyDigest {
        fn finalize_into_reset(&mut self, _out: &mut Output<Self>) {
            panic!("finalize_into_reset shouldn't be called on dummy digest")
        }
    }
}
