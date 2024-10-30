use super::*;

#[ink::trait_definition]
pub trait NamedDao {
    #[ink(message)]
    fn name(&self) -> String;
}

#[ink::trait_definition]
pub trait BasicDao {
    #[ink(message)]
    fn register_member(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn deregister_member(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn get_member(&self, member: AccountId) -> Option<AccountId>;

    #[ink(message)]
    fn create_proposal(&mut self) -> Result<(), DaoError>;

    #[ink(message)]
    fn get_proposal(&self, proposal: u32) -> Option<BasicProposal>;

    #[ink(message)]
    fn vote(&mut self, proposal: u32) -> Result<(), DaoError>;

    #[ink(message)]
    fn vote_count(&self, proposal: i32, member: AccountId) -> Option<u32>;
}
