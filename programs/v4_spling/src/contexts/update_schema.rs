use crate::*;

#[derive(Accounts)]
#[instruction(index: u8, name: String, description: String)]
pub struct UpdateSchema<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"schema".as_ref(), &name.as_ref()], bump)]
    pub schema: Account<'info, Schema>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> UpdateSchema<'_> {
    pub fn process(&mut self, index: u8, name: String, description: String) -> Result<()> {
        let Self {schema,..} = self;

        schema.index = index;
        schema.title = name;
        schema.description = description;

        Ok(())
    }
}

