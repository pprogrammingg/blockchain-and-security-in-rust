use num::{CheckedAdd, One, Zero};
use std::collections::BTreeMap;
use std::ops::AddAssign;

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
    pub fn new() -> Self {
        Pallet {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // On purpose crash if overflows
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .unwrap();
    }

    pub fn get_nonce(&self, who: T::AccountId) -> T::Nonce {
        *self
            .nonce
            .get(&who)
            .unwrap_or(&T::Nonce::zero())
    }
    pub fn inc_nonce(&mut self, who: T::AccountId) {
        let nonce = self.get_nonce(who.clone());

        // crash on purpose if nonce value overflows
        self.nonce.insert(
            who,
            nonce
                .checked_add(&T::Nonce::one())
                .unwrap(),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::system::{Config, Pallet};

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    type TestPallet = Pallet<TestConfig>;

    #[test]
    fn init_system() {
        // arrange
        let system: TestPallet = Pallet::new();

        // assert
        assert_eq!(system.block_number, 0);
    }

    #[test]
    fn inc_block_number() {
        // arrange
        let mut system: TestPallet = Pallet::new();

        // act
        system.inc_block_number();

        // assert
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn inc_nonce() {
        // arrange
        let mut system: TestPallet = Pallet::new();

        // act
        system.inc_nonce("alice".to_string());
        system.inc_nonce("alice".to_string());

        // assert
        assert_eq!(system.get_nonce("alice".to_string()), 2);
    }
}
