use std::collections::BTreeMap;

/// Keep track of blockchain state

pub struct Pallet {
    block_number: u32,
    // keep track of user vs number of tx
    nonce: BTreeMap<String, u32>
}

impl Pallet {
    pub fn new() -> Pallet {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        // On purpose crash if overflows
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn get_nonce(&self, who: &str) -> u32 {
        *self.nonce.get(&who.to_string()).unwrap_or(&0)
    }
    pub fn inc_nonce(&mut self, who: &str) {
        let nonce = self.get_nonce(who);

        // crash on purpose if nonce value overflows
        self.nonce.insert(who.into(), nonce.checked_add(1).unwrap());
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
        system.inc_nonce("alice");
        system.inc_nonce("alice");

        // assert
        assert_eq!(system.get_nonce("alice"), 2);
    }
}
