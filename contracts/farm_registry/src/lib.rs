#![no_std]
use soroban_sdk::{contractimpl, symbol, Address, Env, Map, Symbol};

pub struct FarmRegistry;

#[derive(Clone)]
pub struct Farm {
    pub owner: Address,
    pub metadata: Symbol,
}

#[contractimpl]
impl FarmRegistry {
    pub fn register(env: Env, farm_id: Symbol, owner: Address, metadata: Symbol) {
        let key = Symbol::short("farm_owner");
        let mut map: Map<Symbol, Farm> = env.storage().get(&key).unwrap_or_else(|| Map::new(&env));
        if map.contains_key(&farm_id) {
            panic!("farm-already-registered")
        }
        let farm = Farm { owner, metadata };
        map.set(farm_id, farm);
        env.storage().set(&key, &map);
        env.events().publish((symbol!(farm_registered),), farm_id);
    }

    pub fn get_owner(env: Env, farm_id: Symbol) -> Option<Address> {
        let key = Symbol::short("farm_owner");
        let map: Option<Map<Symbol, Farm>> = env.storage().get(&key);
        map.and_then(|m| m.get(farm_id).map(|f| f.owner))
    }
}

fn symbol_registered() -> Symbol { symbol!("farm_registered") }

mod tests {
    // Tests would go here using soroban-sdk test harness
}
