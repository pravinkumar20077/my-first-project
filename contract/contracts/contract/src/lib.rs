#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, symbol_short, Symbol};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Charity,   // The address that receives the funds
    Donors,    // A map of donor addresses to amounts
    Total,     // Total amount raised
}

#[contract]
pub struct CharityTracker;

#[contractimpl]
impl CharityTracker {
    /// Initialize the contract with the charity's wallet address
    pub fn init(env: Env, charity: Address) {
        if env.storage().instance().has(&DataKey::Charity) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Charity, &charity);
        env.storage().instance().set(&DataKey::Total, &0u128);
    }

    /// Donate funds to the charity
    pub fn donate(env: Env, donor: Address, amount: u128) {
        donor.require_auth();

        // 1. Update total funds
        let mut total: u128 = env.storage().instance().get(&DataKey::Total).unwrap_or(0);
        total += amount;
        env.storage().instance().set(&DataKey::Total, &total);

        // 2. Update individual donor history
        let mut donors: Map<Address, u128> = env
            .storage()
            .persistent()
            .get(&DataKey::Donors)
            .unwrap_or(Map::new(&env));
        
        let prev_amount = donors.get(donor.clone()).unwrap_or(0);
        donors.set(donor, prev_amount + amount);
        
        env.storage().persistent().set(&DataKey::Donors, &donors);

        // Note: In a production app, you would include a Cross-Contract Call 
        // to the Stellar Asset Contract (SAC) here to actually move the XLM.
    }

    /// Get the total amount raised
    pub fn get_total(env: Env) -> u128 {
        env.storage().instance().get(&DataKey::Total).unwrap_or(0)
    }

    /// Check how much a specific address has donated
    pub fn get_donor_amount(env: Env, donor: Address) -> u128 {
        let donors: Map<Address, u128> = env
            .storage()
            .persistent()
            .get(&DataKey::Donors)
            .unwrap_or(Map::new(&env));
        donors.get(donor).unwrap_or(0)
    }
}