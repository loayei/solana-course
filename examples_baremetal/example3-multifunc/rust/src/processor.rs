use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,    
};


use crate::functions::{function_A, function_B};
use crate::instruction::Instruction;

pub struct Processor;
impl Processor {
	pub fn process_program_call(
		program_id: &Pubkey,
		_accounts: &[AccountInfo],
		instruction_data: &[u8],
	) -> ProgramResult {

		// turns bytecode into instrucion which contains function to invoke
		let instruction = Instruction::unpack(instruction_data)?;

		// split above top remove ?

		msg!("[processor] Received: {:?}",instruction);

		match instruction {
			Instruction::FunctionA => {
				function_A();
			}
			Instruction::FunctionB => {
				function_B();
			}
		}
		
		Ok(())
	}
}