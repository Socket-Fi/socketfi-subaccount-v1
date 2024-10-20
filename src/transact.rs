use crate::data::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};
use soroban_sdk::{token, Address, Bytes, Env};

pub fn read_nonce(e: &Env) -> u32 {
    let key = DataKey::Nonce;
    if let Some(nonce) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        nonce
    } else {
        0
    }
}

pub fn increase_nonce(e: &Env, inc: u32) {
    let key = DataKey::Nonce;
    let new_nonce = read_nonce(e) + inc;
    e.storage().persistent().set(&key, &new_nonce);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn read_tx_nonce(e: &Env) -> Bytes {
    let key = DataKey::TxNonce;

    if let Some(nonce) = e.storage().instance().get(&key) {
        nonce
    } else {
        Bytes::new(&e)
    }
}

pub fn write_tx_nonce(e: &Env, new_nonce: Bytes) {
    let key = DataKey::TxNonce;
    e.storage().instance().set(&key, &new_nonce)
}

pub fn read_transact_no(e: &Env) -> u32 {
    let key = DataKey::TransactionCount;
    if let Some(count) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        count
    } else {
        0
    }
}

pub fn inc_transact_no(e: &Env) {
    let key = DataKey::TransactionCount;
    let new_count = read_transact_no(e) + 1;
    e.storage().persistent().set(&key, &new_count);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn take_token(env: &Env, from: &Address, token_id: &Address, amount: i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();

    token.transfer(from, &contract_address, &amount);
    inc_transact_no(env);
}

pub fn send_token(env: &Env, to: &Address, token_id: &Address, amount: i128) {
    let token = token::Client::new(env, token_id);
    let contract_address = env.current_contract_address();

    token.transfer(&contract_address, to, &amount);
    inc_transact_no(env);
}
