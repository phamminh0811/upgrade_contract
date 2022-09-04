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
    pub info: UnorderedMap<AccountId,UpgradeInfo>,
}


#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct CountContract{
    pub count_num: usize,
    pub balance_send: Balance,
    pub info: UnorderedMap<AccountId,UpgradeInfo>,
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

    pub fn add_info(&mut self, citizen_id: String,name: String, age: u8){
        let info = InformationV2 { name, age, citizen_id};

        self.info.insert(&env::predecessor_account_id(), &UpgradeInfo::from(info));
    }


    pub fn get_info(&self, account_id: AccountId) -> Option<InformationV2>{
        let upgrade_info = self.info.get(&account_id);
        if upgrade_info.is_none(){
            None
        } else {
            Some(InformationV2::from(upgrade_info.unwrap()))
        }
    }


    pub fn get_all_info(&self) -> Vec<InformationV2>{ 
        self.info
            .values_as_vector()
            .iter()
            .map(|upgrade_info| InformationV2::from(upgrade_info))
            .collect()
    }
}

impl CountContract{
    pub fn internal_increment_count(&mut self, count: usize){
        self.count_num += count;
    }
}
