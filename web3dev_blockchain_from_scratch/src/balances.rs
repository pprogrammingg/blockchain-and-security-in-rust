use std::collections::BTreeMap;

type AccountId = String;
type Balance = u128;

#[derive(Debug)]
pub struct Pallet {
    // key/value structure, key is
    balances: BTreeMap<AccountId, Balance>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    /// set the balance of who
    pub fn set_balance(&mut self, who: AccountId, amount: Balance) {
        self.balances.insert(who, amount);

    }

    /// get the balance of who
    pub fn balance(&self, who: AccountId) -> Balance {
        *self.balances.get(&who).unwrap_or(&0)
    }

    pub fn transfer(&mut self, caller: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str>
    {
        let caller_balance = self.balance(caller.clone());
        let to_balance = self.balance(to.clone());

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding balance")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}
