
use web3dev_blockchain_from_scratch::{balances, system};
mod support;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;

    pub type BlockNumber = u64;

    pub type Nonce = u64;
}


impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;

}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<Runtime>,
    system: system::Pallet<Runtime>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new()
        }
    }
}

fn bob() -> String {
    "bob".to_string()
}

fn alice() -> String {
    "alice".to_string()
}

fn charlie() -> String {
    "charlie".to_string()
}
fn main() {
    let mut run_time = Runtime::new();
    let alice = alice();
    let bob = bob();
    let charlie = charlie();

    run_time.balances.set_balance(alice.clone(), 100);

    // init block number
    run_time.system.inc_block_number();
    assert_eq!(run_time.system.block_number(), 1);

    // init a tx on behalf of alice
    run_time.system.inc_nonce(alice.clone());
    let result = run_time.balances.transfer(alice.clone(), bob, 50)
        .map_err(|e| println!("Error: {:?}", e));

    run_time.system.inc_nonce(alice.clone());
    let result = run_time.balances.transfer(alice, charlie, 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:?}", run_time);
}

#[cfg(test)]
mod tests {
    use web3dev_blockchain_from_scratch::balances::Pallet;
    use super::*;


    #[test]
    fn init_balance() {
        // arrange
        let mut balances: Pallet<Runtime> = Pallet::new();

        // assert
        assert_eq!(balances.balance(bob()), 0);

        // act
        balances.set_balance(bob(), 100);

        // assert
        assert_eq!(balances.balance(bob()), 100);
        assert_eq!(balances.balance(alice()), 0);
    }

    #[test]
    fn transfer_balance() {
        // arrange
        let mut balances: Pallet<Runtime> = Pallet::new();
        assert_eq!(balances.balance(bob()), 0);

        // act
        balances.set_balance(bob(), 100);
        balances.set_balance(alice(), 50);

        // Bob transfers 50 to Alice
        balances.transfer(bob(), alice(), 50).unwrap();

        // assert
        assert_eq!(balances.balance(bob()), 50);
        assert_eq!(balances.balance(alice()), 100);
    }

    #[test]
    fn transfer_balance_insufficient() {
        // arrange
        let mut balances: Pallet<Runtime>  = Pallet::new();
        assert_eq!(balances.balance(bob()), 0);

        // act
        balances.set_balance(bob(), 100);
        balances.set_balance(alice(), 50);

        // Bob transfers 50 to Alice
        let transfer_result = balances.transfer(bob(), alice(), 110);

        // assert
        assert_eq!(transfer_result, Err("Insufficient balance"));
        assert_eq!(balances.balance(bob()), 100);
        assert_eq!(balances.balance(alice()), 50);
    }

    #[test]
    fn transfer_balance_overflow() {
        // arrange
        let mut balances: Pallet<Runtime>  = Pallet::new();
        assert_eq!(balances.balance(bob()), 0);

        // act
        balances.set_balance(bob(), 100);
        balances.set_balance(alice(), u128::MAX);

        // Bob transfers 50 to Alice
        let transfer_result = balances.transfer(bob(), alice(), 50);

        // assert
        assert_eq!(transfer_result, Err("Overflow when adding balance"));
        assert_eq!(balances.balance(bob()), 100);
        assert_eq!(balances.balance(alice()), u128::MAX);
    }
}