use std::{
    collections::BTreeMap,
    ops::AddAssign,
};

use num::{
    CheckedAdd,
    CheckedSub,
    One,
    Zero,
};

pub trait Config: crate::system::Config {
    type Balance: Zero + One + CheckedAdd + CheckedSub + AddAssign + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> Result<(), &'static str> {
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

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// set the balance of who
    pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) {
        self.balances
            .insert(who, amount);
    }

    /// get the balance of who
    pub fn balance(&self, who: T::AccountId) -> T::Balance {
        *self
            .balances
            .get(&who)
            .unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    struct TestConfig;

    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Balance = u128;
    }

    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(balances.balance("alice".to_string()), 0);
        balances.set_balance("alice".to_string(), 100);
        assert_eq!(balances.balance("alice".to_string()), 100);
        assert_eq!(balances.balance("bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = super::Pallet::<TestConfig>::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Insufficient balance")
        );

        balances.set_balance("alice".to_string(), 100);
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Ok(())
        );
        assert_eq!(balances.balance("alice".to_string()), 49);
        assert_eq!(balances.balance("bob".to_string()), 51);

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 51),
            Err("Insufficient balance")
        );
    }
}
