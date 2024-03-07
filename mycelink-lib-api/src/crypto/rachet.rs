use crate::crypto::hash_provider::HashProvider;

pub struct Rachet<H: HashProvider> {
    current_iteration: u32,
    current_state: H::Hash,
    purpose: &'static str,
}
