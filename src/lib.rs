use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{TreeMap};
use near_sdk::{near_bindgen, AccountId, env};
use serde::{Serialize, Deserialize};
use serde_json::json;
use near_sdk::json_types::U128;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct CollectionIssues {
    treemap_num: TreeMap<u128, String>,
    treemap: TreeMap<String, String>,
    nested_treemap: TreeMap<AccountId, TreeMap<u128, String>>,
}

impl Default for CollectionIssues {
    fn default() -> Self {
        panic!("Initialize before usage, please.")
    }
}

#[near_bindgen]
impl CollectionIssues {
    #[init]
    pub fn new() -> Self {
        Self {
            treemap_num: TreeMap::new(b"m".to_vec()),
            treemap: TreeMap::new(b"t".to_vec()),
            nested_treemap: TreeMap::new(b"n".to_vec())
        }
    }

    pub fn insert_treemap_string(&mut self, key: String, val: String) {
        self.treemap.insert(&key, &val);
    }

    pub fn remove_treemap_string(&mut self, key: String) {
        self.treemap.remove(&key);
    }

    pub fn insert_treemap_num(&mut self, key: U128, val: String) {
        self.treemap_num.insert(&key.into(), &val);
    }

    pub fn remove_treemap_num(&mut self, key: U128) {
        self.treemap_num.remove(&key.into());
    }

    pub fn insert_nested_treemap_num(&mut self, key: U128, val: String) {
        let key_u128: u128 = key.into();
        let sender = env::predecessor_account_id();
        let inner_treemap_option = self.nested_treemap.get(&sender);
        let mut inner_treemap = if inner_treemap_option.is_none() {
            TreeMap::new(sender.clone().into_bytes())
        } else {
            inner_treemap_option.unwrap()
        };
        inner_treemap.insert(&key_u128, &val);
        self.nested_treemap.insert(&sender.clone(), &inner_treemap);
    }

    pub fn remove_nested_treemap_num(&mut self, key: U128) {
        let key_u128: u128 = key.into();
        let sender = env::predecessor_account_id();
        let inner_treemap_option = self.nested_treemap.get(&sender);
        let mut inner_treemap = if inner_treemap_option.is_none() {
            TreeMap::new(sender.clone().into_bytes())
        } else {
            inner_treemap_option.unwrap()
        };
        if inner_treemap.contains_key(&key_u128) {
            inner_treemap.remove(&key_u128);
            self.nested_treemap.insert(&sender.clone(), &inner_treemap);
        } else {
            env::panic(b"didn't find number to remove");
        }
    }

    pub fn remove_nested_treemap_me(&mut self) {
        let sender = env::predecessor_account_id();
        if self.nested_treemap.contains_key(&sender) {
            self.nested_treemap.remove(&sender);
        } else {
            env::panic(b"didn't find user to remove");
        }
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    fn get_context() -> VMContext {
        VMContext {
            current_account_id: "hardcoded".to_string(),
            signer_account_id: "hardcoded".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "hardcoded".to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 1_000_000_000_000_000_000_000_000_000u128,
            account_locked_balance: 0,
            storage_usage: 10u64.pow(6),
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn test_double_add() {
        let context = get_context();
        testing_env!(context);
        let mut contract = CollectionIssues::new();
        contract.insert_treemap_string("sasha".to_string(), "services".to_string());
        contract.insert_treemap_string("sasha".to_string(), "dervishes".to_string());
    }

    #[test]
    fn test_add_remove_add() {
        let context = get_context();
        testing_env!(context);
        let mut contract = CollectionIssues::new();
        contract.insert_treemap_string("sasha".to_string(), "services".to_string());
        contract.remove_treemap_string("sasha".to_string());
        contract.insert_treemap_string("sasha".to_string(), "dervishes".to_string());
    }


    #[test]
    fn test_break_nested() {
        let context = get_context();
        testing_env!(context);
        let mut contract = CollectionIssues::new();
        contract.insert_nested_treemap_num(U128(19), "aloha".to_string());
        contract.remove_nested_treemap_me();
        contract.insert_nested_treemap_num(U128(19), "aloha".to_string());
    }
}