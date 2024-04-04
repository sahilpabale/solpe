use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;

declare_id!("AHnWUmyXfyRqtN3AwoRxMoyVKskyF9cS59YE2ZbqB8x6");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        initializer_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize_vault(seed, &ctx.bumps, initializer_amount, taker_amount)?;
        ctx.accounts.deposit(initializer_amount)
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }
}
