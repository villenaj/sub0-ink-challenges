use super::*;

// Challenge 1: Basics of ink! and setting up a DAO contract
#[ink::trait_definition]
pub trait SimpleDao {
    /// Returns the name of the DAO.
    #[ink(message)]
    fn name(&self) -> String;
}

// Challenge 2: Implement voter registration, proposal management, and voting in your DAO
#[ink::trait_definition]
pub trait SubDao {
    // Register a new member to the DAO.
    #[ink(message)]
    fn register_member(&mut self);

    // Register a new member to the DAO.
    #[ink(message)]
    fn deregister_member(&mut self);

    // Returns all the DAO members.
    #[ink(message)]
    fn get_members(&self) -> Vec<AccountId>;

    // Returns the number of votes a DAO member has.
    #[ink(message)]
    fn vote_count(&self, id: AccountId) -> u32;
}
