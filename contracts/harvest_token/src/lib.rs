#![no_std]
use soroban_sdk::{Address, contractimpl, Env, Symbol, Vec, Map};

pub struct HarvestToken;

#[contractimpl]
impl HarvestToken {
    pub fn mint(env: Env, token_id: Symbol, to: Address, amount: i128) {
        let key = Symbol::short("balances");
        let mut balances: Map<Symbol, Map<Address, i128>> = env.storage().get(&key).unwrap_or_else(|| Map::new(&env));
        let mut tokmap = balances.get(token_id).unwrap_or_else(|| Map::new(&env));
        let cur = tokmap.get(to.clone()).unwrap_or(0i128);
        tokmap.set(to.clone(), cur + amount);
        balances.set(token_id, tokmap);
        env.storage().set(&key, &balances);
        env.events().publish((symbol!("mint"),), (token_id, to, amount));
    }

    pub fn balance_of(env: Env, token_id: Symbol, who: Address) -> i128 {
        let key = Symbol::short("balances");
        let balances: Option<Map<Symbol, Map<Address, i128>>> = env.storage().get(&key);
        balances
            .and_then(|b| b.get(token_id))
            .and_then(|m| m.get(who))
            .unwrap_or(0i128)
    }
}
