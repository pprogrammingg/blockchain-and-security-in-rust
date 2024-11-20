use web3dev_blockchain_from_scratch::{
    balances,
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
}

pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
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
            system: system::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        // increment system block numer
        self.system.inc_block_number();

        // check the incoming block number matches system block number
        // return error if not
        let curr_block_number = self.system.block_number();
        if curr_block_number != block.header.block_number {
            return Err("Block number mismatch");
        }

        // iterate over the extrinsics in the block...
        // enumerate to get iteration number
        for (i, support::Extrinsic { caller, call }) in block
            .extrinsics
            .into_iter()
            .enumerate()
        {
            // increment the nonce of the caller
            self.system
                .inc_nonce(caller.clone());

            // dispatch the extrinsic using the caller and the call contained within extrinsic
            let _ = self
                .dispatch(caller, call)
                .map_err(|e| {
                    eprintln!(
                        "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                        block.header.block_number, i, e
                    )
                });
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        // This match statement will allow us to correctly route `RuntimeCall`s
        // to the appropriate pallet level function.
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.balances
                    .dispatch(caller, call)?;
            }
        }
        Ok(())
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

    run_time
        .balances
        .set_balance(alice.clone(), 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: bob,
                    amount: 30,
                }),
            },
            support::Extrinsic {
                caller: alice,
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: charlie,
                    amount: 20,
                }),
            },
        ],
    };

    run_time
        .execute_block(block_1)
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
