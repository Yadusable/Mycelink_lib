use crate::crypto::key_exchange::{AnswerKeyExchange, InitiateKeyExchange};
use crate::crypto::key_exchange_providers::x25519::X25519;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TaggedInitiateKeyExchange {
    X25519(InitiateKeyExchange<X25519>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TaggedAnswerKeyExchange {
    X25519(AnswerKeyExchange<X25519>),
}
