use std::collections::BTreeMap;

/// Keep track of blockchain state

pub struct Pallet {
    block_number: u32,
    // keep track of user vs number of tx
    none: BTreeMap<String, u32>
}

impl Pallet {
    pub fn new() -> Pallet {
        Pallet {
            block_number: 0,
            none: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self, value: u32) {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}
