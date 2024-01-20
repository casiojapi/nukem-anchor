use anchor_lang::prelude::*;

declare_id!("4Mx8JsEfP1pYkddqimqUt3zhP9tBxXJxNg3qtMJTDPVc");

#[program]
pub mod nukem_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    
    }
}

#[derive(Accounts)]
pub struct Initialize {}
