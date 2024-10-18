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
    pub fn set_balance(&mut self, who: &str, amount: u128) {
        self.balances.insert(who.to_string(), amount);

    }

    /// get the balance of who
    pub fn balance(&self, who: &str) -> u128 {
        *self.balances.get(&who.to_string()).unwrap_or(&0)
    }

    pub fn transfer(&mut self, caller: &str, to: &str, amount: u128) -> Result<(), &'static str>
    {
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}
