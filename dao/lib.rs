#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # List of challenges
//
// Challenge 1: Basics of ink! and setting up a Dao contract
// Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
// Challenge 3: Connect your DAO to the Super DAO with registration and voting
// Challenge 4: Support creating cross-chain proposals to the Super DAO

use crate::{traits::*, types::*};
use ink::codegen::Env;
use ink::{
    env::{DefaultEnvironment, Environment},
    prelude::vec::Vec,
    primitives::AccountId,
};
use superda0_traits::superdao::{Call, Proposal as SuperDaoProposal};

mod traits;
mod types;
mod utils;

#[ink::contract]
mod dao {
    use super::*;

    #[ink(storage)]
    pub struct Dao {
        value: bool,
    }

    impl Dao {
        // Constructor that initializes the values for the contract.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
    }

    impl NamedDao for Dao {
        #[ink(message)]
        fn name(&self) -> String {
            // - Returns the name of the Dao
            todo!();
        }
    }

    impl BasicDao for Dao {
        #[ink(message)]
        fn register_member(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `Error::MemberAlreadyRegistered` if the member is registered
            // - Success: Register a new `member` to the Dao
            Ok(())
        }

        #[ink(message)]
        fn deregister_member(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `Error::MemberNotRegistered` if the member is not registered
            // - Success: Register a new `member` to the Dao
            Ok(())
        }

        #[ink(message)]
        fn get_member(&self, _member: AccountId) -> Option<AccountId> {
            // - Success: Returns the Dao `member` by address. `None` if there is no member found.
            todo!();
        }

        #[ink(message)]
        fn create_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `members`
            Ok(())
        }

        #[ink(message)]
        fn get_proposal(&self, _proposal: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            todo!();
        }

        #[ink(message)]
        fn vote(&mut self, _proposal: u32) -> Result<(), DaoError> {
            // - Error: Throw error `Error::MemberNotRegistered` if the member is not registered
            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal
            Ok(())
        }

        #[ink(message)]
        fn vote_count(&self, _proposal: i32, _member: AccountId) -> Option<u32> {
            // - Returns the number of `votes` a Dao `member` voted
            todo!();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::dao::Dao;

        #[ink::test]
        fn mock_test() {
            assert_eq!(1 + 1, 2);
        }

        #[ink::test]
        fn challenge_1__test_name() {
            let dao = Dao::new("any name");
            assert_eq!(dao.name, "any name");
        }

        #[ink::test]
        fn challenge_2__test_member_registration() {
            let dao = Dao::default();
            let accounts = ink::env::test::default_accounts::<Environment>();

            assert_eq!(dao.get_member(accounts.alice), None);
            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);

            assert_ok!(dao.register_member());
            assert_err!(dao.register_member(), Error::MemberAlreadyRegistered);
            assert_eq!(dao.members.len(), 1);
            assert!(dao.members.contains(&accounts.alice));
            assert_eq!(dao.get_member(accounts.alice), Some(account.alice));
        }

        #[ink::test]
        fn challenge_2__test_vote_count() {
            todo!();
        }

        #[ink::test]
        fn challenge_2__test_create_proposal() {
            todo!();
        }
    }
}
