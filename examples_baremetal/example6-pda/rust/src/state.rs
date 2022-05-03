use borsh::{BorshDeserialize, BorshSerialize};
/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GreetingAccount {
    // number of greetings
    pub counter: u32,
}
