use std::{
    collections::BTreeMap,
    ops::AddAssign,
};

use num::{
    CheckedAdd,
    One,
    Zero,
};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + CheckedAdd + AddAssign + Copy;
    type Nonce: Zero + One + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    // keep track of user vs number of tx
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self
            .nonce
            .get(&who)
            .unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce + T::Nonce::one();
        self.nonce
            .insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();
        system.inc_block_number();
        system.inc_nonce(&"alice".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(
            system
                .nonce
                .get(&"alice".to_string()),
            Some(&1)
        );
        assert_eq!(
            system
                .nonce
                .get(&"bob".to_string()),
            None
        );
    }
}
