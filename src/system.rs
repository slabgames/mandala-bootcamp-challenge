use std::hash::Hash;

pub trait SystemConfig {
    // Define the account identifier type
    type AccountId: Eq + Hash + Clone;
}
