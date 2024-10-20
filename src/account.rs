use soroban_sdk::{contract, contractimpl, xdr::ToXdr, Address, Bytes, BytesN, Env, String, Vec};

use crate::{
    access::{
        check_zk_validation, has_owner, read_encrypted_keys, read_executor, read_executors_set,
        read_is_executor, read_is_initialized, read_max_allowance, read_owner, write_controller,
        write_executor, write_executors_set, write_is_initialized, write_max_allowance,
        write_owner, write_passkey_hash, write_profile_encrypted,
    },
    balance::{read_balance, write_balance},
    data::{DataKey, EncryptedKeys, Token},
    tokens::{
        read_has_been_added, read_token_count, read_tokens, save_token_id,
        write_smart_transact_active, write_token_count,
    },
    transact::{
        increase_nonce, read_nonce, read_transact_no, read_tx_nonce, send_token, take_token,
        write_tx_nonce,
    },
    types::UserPoints,
    user_quest::{read_quest_data, write_quest_data},
};

pub trait SubAccountTrait {
    fn init_with_address(e: Env, controller_id: Address, owner_id: Address);
    fn init_with_profile(
        e: Env,
        controller_id: Address,
        platform: String,
        profile_id: String,
        salt: String,
        salt_iv: String,
        key_index: String,
        index_iv: String,
        passkey_hash: String,
        max_allowance: i128,
    );
    fn set_executor(e: Env, index: u32, executor: Address);
    fn set_executor_done(e: Env);
    fn update_user_points(e: Env, caller: Address, user_data: UserPoints);
    fn set_allowance_with_addr(e: Env, allowance: i128);
    fn set_allowance_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        allowance: i128,
    );
    fn set_owner_pkey(e: Env, executor_index: u32, entered_passkey_hash: String, owner_id: Address);
    fn receive(e: Env, from: Address, token_id: Address, amount: i128);
    fn send_auth_addr(e: Env, to: Address, token_id: Address, amount: i128);
    fn send_with_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        to: Address,
        token_id: Address,
        amount: i128,
    );
    fn set_smart_transact_addr(e: Env, token_id: Address, activate_disable: bool);
    fn set_smart_transact_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        token_id: Address,
        activate_disable: bool,
    );
    fn create_tx_nonce(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        spender: Address,
        token_id: Address,
        amount: i128,
    );
    fn set_dashboard_balance(e: Env, caller: Address, token_id: Address, amount: i128);
    fn clear_tx_nonce(e: Env);
    fn get_tx_count(e: Env) -> u32;
    fn get_owner(e: Env) -> Address;
    fn get_encrypted_keys(e: Env, profile_id: String) -> EncryptedKeys;
    fn get_executor(e: Env, index: u32) -> Address;
    fn get_tokens(e: Env) -> Vec<Token>;
    fn get_balance(e: Env, token_id: Address) -> i128;
    fn get_nonce(e: Env) -> u32;
    fn get_tx_nonce(e: Env) -> Bytes;
    fn get_user_points(e: Env) -> UserPoints;
    fn get_allowance(e: Env) -> i128;
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}

#[contract]
pub struct SubAccount;

#[contractimpl]
impl SubAccountTrait for SubAccount {
    //Initialization of smart wallet when it is created using an external account
    fn init_with_address(e: Env, controller_id: Address, owner_id: Address) {
        let is_initialized = read_is_initialized(&e);

        if is_initialized {
            panic!("has already been initilized")
        }
        write_owner(&e, &owner_id);
        write_controller(&e, &controller_id);

        let user_points = UserPoints {
            has_set_signer: false,
            has_set_allowance: false,
            has_received: false,
            has_sent: false,
            points: 2000,
        };

        write_quest_data(&e, user_points);

        write_is_initialized(&e)
    }
    //Initialization of smart wallet when it is created using social media credentials

    fn init_with_profile(
        e: Env,
        controller_id: Address,
        platform: String,
        profile_id: String,
        salt: String,
        salt_iv: String,
        key_index: String,
        index_iv: String,
        passkey_hash: String,
        max_allowance: i128,
    ) {
        let is_initialized = read_is_initialized(&e);

        if is_initialized {
            panic!("has already been initilized")
        }
        // write_id(&e, platform, profile_id, encrypted_token, encrypted_index);
        write_controller(&e, &controller_id);
        write_passkey_hash(&e, passkey_hash);
        write_profile_encrypted(&e, platform, profile_id, salt, salt_iv, key_index, index_iv);
        write_max_allowance(&e, max_allowance);

        //initialize point data, creating account with twitter earns 2000 points
        let user_points = UserPoints {
            has_set_signer: false,
            has_set_allowance: false,
            has_received: false,
            has_sent: false,
            points: 2000,
        };

        write_quest_data(&e, user_points);

        write_is_initialized(&e)
    }

    //Sets the authorized executors, called during account creation only

    fn set_executor(e: Env, index: u32, executor: Address) {
        let executors_set = read_executors_set(&e);

        if executors_set {
            panic!("Executors have already been set")
        }
        write_executor(&e, index, executor)
    }

    //Set to true when executors have been set during creation of account
    //Prevent additional executors from be set after account creation

    fn set_executor_done(e: Env) {
        let executors_set = read_executors_set(&e);
        if executors_set {
            panic!("Executors have already been set and flagged as done.")
        }
        write_executors_set(&e)
    }

    //Allowance limit the amount of funds the smart wallet controller contract can access
    //For programed transactions

    //This set allowance using the external account used to create the smart account
    //only the owner(wallet that created the account) can invoke this
    fn set_allowance_with_addr(e: Env, allowance: i128) {
        let owner = read_owner(&e).expect("No owner address found");
        owner.require_auth();
        write_max_allowance(&e, allowance);
    }

    //This set allowance using the social media auth credentials that were used to create the smart account
    //only authorized executors can sign this invocation
    fn set_allowance_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        allowance: i128,
    ) {
        let authorized = check_zk_validation(&e, entered_passkey_hash);
        if !authorized {
            panic!("Not authorized to invoke this function")
        }

        let executor = read_executor(&e, executor_index);
        executor.require_auth();
        write_max_allowance(&e, allowance);
        let mut user_data = read_quest_data(&e);
        user_data.has_set_allowance = true;
        user_data.points += 500;
        write_quest_data(&e, user_data);
    }

    //set the owners external wallet for a smart account created with twitter

    fn set_owner_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        owner_id: Address,
    ) {
        let executor = read_executor(&e, executor_index);
        executor.require_auth();
        let has_owner = has_owner(&e);
        if has_owner {
            panic!("Owner has already been set")
        }

        let authorized = check_zk_validation(&e, entered_passkey_hash);
        if !authorized {
            panic!("Not authorized to invoke this function")
        }

        write_owner(&e, &owner_id);
        let mut user_data = read_quest_data(&e);
        user_data.has_set_signer = true;
        user_data.points += 500;
        write_quest_data(&e, user_data);
    }

    //This allow users to send funds from an external wallet to the smart wallet.
    // Receive is in respect to the smart wallet (funds are received by the smart wallet)

    fn receive(e: Env, from: Address, token_id: Address, amount: i128) {
        from.require_auth();
        take_token(&e, &from, &token_id, amount);
        write_balance(&e, token_id.clone(), amount);
        let has_been_added = read_has_been_added(&e, token_id.clone());
        if !has_been_added {
            let key = DataKey::TokenAdded(token_id.clone());
            e.storage().instance().set(&key, &true);
            let new_count = read_token_count(&e) + 1;
            save_token_id(&e, new_count, token_id);
            write_token_count(&e, new_count)
        }

        let mut user_data = read_quest_data(&e);
        user_data.has_received = true;
        user_data.points += 250;
        write_quest_data(&e, user_data);
    }

    fn set_dashboard_balance(e: Env, caller: Address, token_id: Address, amount: i128) {
        let is_executor = read_is_executor(&e, caller);

        if is_executor == false {
            panic!("caller is not an executor")
        }

        write_balance(&e, token_id.clone(), amount);
        let has_been_added = read_has_been_added(&e, token_id.clone());
        if !has_been_added {
            let key = DataKey::TokenAdded(token_id.clone());
            e.storage().instance().set(&key, &true);
            let new_count = read_token_count(&e) + 1;
            save_token_id(&e, new_count, token_id);
            write_token_count(&e, new_count)
        }
    }

    //Updates users quest data and onchain activities points

    fn update_user_points(e: Env, caller: Address, user_data: UserPoints) {
        let is_executor = read_is_executor(&e, caller);

        if is_executor == false {
            panic!("caller is not an executor")
        }

        write_quest_data(&e, user_data);
    }

    //this allow the owner (external account that created the wallet) to send funds from the smart wallet
    //to an external account

    fn send_auth_addr(e: Env, to: Address, token_id: Address, amount: i128) {
        let owner = read_owner(&e).expect("Owner not set");
        owner.require_auth();
        send_token(&e, &to, &token_id, amount);
        write_balance(&e, token_id, -amount)
    }

    //this allow the owner to send funds from the smart wallet, the owner authenticate using
    //same social auth credentials used to create the account.
    //Only authorized excutor can sign or approve the transaction.
    fn send_with_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        to: Address,
        token_id: Address,
        amount: i128,
    ) {
        let authorized = check_zk_validation(&e, entered_passkey_hash);
        if !authorized {
            panic!("Not authorized to invoke this function")
        }
        let allowance = read_max_allowance(&e);
        if amount > allowance {
            panic!("You cannot send an amount greater than your allowance")
        }
        let caller_id = read_executor(&e, executor_index);
        let is_executor = read_is_executor(&e, caller_id);

        if is_executor == false {
            panic!("Caller is not an executor")
        }

        write_balance(&e, token_id.clone(), -amount);
        send_token(&e, &to, &token_id, amount);
    }

    //This allows the owner to enable smart transaction for a token with balance greater than zero
    //for this, the owner is the extenal account that created it

    fn set_smart_transact_addr(e: Env, token_id: Address, activate_disable: bool) {
        let owner = read_owner(&e).expect("Owner not set");
        owner.require_auth();
        write_smart_transact_active(&e, token_id, activate_disable);
    }

    //This allows the owner to enable smart transaction for a token with balance greater than zero
    //for this, the owner authenticate using social credentials

    fn set_smart_transact_pkey(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        token_id: Address,
        activate_disable: bool,
    ) {
        let authorized = check_zk_validation(&e, entered_passkey_hash);
        if !authorized {
            panic!("Not authorized to invoke this function")
        }

        let executor = read_executor(&e, executor_index);
        executor.require_auth();
        write_smart_transact_active(&e, token_id, activate_disable);
    }

    // this must run before any transaction can run
    fn create_tx_nonce(
        e: Env,
        executor_index: u32,
        entered_passkey_hash: String,
        spender: Address,
        token_id: Address,
        amount: i128,
    ) {
        let authorized = check_zk_validation(&e, entered_passkey_hash.clone());
        if !authorized {
            panic!("Not authorized to invoke this function")
        }

        let executor = read_executor(&e, executor_index);
        executor.require_auth();
        let pre_nonce = read_nonce(&e);
        let seq_nonce = pre_nonce + 1;
        increase_nonce(&e, 1);
        let mut salt = Bytes::new(&e);
        salt.append(&executor.to_xdr(&e));
        salt.append(&seq_nonce.to_xdr(&e));
        salt.append(&entered_passkey_hash.to_xdr(&e));
        salt.append(&spender.to_xdr(&e));
        salt.append(&token_id.to_xdr(&e));
        salt.append(&amount.to_xdr(&e));
        let new_nonce = e.crypto().sha256(&salt).to_xdr(&e);
        write_tx_nonce(&e, new_nonce);
    }

    fn clear_tx_nonce(e: Env) {
        let nonce = Bytes::new(&e);
        write_tx_nonce(&e, nonce);
    }

    fn get_tx_count(e: Env) -> u32 {
        read_transact_no(&e)
    }

    //Gets the smart account owner
    fn get_owner(e: Env) -> Address {
        read_owner(&e).expect("Owner not found!")
    }

    //Gets encrypted keys needed for validation
    fn get_encrypted_keys(e: Env, profile_id: String) -> EncryptedKeys {
        read_encrypted_keys(&e, profile_id)
    }

    //Gets the executor selected
    fn get_executor(e: Env, index: u32) -> Address {
        read_executor(&e, index)
    }
    //get all tokens with balance greater than zeor

    fn get_tokens(e: Env) -> Vec<Token> {
        read_tokens(&e)
    }

    //get the balance of a specific token

    fn get_balance(e: Env, token_id: Address) -> i128 {
        read_balance(&e, token_id)
    }

    //Get transaction nonce
    fn get_nonce(e: Env) -> u32 {
        read_nonce(&e)
    }

    fn get_tx_nonce(e: Env) -> Bytes {
        read_tx_nonce(&e)
    }

    fn get_user_points(e: Env) -> UserPoints {
        read_quest_data(&e)
    }

    fn get_allowance(e: Env) -> i128 {
        read_max_allowance(&e)
    }

    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let owner = read_owner(&e).unwrap();
        owner.require_auth();
        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}
