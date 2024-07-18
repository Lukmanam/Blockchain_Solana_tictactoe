use anchor_lang::prelude::*;
use instructions::*;
use state::game::Square;

pub mod errors;
pub mod instructions;
pub mod state;


declare_id!("11111111111111111111111111111111");


#[program]
pub mod  solana_tic_tac_toe {
    use super::*;

// creating a token Mint for gameplay 
    pub fn init(ctx: Context<Init>) -> Result <()> {
        instructions::init::init(ctx)
    }

}

// Init New Player Account
    pub fn create_player(ctx: Context<CreatePlayer>) -> Result <()> {
        instructions::create_player::create_player(ctx)
    }

// Create new Game

    pub fn create_game(ctx: Context<CreateGame>, game_id: u64) -> Result<()> {
        instructions::create_game::create_game(ctx, game_id)
    }

//join the game

    pub fn join_game(ctx:Context<JoinGame>) -> Result<()> {
        instructions::join_game::join_game(ctx)
    }

// Movement

    pub fn play(ctx: Context<Play>, square: Square) -> Result<()> {
        instructions::play::play(ctx, square)
    }

// Claim reward

     pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::claim_reward::claim_reward(ctx)
    }





