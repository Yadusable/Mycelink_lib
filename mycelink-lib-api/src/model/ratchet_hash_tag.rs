use crate::crypto::hash_provider::{Hash, HashProvider};
use crate::crypto::rachet::Ratchet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum RatchetHashTag {
    Sha512,
}

impl RatchetHashTag {
    pub fn create_ratchet() -> Box<Ratchet<dyn HashProvider<Hash = dyn Hash>>> {
        todo!()
    }
}
