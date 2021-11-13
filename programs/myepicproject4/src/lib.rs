use anchor_lang::prelude::*;

declare_id!("F6ZPB3C5VaxqrYNihNmVMQzZruqjADiPe9NznZJvzcQv");

#[program]
pub mod myepicproject4 {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        // get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialise total_gifs
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}

// attach variables to the Start stuff off
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    // add a signer so we can save it
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct to work with
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// tell Solana what we want to store on this account
// initialise account
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // attach a vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}