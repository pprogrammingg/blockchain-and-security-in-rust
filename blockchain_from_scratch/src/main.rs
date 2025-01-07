use std::fmt::Debug;

use web3dev_blockchain_from_scratch::{
    balances,
    proof_of_existence,
    support,
    support::Dispatch,
    system,
};

mod types {
    use web3dev_blockchain_from_scratch::support;

    pub type AccountId = String;
    pub type Balance = u128;

    pub type BlockNumber = u64;

    pub type Nonce = u64;

    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;

    pub type Header = support::Header<BlockNumber>;

    pub type Block = support::Block<Header, Extrinsic>;

    pub type Content = &'static str;
}
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
    proof_of_existence: proof_of_existence::Pallet<Runtime>,
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

    run_time
        .balances
        .set_balance(alice.clone(), 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
        ],
    };

    run_time
        .execute_block(block_1)
        .expect("wrong block execution!");

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "my_document",
                }),
            },
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "bob's doc",
                }),
            },
        ],
    };

    run_time
        .execute_block(block_2)
        .expect("wrong block execution!");
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
        balances
            .transfer(bob(), alice(), 50)
            .unwrap();

        // assert
        assert_eq!(balances.balance(bob()), 50);
        assert_eq!(balances.balance(alice()), 100);
    }

    #[test]
    fn transfer_balance_insufficient() {
        // arrange
        let mut balances: Pallet<Runtime> = Pallet::new();
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
        let mut balances: Pallet<Runtime> = Pallet::new();
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
