use anchor_lang::prelude::*;

// esta pija declara un id unico del programa (no es un hash del codigo ni nada, es un id
// autogenerado cuando creas el proyecto con anchor)
declare_id!("4Mx8JsEfP1pYkddqimqUt3zhP9tBxXJxNg3qtMJTDPVc");

// seria como el main, la logica mas de alto nivel
#[program]
pub mod nukem_anchor {
    use super::*;

    // funcion init: setea el initial state del programa
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        let global_state = &mut ctx.accounts.global_state;
        global_state.total_staked = 0;
        Ok(())
    }

    // llamada para cuando un usuario quiere stakear
    pub fn stake_tokens(ctx: Context<StakeTokens>, amount: u64) -> ProgramResult {
        let stake_account = &mut ctx.accounts.stake_account;
        let global_state = &mut ctx.accounts.global_state;
        
        if amount <= 0 {
            return Err(ProgramError::InvalidArgument)
        }
        stake_account.staked_amount += amount;
        global_state.total_staked += amount;

        Ok(())
    }

    // high-level llamada cuando un jugador quiere nukear
    pub fn fire(ctx: Context<Fire>) -> ProgramResult {
        // TODO: hacer un pseudorand() 
        // idea: input -> concat time con blockhash o algo asi
        Ok(())
    }
}


#[account]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_amount: u64,
}

#[account]
pub struct GlobalState {
    pub total_staked: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub global_state: Account<'info, GlobalState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    #[account(mut)]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(mut)]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)] 
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Fire<'info> {
    // Define necessary accounts, such as stakers' accounts
}

