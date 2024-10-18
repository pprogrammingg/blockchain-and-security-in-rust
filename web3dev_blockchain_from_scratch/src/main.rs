use web3dev_blockchain_from_scratch::balances::Pallet;
use web3dev_blockchain_from_scratch::{balances, system};

pub struct RunTime {
    balances: balances::Pallet,
    system: system::Pallet,
}

impl RunTime {
    pub fn new() -> Self {
        Self {
            balances: balances::Pallet::new(),
            system: system::Pallet::new()
        }
    }
}

fn main() {
    let run_time = RunTime::new();
    println!("Hello, world!");
}


#[test]
fn init_balance() {
    // arrange
    let mut balances = Pallet::new();

    // assert
    assert_eq!(balances.balance("bob"), 0);

    // act
    balances.set_balance("bob", 100);

    // assert
    assert_eq!(balances.balance("bob"), 100);
    assert_eq!(balances.balance("alice"), 0);
}

#[test]
fn transfer_balance() {
    // arrange
    let mut balances = Pallet::new();
    assert_eq!(balances.balance("bob"), 0);

    // act
    balances.set_balance("bob", 100);
    balances.set_balance("alice", 50);

    // Bob transfers 50 to Alice
    balances.transfer("bob", "alice", 50).unwrap();

    // assert
    assert_eq!(balances.balance("bob"), 50);
    assert_eq!(balances.balance("alice"), 100);
}

#[test]
fn transfer_balance_insufficient() {
    // arrange
    let mut balances = Pallet::new();
    assert_eq!(balances.balance("bob"), 0);

    // act
    balances.set_balance("bob", 100);
    balances.set_balance("alice", 50);

    // Bob transfers 50 to Alice
    let transfer_result = balances.transfer("bob", "alice", 110);

    // assert
    assert_eq!(transfer_result, Err("Insufficient balance"));
    assert_eq!(balances.balance("bob"), 100);
    assert_eq!(balances.balance("alice"), 50);
}

#[test]
fn transfer_balance_overflow() {
    // arrange
    let mut balances = Pallet::new();
    assert_eq!(balances.balance("bob"), 0);

    // act
    balances.set_balance("bob", 100);
    balances.set_balance("alice", u128::MAX);

    // Bob transfers 50 to Alice
    let transfer_result = balances.transfer("bob", "alice", 50);

    // assert
    assert_eq!(transfer_result, Err("Overflow when adding balance"));
    assert_eq!(balances.balance("bob"), 100);
    assert_eq!(balances.balance("alice"), u128::MAX);
}