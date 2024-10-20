use soroban_sdk::Env;

use crate::data::DataKey;
use crate::types::UserPoints;

pub fn read_quest_data(e: &Env) -> UserPoints {
    let key = DataKey::QuestPoints;
    e.storage().instance().get(&key).unwrap()
}

pub fn write_quest_data(e: &Env, data: UserPoints) {
    let key = DataKey::QuestPoints;

    e.storage().instance().set(&key, &data);
}
