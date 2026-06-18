#![no_std]
use soroban_sdk::{contractimpl, Env, Symbol, Map};

pub struct YieldOracle;

#[contractimpl]
impl YieldOracle {
    pub fn set_yield(env: Env, farm_id: Symbol, expected_yield: i128) {
        let key = Symbol::short("yields");
        let mut ys: Map<Symbol, i128> = env.storage().get(&key).unwrap_or_else(|| Map::new(&env));
        ys.set(farm_id.clone(), expected_yield);
        env.storage().set(&key, &ys);
        env.events().publish((symbol!("yield_updated"),), (farm_id, expected_yield));
    }

    pub fn get_yield(env: Env, farm_id: Symbol) -> i128 {
        let key = Symbol::short("yields");
        let ys: Option<Map<Symbol, i128>> = env.storage().get(&key);
        ys.and_then(|m| m.get(farm_id)).unwrap_or(0i128)
    }
}
