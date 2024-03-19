use crate::crypto::hash_provider::HashProvider;
use crate::crypto::key_material::KeyMaterial;
use crate::crypto::types::byte_array_64::ByteArray64;
use hkdf::Hkdf;
use sha2::Digest;

#[derive(Debug)]
pub struct Sha512 {}
impl HashProvider for Sha512 {
    type Hash = ByteArray64;

    fn hash(data: &[u8]) -> Self::Hash {
        let mut digest = sha2::Sha512::new();
        digest.update(data);
        let hash = digest.finalize();
        ByteArray64(hash.as_slice().try_into().unwrap())
    }

    fn derive_key(key_material: &KeyMaterial, purpose: &'static str) -> KeyMaterial {
        let hkdf = Hkdf::<sha2::Sha512>::new(None, key_material.as_ref());
        let mut out: Self::Hash = ByteArray64([0; 64]);
        hkdf.expand(purpose.as_bytes(), &mut out.as_mut()).unwrap();
        (*out).into()
    }
}
