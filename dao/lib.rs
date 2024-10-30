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
        fn register_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberAlreadyRegistered` if the member is registered
            // - Success: Register a new `member` to the Dao
            Ok(())
        }

        #[ink(message)]
        fn deregister_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Success: Deregister a new `voter` from the Dao
            Ok(())
        }

        #[ink(message)]
        fn has_voter(&self, voter: AccountId) -> bool {
            // - Success: Return if the voter is registered.
            todo!();
        }

        #[ink(message)]
        fn create_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Success: Create a new proposal that stores `votes` from `voters`
            Ok(())
        }

        #[ink(message)]
        fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `voters`
            Ok(())
        }

        #[ink(message)]
        fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::MemberNotRegistered` if the member is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal
            Ok(())
        }

        #[ink(message)]
        fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            todo!();
        }

        #[ink(message)]
        fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            todo!();
        }
    }

    impl SubDao for Dao {
        #[ink(message)]
        fn create_superdao_contract_call_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to call a contract method.
            Ok(())
        }

        #[ink(message)]
        fn vote_superdao_proposal(&mut self, proposal_id: u32, vote: bool) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Vote a SuperDao proposal.
            Ok(())
        }

        #[ink(message)]
        fn create_superdao_cross_chain_proposal(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a SuperDao proposal to execute a cross-chain message.
            Ok(())
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
        fn challenge_1_test_name() {
            let dao = Dao::new(String::from("any name"));
            assert_eq!(dao.name, String::from("any name"));
        }

        #[ink::test]
        fn challenge_2_test_voter_registration() {
            // Example of the unit test for voter registration.
            let mut dao = Dao::default();
            let accounts = ink::env::test::default_accounts::<Environment>();

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.register_voter().is_ok());
            assert_eq!(dao.register_voter(), Err(DaoError::VoterAlreadyRegistered;));
            assert_eq!(dao.voters.len(), 1);
            assert!(dao.has_voter(accounts.alice));

            ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
            assert!(dao.deregister_voter().is_ok());
            assert_eq!(dao.deregister_voter(), Err(DaoError::VoterNotRegistered));
            assert_eq!(dao.voters.get(0), None);
            assert!(!dao.has_voter(accounts.alice));
        }

        #[ink::test]
        fn challenge_2_test_proposal_management() {
            todo!("Challenge 2");
        }

        #[ink::test]
        fn challenge_2_test_vote() {
            todo!("Challenge 3");
        }

        #[ink::test]
        fn challenge_3_test_create_superdao_contract_call_proposal() {
            todo!("Challenge 3");
        }

        #[ink::test]
        fn challenge_3_test_vote_superdao_proposal() {
            todo!("Challenge 3");
        }

        #[ink::test]
        fn challenge_4_test_vote_superdao_cross_chain_proposal() {
            todo!("Challenge 4");
        }
    }
}
