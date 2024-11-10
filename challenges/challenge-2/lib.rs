#![cfg_attr(not(feature = "std"), no_std, no_main)]

// # ✒️ Challenge 2: Implement voter registration, proposal management, and voting in your Dao.
//
// - **Difficulty**: Mid
// - **Submission Criteria:** ink! contract must
//     - Use a storage-optimized data-structure `Mapping` or `StorageVec`
//     - Store registered members, member votes, and proposals to vote on.
//     - Implement methods to register and de-register members.
//     - Implement methods to create proposals and a method to vote on proposals.
//     - Unit tests for adding members, votes, and proposals.
// - **Submission Guidelines:**
//     - Verify with R0GUE DevRel, and post on X.
// - **Prize:** sub0 merch

#[ink::contract]
mod dao {
    use ink::{
        prelude::string::String,
        storage::{Mapping, StorageVec},
    };
    use minidao_common::*;

    #[derive(Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
    )]
    #[ink::scale_derive(Encode, Decode, TypeInfo)]
    pub struct BasicProposal {
        pub description: String,
        pub vote_count: u32,
    }

    #[ink(storage)]
    pub struct Dao {
        voters: Mapping<AccountId, u32>,
        proposals: Mapping<u32, BasicProposal>,
        proposal_count: u32,
    }

    impl Dao {
        // Constructor that initializes the default values for the contract.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                voters: Mapping::new(),
                proposals: Mapping::new(),
                proposal_count: 0,
            }
        }

        #[ink(message)]
        pub fn get_name(&self) -> String {
            // - Returns the name of the Dao
            todo!()
        }

        #[ink(message)]
        pub fn register_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterAlreadyRegistered` if the voter is registered
            // - Success: Register a new `voter` to the Dao
			let caller = self.env().caller();
            if self.voters.contains(&caller) {
                return Err(DaoError::VoterAlreadyRegistered);
            }
            self.voters.insert(caller, &0);
            Ok(())
        }

        #[ink(message)]
        pub fn deregister_voter(&mut self) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Deregister a new `voter` from the Dao
			let caller = self.env().caller();
            if !self.voters.contains(&caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            self.voters.remove(&caller);
            Ok(())
        }

        #[ink(message)]
        pub fn has_voter(&self, voter: AccountId) -> bool {
            // - Success: Return if the voter is registered.
            // todo!();
			self.voters.contains(&voter)
        }

        #[ink(message)]
        pub fn create_proposal(&mut self, description: String) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Success: Create a new proposal that stores `votes` from `voters`
			let caller = self.env().caller();
            if !self.voters.contains(&caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            let proposal_id = self.proposal_count;
            self.proposals.insert(proposal_id, &BasicProposal { description, vote_count: 0 });
            self.proposal_count += 1;
            Ok(())
        }

        #[ink(message)]
        pub fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `DaoError::ProposalDoesNotExist` if the proposal is not created
            // - Success: Create a new proposal that stores `votes` from `voters`
			let caller = self.env().caller();
            if !self.voters.contains(&caller) {
                return Err(DaoError::VoterNotRegistered);
            }
            if !self.proposals.contains(&proposal_id) {
                return Err(DaoError::ProposalDoesNotExist);
            }
            self.proposals.remove(&proposal_id);
            Ok(())
        }

        #[ink(message)]
        pub fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal> {
            // - Success: Returns the proposal detail
            // todo!()
			self.proposals.get(proposal_id)
        }

        #[ink(message)]
        pub fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError> {
            // - Error: Throw error `DaoError::VoterNotRegistered` if the voter is not registered
            // - Error: Throw error `Error::ProposalDoesNotExist` if the proposal is not created
            // - Success: Vote on the proposal
			let caller = self.env().caller();
            if !self.voters.contains(&caller) {
                return Err(DaoError::VoterNotRegistered);
            }
			
            let mut proposal = self.proposals.get(proposal_id).ok_or(DaoError::ProposalDoesNotExist)?;
            proposal.vote_count += 1;
            self.proposals.insert(proposal_id, &proposal);
            Ok(())
        }

        #[ink(message)]
        pub fn vote_count(&self, voter: AccountId) -> u32 {
            // - Returns the number of `votes` a Dao `voter` voted
            // todo!();
			self.voters.get(&voter).unwrap_or(0)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
		use ink::env::test;
        use crate::dao::Dao;

        #[ink::test]
        fn test_voter_registration() {
            // todo!("Challenge 2");
			let mut dao = Dao::default();
			let caller = test::default_accounts::<ink::env::DefaultEnvironment>().alice;

			test::set_caller::<ink::env::DefaultEnvironment>(caller);
			// Register the voter
			assert_eq!(dao.register_voter(), Ok(()));
			assert!(dao.has_voter(caller));

			// Attempt to register the same voter again
			assert_eq!(dao.register_voter(), Err(DaoError::VoterAlreadyRegistered));

			// Deregister the voter
			assert_eq!(dao.deregister_voter(), Ok(()));
			assert!(!dao.has_voter(caller));

			// Attempt to deregister a non-registered voter
			assert_eq!(dao.deregister_voter(), Err(DaoError::VoterNotRegistered));
        }

        #[ink::test]
        fn test_proposal_management() {
            // todo!("Challenge 2");
			let mut dao = Dao::default();
			let caller = test::default_accounts::<ink::env::DefaultEnvironment>().alice;
			test::set_caller::<ink::env::DefaultEnvironment>(caller);

			// Register the caller as a voter
			dao.register_voter().unwrap();

			// Create a proposal
			let proposal_desc = "Proposal 1".to_string();
			assert_eq!(dao.create_proposal(proposal_desc.clone()), Ok(()));
			assert_eq!(dao.get_proposal(0).unwrap().description, proposal_desc);

			// Attempt to remove a non-existent proposal
			assert_eq!(dao.remove_proposal(1), Err(DaoError::ProposalDoesNotExist));

			// Remove an existing proposal
			assert_eq!(dao.remove_proposal(0), Ok(()));
			assert_eq!(dao.get_proposal(0), None);
        }

        #[ink::test]
        fn test_vote() {
            // todo!("Challenge 2");
			let mut dao = Dao::default();
			let caller = test::default_accounts::<ink::env::DefaultEnvironment>().alice;
			test::set_caller::<ink::env::DefaultEnvironment>(caller);

			// Register the caller as a voter
			dao.register_voter().unwrap();

			// Create a proposal
			dao.create_proposal("Proposal 1".to_string()).unwrap();

			// Vote on the proposal
			assert_eq!(dao.vote(0), Ok(()));
			assert_eq!(dao.get_proposal(0).unwrap().vote_count, 1);

			// Attempt to vote on a non-existent proposal
			assert_eq!(dao.vote(1), Err(DaoError::ProposalDoesNotExist));
        }
    }
}
