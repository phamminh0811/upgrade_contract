use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InformationV1 {
    pub name: String,
    pub age: u8
}


#[derive(BorshDeserialize, BorshSerialize)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InformationV2 {
    pub name: String,
    pub age: u8,
    pub citizen_id: String
}


#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradeInfo{
    V1(InformationV1),
    V2(InformationV2),
}

impl From<InformationV2> for UpgradeInfo {
    fn from(info: InformationV2) -> UpgradeInfo {
        UpgradeInfo::V2(info)
    }
}

impl From<UpgradeInfo> for InformationV2 {
    fn from(info: UpgradeInfo) -> InformationV2 {
        match info {
            UpgradeInfo::V2(info) => info,
            UpgradeInfo::V1(info) => InformationV2 { 
                name: info.name, 
                age: info.age,
                citizen_id: "Not have yet".to_string(),
            }
        }
    }
}