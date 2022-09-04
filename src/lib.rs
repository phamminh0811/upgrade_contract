use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen,env, Balance, PanicOnDefault, AccountId,BorshStorageKey};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::U128;

use info::*;
mod info;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    List
}



#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct CountContractV1{
    pub count_num: usize,
    pub balance_send: Balance,
    pub info: UnorderedMap<AccountId,InformationV1>,
}


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct CountContract{
    pub count_num: usize,
    pub balance_send: Balance,
    pub info: UnorderedMap<AccountId,InformationV1>,
    pub new_data: U128
}

#[near_bindgen]
impl CountContract{
    #[init]
    pub fn new() -> Self{
        Self{
            count_num: 0,
            balance_send: 0,
            info: UnorderedMap::new(StorageKeys::List),
            new_data: U128(0)
        }
    }

    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let contract_v1 : CountContractV1 = env::state_read().expect("Can't read state");
        Self {
            count_num : contract_v1.count_num,
            balance_send: contract_v1.balance_send,
            info: contract_v1.info,
            new_data: U128(0)
        }
    }
    
    pub fn increment_count(&mut self, count: usize) {
        self.internal_increment_count(count);
    }

    pub fn get_count(&self) -> usize {
        self.count_num
    }

    #[payable]
    pub fn deposit(&mut self) {
        let deposit: Balance = env::attached_deposit();
        self.balance_send += deposit;
    }

    pub fn add_info(&mut self,name: String, age: u8){
        let info = InformationV1 { name, age};

        self.info.insert(&env::predecessor_account_id(), &info);
    }


    pub fn get_info(&self, account_id: AccountId) -> Option<InformationV1>{
        self.info.get(&account_id)
    }


    pub fn get_all_info(&self) -> Vec<InformationV1>{ 
        self.info
            .values_as_vector()
            .to_vec()
    }
}

impl CountContract{
    pub fn internal_increment_count(&mut self, count: usize){
        self.count_num += count;
    }
}
