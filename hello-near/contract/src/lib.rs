// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, Balance, near_bindgen};
use near_sdk::serde::Serialize;
use near_sdk::collections::Vector;

// Define the default message
const POINT_ONE:Balance = 100_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct PostedMessage {
    pub index: u64,
    pub premium: bool,
    pub sender: AccountId,
    pub text: String
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    core: Vector<PostedMessage>,
}


// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self {
            core: Vector::new(b"m")
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> Vec<PostedMessage> {
        self.core.iter().collect()
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    #[payable]
    pub fn set_greeting(&mut self, text: String) {
        let premium = env::attached_deposit() >= POINT_ONE;
        let sender = env::predecessor_account_id();
        let index = self.core.len();

        let message = PostedMessage{index, premium, sender, text};
        self.core.push(&message);
    }

    pub fn total_greetings(&self) -> u64 {
        self.core.len()
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_messages() {
        let mut contract = Contract::default();

        contract.set_greeting("1".to_string());
        contract.set_greeting("2".to_string());
        contract.set_greeting("3".to_string());

        let total = &contract.total_greetings();

        assert_eq!(*total, 3);

        let messages = &contract.get_greeting();

        assert_eq!(messages[0].text, "1".to_string())
    }
}
