use anchor_spl::token::Token;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, TokenAccount, Mint},
};

declare_id!("5nE3Ww1x63FDGzXuJT2cNmZsMn6xKymMVcShXBy3m2b");

#[program]
pub mod stake_nft {
    use super::*;
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    pub fn initialize(ctx: Context<Initialize>,nft_acc: Pubkey,staker: Pubkey,bump: u8,) -> Result<()> {
        let my_account = &mut ctx.accounts.my_account;
        my_account.nft_acc = nft_acc;
        my_account.staker = staker;
        my_account.bump = bump;
        Ok(())
    }
    pub fn stake(ctx: Context<Stake>)->ProgramResult{
        let _my_account = &mut ctx.accounts.my_account;
        token::transfer( 
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::Transfer {
                    from: ctx.accounts.staker_token_account.to_account_info(),
                    to: ctx.accounts.program_token_account.to_account_info(),
                    authority: ctx.accounts.staker.to_account_info(),
                },
            ),
            1,
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer = user,space = 8 + MyAccount::LEN)] 
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>, 
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut,has_one = staker,constraint = my_account.is_staked == 0 )] 
    pub my_account: Account<'info, MyAccount>,
    #[account(seeds = [my_account.key().as_ref(), b"authority"],bump,)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub authority: UncheckedAccount<'info>,
    #[account(init,payer = staker,token::mint = stake_mint,token::authority = authority,seeds =[my_account.key().as_ref(), b"stake"],bump,)]
    pub program_token_account:Box<Account<'info, TokenAccount>>,
    pub stake_mint: Box<Account<'info, Mint>>, 
    #[account(mut)]
    pub staker: Signer<'info>, 
    #[account(mut)]
    pub staker_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    
}
#[account]
pub struct MyAccount{
    pub nft_acc:Pubkey,
    pub staker:Pubkey,
    pub is_staked:u64,
    pub bump:u8,  
} 
impl MyAccount{
    pub const LEN:usize=32+32+8+1;
}
