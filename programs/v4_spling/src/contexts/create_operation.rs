use crate::*;

#[derive(Accounts)]
#[instruction(index: u8, name: String, description: String)]
pub struct CreateOperation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, 
        space = 8 + mem::size_of::<Operation>(),
        seeds = [b"operation".as_ref(), &name.as_ref()], 
        bump)]
    pub operation: Account<'info, Operation>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreateOperation<'_> {
    pub fn process(&mut self, index: u8, name: String, description: String) -> Result<()> {
        let Self {operation,..} = self;

        operation.index = index;
        operation.title = name;
        operation.description = description;

        Ok(())
    }
}

