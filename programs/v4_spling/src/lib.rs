use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::solana_program::system_program;
use std::mem;
use std::io::Write;

declare_id!("fvHdsgDm2dk6Wo8sjYiDu6EALS9bLehLwXSjsPPwMjZ");

pub mod contexts;
pub mod states;

pub use contexts::*;
pub use states::*;


#[program]
pub mod v4_spling {
    use super::*;

    pub fn create_payer(ctx: Context<CreatePayer>) -> Result<()> {
        let bump = *ctx.bumps.get("payer").unwrap();
        ctx.accounts.process(bump)
    }

    pub fn submit_transaction(ctx: Context<SubmitTransaction>, operation: u8, schema: u8, storage: Pubkey, hash: Option<String>, pointer: Option<String>, unique: Option<String>) -> Result<()> {
        ctx.accounts.process(operation, schema, storage, hash, pointer, unique)
    }

    pub fn create_operation(ctx: Context<CreateOperation>, index: u8, name: String, description: String) -> Result<()> {
        ctx.accounts.process(index, name, description)
    }

    pub fn update_operation(ctx: Context<UpdateOperation>, index: u8, name: String, description: String) -> Result<()> {
        ctx.accounts.process(index, name, description)
    }

    pub fn create_schema(ctx: Context<CreateSchema>, index: u8, name: String, description: String) -> Result<()> {
        ctx.accounts.process(index, name, description)
    }

    pub fn update_schema(ctx: Context<UpdateSchema>, index: u8, name: String, description: String) -> Result<()> {
        ctx.accounts.process(index, name, description)
    }
}
