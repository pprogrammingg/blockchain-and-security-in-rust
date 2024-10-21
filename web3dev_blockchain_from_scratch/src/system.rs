use std::collections::BTreeMap;
use std::ops::AddAssign;
use num::{CheckedAdd, CheckedSub, One, Zero};

/// Keep track of blockchain state

#[derive(Debug)]
pub struct Pallet<A, B, N> {
    block_number: B,
    // keep track of user vs number of tx
    nonce: BTreeMap<A, N>
}

impl<A, B, N> Pallet<A, B, N>
where
    A: Ord + Clone,
    B: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign,
    N: Zero + One + CheckedAdd + CheckedSub + Copy + AddAssign,
{
    pub fn new() -> Self {
        Pallet {
            block_number: B::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> B {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // On purpose crash if overflows
        self.block_number = self.block_number.checked_add(&B::one()).unwrap();
    }

    pub fn get_nonce(&self, who: A) -> N {
        *self.nonce.get(&who).unwrap_or(&N::zero())
    }
    pub fn inc_nonce(&mut self, who: A) {
        let nonce = self.get_nonce(who.clone());

        // crash on purpose if nonce value overflows
        self.nonce.insert(who, nonce.checked_add(&N::one()).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::system::Pallet;

    #[test]
    fn init_system() {
        // arrange
        let system = Pallet::new();

        // assert
        assert_eq!(system.block_number, 0);
    }

    #[test]
    fn inc_block_number() {
        // arrange
        let mut system = Pallet::new();

        // act
        system.inc_block_number();

        // assert
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn inc_nonce() {
        // arrange
        let mut system = Pallet::new();

        // act
        system.inc_nonce("alice".to_string());
        system.inc_nonce("alice".to_string());

        // assert
        assert_eq!(system.get_nonce("alice".to_string()), 2);
    }
}
