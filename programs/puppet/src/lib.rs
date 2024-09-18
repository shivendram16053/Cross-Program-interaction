use anchor_lang::prelude::*;

declare_id!("CjQhTWFwN7jQYo2SUjhkdi3R94eg94sGAU2cRkPAzkPk");

#[program]
pub mod puppet {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,authority:Pubkey) -> Result<()> {
        ctx.accounts.puppet.authority = authority;
        Ok(())
    }

    pub fn setdata(ctx:Context<SetData>,data:u64) -> Result<()>{
        let puppet = &mut ctx.accounts.puppet;
        puppet.data = data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,payer=user,space=8+8+128)]
    pub puppet:Account<'info,Data>,
    #[account(mut)]
    pub user:Signer<'info>,
    pub system_program: Program<'info,System>,
}

#[derive(Accounts)]
pub struct SetData<'info> {
    #[account(mut)]
    pub puppet: Account<'info, Data>,
    pub authority:Signer<'info>
}

#[account]
pub struct Data{
    pub data:u64,
    pub authority:Pubkey
}