use web3dev_blockchain_from_scratch::balances::Pallet;

fn main() {
    println!("Hello, world!");

    let mut pallet = Pallet::new();
}


#[test]
fn init_balance() {
    // arrange
    let mut balances = Pallet::new();

    // assert
    assert_eq!(balances.balance("bob".to_string()), 0);

    // act
    balances.set_balance("bob".to_string(), 100);

    // assert
    assert_eq!(balances.balance("bob".to_string()), 100);
    assert_eq!(balances.balance("alice".to_string()), 0);
}