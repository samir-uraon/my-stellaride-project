#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, Address, Map
};

#[contract]
pub struct AffiliatePlatform;

#[contracttype]
#[derive(Clone)]
pub struct Affiliate {
    pub code: Symbol,
}

#[contractimpl]
impl AffiliatePlatform {

    // Register an affiliate with a referral code
    pub fn register(env: Env, user: Address, code: Symbol) {
        user.require_auth();

        let mut affiliates: Map<Address, Symbol> =
            env.storage()
                .instance()
                .get(&symbol_short!("AFF"))
                .unwrap_or(Map::new(&env));

        // Prevent duplicate registration
        if affiliates.contains_key(user.clone()) {
            panic!("Already registered");
        }

        affiliates.set(user.clone(), code);

        env.storage().instance().set(&symbol_short!("AFF"), &affiliates);
    }

    // Get affiliate code of a user
    pub fn get_code(env: Env, user: Address) -> Option<Symbol> {
        let affiliates: Map<Address, Symbol> =
            env.storage()
                .instance()
                .get(&symbol_short!("AFF"))
                .unwrap_or(Map::new(&env));

        affiliates.get(user)
    }

    // Record a referral (referrer → referee)
    pub fn record_referral(env: Env, referrer: Address, referee: Address) {
        referee.require_auth();

        let mut referrals: Map<Address, Address> =
            env.storage()
                .instance()
                .get(&symbol_short!("REF"))
                .unwrap_or(Map::new(&env));

        // Prevent overwriting existing referral
        if referrals.contains_key(referee.clone()) {
            panic!("Referral already exists");
        }

        referrals.set(referee.clone(), referrer.clone());

        env.storage().instance().set(&symbol_short!("REF"), &referrals);
    }

    // Get referrer of a user
    pub fn get_referrer(env: Env, user: Address) -> Option<Address> {
        let referrals: Map<Address, Address> =
            env.storage()
                .instance()
                .get(&symbol_short!("REF"))
                .unwrap_or(Map::new(&env));

        referrals.get(user)
    }
}