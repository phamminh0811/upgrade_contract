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
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct InformationV3 {
    pub name: String,
    pub age: u8,
    pub citizen_id: String,
    pub country_name: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub enum UpgradeInfo{
    V1(InformationV1),
    V2(InformationV2),
    V3(InformationV3),
}

impl From<InformationV3> for UpgradeInfo {
    fn from(info: InformationV3) -> UpgradeInfo {
        UpgradeInfo::V3(info)
    }
}

impl From<UpgradeInfo> for InformationV3 {
    fn from(info: UpgradeInfo) -> InformationV3 {
        match info {
            UpgradeInfo::V3(info) => info,
            UpgradeInfo::V2(info) => InformationV3{
                name: info.name,
                age: info.age,
                citizen_id: info.citizen_id,
                country_name: "Lao".to_string(),
            },
            UpgradeInfo::V1(info) => InformationV3 { 
                name: info.name, 
                age: info.age,
                citizen_id: "Not have yet".to_string(),
                country_name: "Viet Nam".to_string()
            }
        }
    }
}