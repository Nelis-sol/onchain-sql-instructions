use crate::*;

#[derive(Accounts)]
pub struct CreatePayer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(init, payer = sender, space = 8 + mem::size_of::<Payer>(), seeds = [b"payer"], bump)]
    pub payer: Account<'info, Payer>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePayer<'_> {
    pub fn process(&mut self, bump: u8) -> Result<()> {
        let Self {payer,..} = self;

        // store the bump for later retrieval
        self.payer.bump = bump;

        Ok(())

    }

}