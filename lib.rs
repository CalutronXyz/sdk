use anchor_lang::prelude::*;

declare_id!("CALuTRoN1111111111111111111111111111111");

#[program]
pub mod calutron {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.volatility_index = 0;
        pool.liquidity = 0;
        Ok(())
    }

    pub fn record_price_move(ctx: Context<RecordMove>, delta: i64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.volatility_index = pool.volatility_index.saturating_add(delta.abs() as u64);
        Ok(())
    }

    pub fn provide_liquidity(ctx: Context<ProvideLiquidity>, amount: u64) -> Result<()> {
        let pool = &mut ctx.accounts.pool;
        pool.liquidity += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(init, payer = user, space = 8 + 16)]
    pub pool: Account<'info, PoolState>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecordMove<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolState>,
}

#[derive(Accounts)]
pub struct ProvideLiquidity<'info> {
    #[account(mut)]
    pub pool: Account<'info, PoolState>,
}

#[account]
pub struct PoolState {
    pub volatility_index: u64,
    pub liquidity: u64,
}
