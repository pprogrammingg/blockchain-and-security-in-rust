use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
        }
    }
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Pallet {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    // add claim
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims
                    .insert(claim, caller);
                Ok(())
            }
        }
    }

    // revoke claim
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self
            .get_claim(&claim)
            .ok_or("Claim does not exist")?;

        if claim_owner != &caller {
            return Err("Caller is not the owner of the claim");
        }

        self.claims.remove(&claim);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::system::Config;

    struct TestConfig;

    impl Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe = super::Pallet::<TestConfig>::new();
        let _ = poe.create_claim("alice", "my_document");
        assert_eq!(poe.get_claim(&"my_document"), Some(&"alice"));

        let res = poe.revoke_claim(&"bob", "my_document");
        assert_eq!(res, Err("Caller is not the owner of the claim"));

        let res = poe.create_claim("bob", "my_document");
        assert_eq!(res, Err("Claim already exists"));

        let res = poe.revoke_claim("alice", "non existent");
        assert_eq!(res, Err("Claim does not exist"));

        let _ = poe.revoke_claim("alice", "my_document");
        assert_eq!(poe.get_claim(&"my_document"), None);
    }
}
