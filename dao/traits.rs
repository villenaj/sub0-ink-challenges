use super::*;

#[ink::trait_definition]
pub trait NamedDao {
    #[ink(message)]
    fn name(&self) -> String;
}

#[ink::trait_definition]
pub trait BasicDao {
    // Vote registration.

    #[ink(message)]
    fn register_voter(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn deregister_voter(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn has_voter(&self, voter: AccountId) -> bool;

    // Proposal management.

    #[ink(message)]
    fn create_proposal(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn remove_proposal(&mut self, proposal_id: u32) -> Result<(), DaoError>;

    #[ink(message)]
    fn get_proposal(&self, proposal_id: u32) -> Option<BasicProposal>;

    // Voting.

    #[ink(message)]
    fn vote(&mut self, proposal_id: u32) -> Result<(), DaoError>;

    #[ink(message)]
    fn vote_count(&self, member: AccountId) -> u32;
}

#[ink::trait_definition]
pub trait SubDao {
    // Connect your DAO to the Super DAO with registration and voting

    #[ink(message)]
    fn create_superdao_contract_call_proposal(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn vote_superdao_proposal(&mut self, proposal_id: u32, vote: bool) -> Result<(), DaoError>;

    // Support creating cross-chain proposals to the Super DAO

    #[ink(message)]
    fn create_superdao_cross_chain_proposal(&mut self) -> Result<(), DaoError>;
}
