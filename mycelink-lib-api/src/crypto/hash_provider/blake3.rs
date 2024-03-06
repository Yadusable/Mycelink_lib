use crate::crypto::hash_provider::HashProvider;
use crate::crypto::types::ByteArray64;

pub struct Blake3 {}
impl HashProvider for Blake3 {
    type Hash = ByteArray64;

    fn hash(data: &[u8]) -> Self::Hash {
        todo!()
    }
}
