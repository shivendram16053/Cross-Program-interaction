use anchor_lang::prelude::*;
use puppet::cpi::accounts::SetData;
use puppet::program::Puppet;
use puppet::{self, Data};

declare_id!("8r1v8mcoxuG3noMMdVcwfS7X2hbLnNLURksDTj4CpM4e");

#[program]
pub mod puppet_master {
    use super::*;

    //Setting up CPI is distractable so we use impl

    // pub fn pull_strings(ctx:Context<PullString>,data:u64) -> Result<()>{
    //     let cpi_program= ctx.accounts.puppet_program.to_account_info();
    //     let cpi_accounts = SetData{
    //         puppet:ctx.accounts.puppet.to_account_info(),
    //     };
    //     let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    //     let _ = puppet::cpi::setdata(cpi_ctx,data);
    //     Ok(())
    // }

    pub fn pull_string(ctx:Context<PullString>,data:u64) -> Result<()> {
        let _ =puppet::cpi::setdata(ctx.accounts.set_data_ctx(), data);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PullString<'info>{
    #[account(mut)]
    pub puppet:Account<'info,Data>,
    pub puppet_program : Program<'info,Puppet>,
    pub authority : Signer<'info>
}

impl<'info> PullString<'info> {
    pub fn set_data_ctx(&self)-> CpiContext<'_,'_,'_,'info,SetData<'info>>{
        let cpi_program = self.puppet_program.to_account_info();
        let cpi_accounts = SetData{
            puppet: self.puppet.to_account_info(),
            authority: self.authority.to_account_info()
        };

        CpiContext::new(cpi_program, cpi_accounts)
    }
    
}