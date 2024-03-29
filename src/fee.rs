use soroban_sdk::{ Env };
use crate::storage_types::{ FEE_DECIMALS, INSTANCE_BUMP_AMOUNT, DataKey, FeeInfo };


pub fn fee_check(e: &Env) -> bool {
    let key = DataKey::FEE;

    if e.storage().instance().has(&key) {
        true
    }
    else {
        false
    }
}

pub fn fee_get(e: &Env) -> FeeInfo {
    let key = DataKey::FEE;

    if !e.storage().instance().has(&key) {
        panic!("FeeInfo wasn't initialized");
    }
    
    e.storage().instance().get(&key).unwrap()
}

pub fn fee_set(e: &Env, fee_info: &FeeInfo) {
    let key = DataKey::FEE;

    e.storage().instance().set(&key, fee_info);
    e.storage().instance().bump(INSTANCE_BUMP_AMOUNT, INSTANCE_BUMP_AMOUNT);
}

pub fn calculate_fee(_e: &Env, fee_info: &FeeInfo, amount: u64) -> u64 {
    amount * (fee_info.fee_rate as u64) / (u64::pow(10, FEE_DECIMALS))
}
