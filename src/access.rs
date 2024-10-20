use soroban_sdk::{Address, Env, String};

use crate::data::{DataKey, EncryptedKeys, ProfileEncrypted, BUMP_AMOUNT, LIFETIME_THRESHOLD};
//Smart wallet sub account owner
pub fn read_is_initialized(e: &Env) -> bool {
    let key = DataKey::Initialized;
    if let Some(flag) = e.storage().instance().get(&key) {
        flag
    } else {
        false
    }
}

pub fn write_is_initialized(e: &Env) {
    let key = DataKey::Initialized;
    e.storage().instance().set(&key, &true);
}

pub fn read_executors_set(e: &Env) -> bool {
    let key = DataKey::ExecutorsSet;
    if let Some(flag) = e.storage().instance().get(&key) {
        flag
    } else {
        false
    }
}

pub fn write_executors_set(e: &Env) {
    let key = DataKey::ExecutorsSet;
    e.storage().instance().set(&key, &true);
}

pub fn read_excecutor_count(e: &Env) -> u32 {
    let key = DataKey::ExcecutorCount;
    if let Some(count) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage()
            .persistent()
            .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
        count
    } else {
        0
    }
}

pub fn write_excecutor_count(e: &Env, new_count: u32) {
    let key = DataKey::ExcecutorCount;
    e.storage().persistent().set(&key, &new_count);
    e.storage()
        .persistent()
        .extend_ttl(&key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
}

pub fn write_executor(e: &Env, index: u32, executor_pub_key: Address) {
    let key = DataKey::Excecutors(index);
    e.storage().instance().set(&key, &executor_pub_key);
    let is_key = DataKey::IsExecutor(executor_pub_key.clone());
    e.storage().instance().set(&is_key, &true);
    write_excecutor_count(e, index);
}

pub fn read_executor(e: &Env, index: u32) -> Address {
    let key = DataKey::Excecutors(index);
    e.storage().instance().get(&key).unwrap()
}

pub fn read_is_executor(e: &Env, caller_id: Address) -> bool {
    let key = DataKey::IsExecutor(caller_id);
    if let Some(flag) = e.storage().instance().get(&key) {
        flag
    } else {
        false
    }
}

pub fn has_owner(e: &Env) -> bool {
    let key = DataKey::Owner;
    e.storage().instance().has(&key)
}

pub fn read_owner(e: &Env) -> Option<Address> {
    let key = DataKey::Owner;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_owner(e: &Env, owner_id: &Address) {
    let key = DataKey::Owner;
    e.storage().instance().set(&key, owner_id);
}

pub fn read_profile_encrypted(e: &Env) -> ProfileEncrypted {
    let key = DataKey::ProfileKeys;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_profile_encrypted(
    e: &Env,
    platform: String,
    profile_id: String,
    salt: String,
    salt_iv: String,
    key_index: String,
    index_iv: String,
) {
    let key = DataKey::ProfileKeys;
    let user_profile_encrypted = ProfileEncrypted {
        platform: platform,
        profile_id: profile_id,
        gen_salt_encrypted: salt,
        salt_encryption_iv: salt_iv,
        index_encrypted: key_index,
        index_encryption_iv: index_iv,
    };
    e.storage().instance().set(&key, &user_profile_encrypted);
}

pub fn write_passkey_hash(e: &Env, passkey_hash: String) {
    let key = DataKey::PassKeyHash;
    e.storage().instance().set(&key, &passkey_hash);
}

pub fn read_encrypted_keys(e: &Env, profile_id: String) -> EncryptedKeys {
    let profile_encrypted = read_profile_encrypted(e);
    if profile_id != profile_encrypted.profile_id {
        panic!("Profile id is not a match")
    }
    let encrypted_keys = EncryptedKeys {
        gen_salt_encrypted: profile_encrypted.gen_salt_encrypted,
        salt_encryption_iv: profile_encrypted.salt_encryption_iv,
        index_encrypted: profile_encrypted.index_encrypted,
        index_encryption_iv: profile_encrypted.index_encryption_iv,
    };
    encrypted_keys
}

pub fn check_zk_validation(e: &Env, entered_passkey_hash: String) -> bool {
    let key = DataKey::PassKeyHash;
    let stored_passkey_hash = e.storage().instance().get(&key).unwrap();
    entered_passkey_hash == stored_passkey_hash
}

pub fn read_max_allowance(e: &Env) -> i128 {
    let key = DataKey::MaxAllowance;
    if let Some(allowance) = e.storage().instance().get(&key).unwrap() {
        allowance
    } else {
        0
    }
}

pub fn write_max_allowance(e: &Env, allowance: i128) {
    let key = DataKey::MaxAllowance;
    e.storage().instance().set(&key, &allowance)
}

//Smart wallet logic contract

pub fn read_controller(e: &Env) -> Address {
    let key = DataKey::Controller;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_controller(e: &Env, controller_id: &Address) {
    let key = DataKey::Controller;
    e.storage().instance().set(&key, controller_id);
}
