use std::collections::BTreeMap;

pub struct Pallet {
    // key/value structure, key is
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    /// set the balance of who
    pub fn set_balance(&mut self, who: String, amount: u128) {
        self.balances.insert(who.clone(), amount);

    }

    /// get the balance of who
    pub fn balance(&self, who: String) -> u128 {
        *self.balances.get(&who).unwrap_or(&0)
    }
}
