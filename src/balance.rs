use crate::data::{DataKey, BUMP_AMOUNT, LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn read_balance(e: &Env, token_id: Address) -> i128 {
    let key = DataKey::Balance(token_id);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        balance
    } else {
        0
    }
}

pub fn write_balance(e: &Env, token_id: Address, amount: i128) {
    let key = DataKey::Balance(token_id.clone());
    let cur_balance = read_balance(e, token_id.clone());
    let updated_balance = cur_balance + amount;
    e.storage().persistent().set(&key, &updated_balance);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}
