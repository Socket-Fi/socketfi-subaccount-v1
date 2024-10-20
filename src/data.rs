use soroban_sdk::{contracttype, Address, String};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct ProfileEncrypted {
    pub platform: String,
    pub profile_id: String,
    pub gen_salt_encrypted: String,
    pub salt_encryption_iv: String,
    pub index_encrypted: String,
    pub index_encryption_iv: String,
}

#[derive(Clone)]
#[contracttype]
pub struct EncryptedKeys {
    pub gen_salt_encrypted: String,
    pub salt_encryption_iv: String,
    pub index_encrypted: String,
    pub index_encryption_iv: String,
}

#[derive(Clone)]
#[contracttype]
pub struct Token {
    pub token_id: Address,
    pub balance: i128,
    pub smart_transact: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
    PassKeyHash,
    ProfileKeys,
    Controller,
    MaxAllowance,
    Balance(Address),
    TokenIds(u32),
    SmartTransactActive(Address),
    Nonce,
    TxNonce,
    TokenAdded(Address),
    TokensCount,
    // UserPlatformId(u32),
    Initialized,
    Excecutors(u32),
    IsExecutor(Address),
    ExcecutorCount,
    ExecutorsSet,
    QuestPoints,
    TransactionCount,
}

//Stellar account pubkey: 0
// Email: 1
// X (Twitter): 2
// Discord: 3
// Telegram: 4
