use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum DaoError {
    // Member is already registered in the Dao.
    MemberAlreadyRegistered,
    // Member is not registered yet..
    MemberNotRegistered,
    // Member already voted the proposal.
    MemberAlreadyVoted,
    // Proposal does not exist in the Dao.
    ProposalDoesNotExist,
}

#[derive(Clone)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct BasicProposal {
    pub vote_count: u32,
}
