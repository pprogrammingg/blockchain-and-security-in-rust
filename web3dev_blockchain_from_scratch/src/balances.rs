use std::collections::BTreeMap;
use std::ops::AddAssign;
use num::{CheckedAdd, CheckedSub, One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type Balance: Zero + One + CheckedAdd + CheckedSub + AddAssign + Copy;
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T>
{

    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    /// set the balance of who
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.balances.insert(who, amount);

    }

    /// get the balance of who
    pub fn balance(&self, who: T::AccountId) -> T::Balance {
        *self.balances.get(&who).unwrap_or(&T::Balance::zero())
    }

    pub fn transfer(&mut self, caller: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str>
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
