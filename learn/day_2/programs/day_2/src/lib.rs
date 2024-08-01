use anchor_lang::prelude::*;

declare_id!("vrRUFgnWdKNNqG9xgXoArH4k3mn4ZnpVNCQXHjN36ov");

#[program]
pub mod day_2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
