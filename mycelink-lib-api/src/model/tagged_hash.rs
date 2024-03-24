use crate::crypto::hash_provider::blake3::Blake3;
use crate::crypto::hash_provider::HashProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TaggedHash {
    Blake3(<Blake3 as HashProvider>::Hash),
}
