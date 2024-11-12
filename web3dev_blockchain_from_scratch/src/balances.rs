use std::collections::BTreeMap;
use num::{CheckedAdd, CheckedSub, Zero};

#[derive(Debug)]
pub struct Pallet<A,B> {
    balances: BTreeMap<A, B>,
}

impl<A, B> Pallet<A, B>
where
    A: Ord + Clone,
    B: Zero + CheckedSub + CheckedAdd + Copy
{

    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    /// set the balance of who
    pub fn set_balance(&mut self, who: A, amount: B) {
        self.balances.insert(who, amount);

    }

    /// get the balance of who
    pub fn balance(&self, who: A) -> B {
        *self.balances.get(&who).unwrap_or(&B::zero())
    }

    pub fn transfer(&mut self, caller: A, to: A, amount: B) -> Result<(), &'static str>
    {
        let caller_balance = self.balance(caller.clone());
        let to_balance = self.balance(to.clone());

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(&amount)
            .ok_or("Overflow when adding balance")?;

        self.set_balance(caller, new_caller_balance);
        self.set_balance(to, new_to_balance);

        Ok(())
    }
}
