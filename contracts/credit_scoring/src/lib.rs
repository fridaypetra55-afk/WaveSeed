#![no_std]
use soroban_sdk::{contractimpl, Address, Env, Map, Symbol};

pub struct CreditScoring;

#[contractimpl]
impl CreditScoring {
    pub fn record_repayment(env: Env, who: Address, amount: i128) {
        let key = Symbol::short("scores");
        let mut scores: Map<Address, i128> = env.storage().get(&key).unwrap_or_else(|| Map::new(&env));
        let cur = scores.get(who.clone()).unwrap_or(0i128);
        let next = cur + (amount / 1000); // simple scoring heuristic
        scores.set(who.clone(), next);
        env.storage().set(&key, &scores);
        env.events().publish((symbol!("score_updated"),), (who, next));
    }

    pub fn get_score(env: Env, who: Address) -> i128 {
        let key = Symbol::short("scores");
        let scores: Option<Map<Address, i128>> = env.storage().get(&key);
        scores.and_then(|s| s.get(who)).unwrap_or(0i128)
    }
}
