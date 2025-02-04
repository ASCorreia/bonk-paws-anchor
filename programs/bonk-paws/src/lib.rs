use anchor_lang::prelude::*;

pub mod contexts;
pub mod programs;
pub mod mints;
pub mod errors;
pub mod macros;
pub mod constants;
pub mod state;

use contexts::*;

declare_id!("AVWhsnDDwm7PEaijsyQEv4aJ6YnjvnW4WgL4569mf6Gt");

#[program]
pub mod bonk_paws {
    use super::*;

    pub fn donate(ctx: Context<DonateSol>, seeds: u64, sol_donation: u64) -> Result<()> {
        ctx.accounts.donate_sol(seeds, sol_donation)
    }

    pub fn match_donation(ctx: Context<MatchDonation>, bonk_donation: u64) -> Result<()> {
        ctx.accounts.match_donation(bonk_donation)
    }

    pub fn finalize_donation(ctx: Context<FinalizeDonation>) -> Result<()> {
        ctx.accounts.finalize_donation()
    }
}