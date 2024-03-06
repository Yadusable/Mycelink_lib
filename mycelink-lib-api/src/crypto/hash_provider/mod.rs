use serde::{Deserialize, Serialize};

pub mod blake3;

pub trait HashProvider {
    type Hash: Clone + Serialize + for<'de> Deserialize<'de> + AsRef<[u8]>;

    fn hash(data: &[u8]) -> Self::Hash;
}
