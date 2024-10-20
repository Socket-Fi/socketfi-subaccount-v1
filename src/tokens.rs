use soroban_sdk::{Address, Env, Vec};

// use crate::rates::read_sale_rate;
use crate::{
    balance::read_balance,
    data::{DataKey, Token, BUMP_AMOUNT, LIFETIME_THRESHOLD},
};

pub fn read_has_been_added(e: &Env, token_id: Address) -> bool {
    let key = DataKey::TokenAdded(token_id);
    e.storage().instance().get(&key).unwrap_or(false)
}

pub fn read_smart_transact_active(e: &Env, token_id: Address) -> bool {
    let key = DataKey::SmartTransactActive(token_id);
    e.storage().instance().get(&key).unwrap_or(false)
}

pub fn write_smart_transact_active(e: &Env, token_id: Address, activate_disable: bool) {
    let key = DataKey::SmartTransactActive(token_id);
    e.storage().instance().set(&key, &activate_disable);
}

pub fn read_token_count(e: &Env) -> u32 {
    let key = DataKey::TokensCount;
    if let Some(count) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        count
    } else {
        0
    }
}

pub fn write_token_count(e: &Env, new_count: u32) {
    let key = DataKey::TokensCount;
    e.storage().persistent().set(&key, &new_count);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn read_tokens(e: &Env) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new(&e);
    let token_count = read_token_count(e);
    for index in 1..=token_count {
        let key = DataKey::TokenIds(index);
        let token: Address = e.storage().instance().get(&key).unwrap();
        let balance = read_balance(e, token.clone());
        let smart_transact_status = read_smart_transact_active(e, token.clone());

        if balance > 0 {
            let token_info = Token {
                token_id: token,
                balance: balance,
                smart_transact: smart_transact_status,
            };

            tokens.push_back(token_info);
        }
    }
    tokens
}

pub fn save_token_id(e: &Env, index: u32, token_id: Address) {
    let key = DataKey::TokenIds(index);
    e.storage().instance().set(&key, &token_id);
}
