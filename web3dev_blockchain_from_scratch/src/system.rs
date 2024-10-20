use std::collections::BTreeMap;

/// Keep track of blockchain state

type AccountId = String;
type BlockNumer = u64;
type Nonce = u64;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumer,
    // keep track of user vs number of tx
    nonce: BTreeMap<AccountId, Nonce>
}

impl Pallet {
    pub fn new() -> Pallet {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> BlockNumer {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // On purpose crash if overflows
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn get_nonce(&self, who: AccountId) -> Nonce {
        *self.nonce.get(&who).unwrap_or(&0)
    }
    pub fn inc_nonce(&mut self, who: AccountId) {
        let nonce = self.get_nonce(who.clone());

        // crash on purpose if nonce value overflows
        self.nonce.insert(who, nonce.checked_add(1).unwrap());
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
