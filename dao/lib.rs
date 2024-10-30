#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::{prelude::vec::Vec, primitives::AccountId};

mod traits;

#[ink::contract]
mod dao {
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

        #[ink(message)]
        pub fn flip(&mut self, value: bool) {
            self.value = value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_test() {
        assert_eq!(1 + 1, 2);
    }
}
