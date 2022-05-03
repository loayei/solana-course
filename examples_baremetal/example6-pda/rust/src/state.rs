use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},    
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


/// Define the type of state stored in accounts
/// // global variables for that account?
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,	
}
