use anchor_lang::prelude::*;

declare_id!("9a5SvrAssQ5QtJTkSXRZYnedFadBNjpadztEsY6wu3ug");

#[program]
pub mod challenge_note {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}
