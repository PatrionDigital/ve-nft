use anchor_lang::prelude::*;

declare_id!("HRsZWKJF1h4xuPwEyu5dxtH5EXkziqhWBr8esmGERCGs");

#[program]
pub mod ve_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
