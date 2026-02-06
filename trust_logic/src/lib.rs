#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct TrustContract;

#[contractimpl]
impl TrustContract {
    // This function increments a "Trust Score" for an account
    pub fn add_trust(env: Env, user: Symbol) -> u32 {
        let count_key = symbol_short!("score");
        let mut score: u32 = env.storage().instance().get(&user).unwrap_or(0);
        
        score += 1;
        
        env.storage().instance().set(&user, &score);
        score
    }
}