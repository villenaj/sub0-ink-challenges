# Sub0 `ink!` Challenges
ink! programming language challenges made for the Sub0 Reset Bangkok

## Getting started

Before tackling the challenges, you need to setup your local environment for ink! contract development first.

Install `pop-cli`: https://learn.onpop.io/cli/installing-pop-cli/

## About the challenges

To tackle the challenges, first fork and clone the repository

```
git clone something_will_be_updated
```

### Challenge 1: Basics of ink! and setting up a DAO contract

#### Description
A DAO is an online group where everyone gets to vote on decisions, so no single person is in charge. It’s a fair way for people to work together from anywhere!
In this first challenge, you will have to build a DAO contract with ink! programming language. This DAO will have a name and the identifier.

#### What you will learn?
- Setup environment with Pop CLI
- Introduce the components of ink!
- Create storage items for the DAO, including DAO name and unique ID.
- Deploying and using contract on Pop Network

#### Goal of the challenge

- [ ] Understand how to run the ink! contract, test and call the contract with `pop-cli`.
- [ ] DAO can have a name on initialization.

### Challenge 2: Membership and voting mechanism to the DAO.

#### Description
Improve your contract by maintaining valid voters and votes.

#### What you will learn?
- How ink! storage works.
- Storage optimized data structures: `StorageVec` and `Mapping`
- Maintain a `StorageVec` of valid voter addresses.
- Maintain a `Mapping` of voter addresses to votes.

#### Goal of the challenge

- [ ] DAO can register new members.
- [ ] DAO members has a permission to vote on the activities of the DAO.
- [ ] DAO authority can also remove the member from the DAO.
