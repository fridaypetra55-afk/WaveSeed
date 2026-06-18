#![no_std]
use soroban_sdk::{contractimpl, Address, Env, Map, Symbol};

pub struct LendingPool;

#[derive(Clone)]
pub struct Loan {
    pub borrower: Address,
    pub principal: i128,
    pub collateral_token: Symbol,
    pub collateral_amount: i128,
    pub repaid: bool,
}

#[contractimpl]
impl LendingPool {
    pub fn deposit_collateral(env: Env, borrower: Address, token: Symbol, amount: i128) {
        let key = Symbol::short("collateral");
        let mut coll: Map<Address, Map<Symbol, i128>> = env.storage().get(&key).unwrap_or_else(|| Map::new(&env));
        let mut by_token = coll.get(borrower.clone()).unwrap_or_else(|| Map::new(&env));
        let cur = by_token.get(token.clone()).unwrap_or(0i128);
        by_token.set(token.clone(), cur + amount);
        coll.set(borrower.clone(), by_token);
        env.storage().set(&key, &coll);
        env.events().publish((symbol!("collateral_deposited"),), (borrower, token, amount));
    }

    pub fn request_loan(env: Env, borrower: Address, principal: i128, token: Symbol, collateral_amount: i128) {
        // Simplified: require collateral exists
        let key = Symbol::short("collateral");
        let coll: Map<Address, Map<Symbol, i128>> = env.storage().get(&key).unwrap_or_else(|| Map::new(&env));
        let by_token = coll.get(borrower.clone()).unwrap_or_else(|| Map::new(&env));
        let cur = by_token.get(token.clone()).unwrap_or(0i128);
        if cur < collateral_amount { panic!("insufficient-collateral") }
        let loans_key = Symbol::short("loans");
        let mut loans: Map<u32, Loan> = env.storage().get(&loans_key).unwrap_or_else(|| Map::new(&env));
        let id = (loans.len() as u32) + 1;
        let loan = Loan { borrower: borrower.clone(), principal, collateral_token: token.clone(), collateral_amount, repaid: false };
        loans.set(id, loan);
        env.storage().set(&loans_key, &loans);
        env.events().publish((symbol!("loan_created"),), (id, borrower, principal));
    }

    pub fn repay(env: Env, loan_id: u32) {
        let loans_key = Symbol::short("loans");
        let mut loans: Map<u32, Loan> = env.storage().get(&loans_key).unwrap_or_else(|| Map::new(&env));
        let mut loan = loans.get(loan_id).expect("loan-not-found");
        loan.repaid = true;
        loans.set(loan_id, loan);
        env.storage().set(&loans_key, &loans);
        env.events().publish((symbol!("loan_repaid"),), loan_id);
    }
}
