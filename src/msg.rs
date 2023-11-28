use schemars::JsonSchema;
use secret_toolkit_permit::Permit;
use serde::{Deserialize, Serialize};
use crate::state::{DonationInfo, AlpineUser};
use cosmwasm_std::Addr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg { }

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SendDonation { sender: String, recipient: String, message: String },
    RegisterUser { user: AlpineUser, username: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermitMsg {
    GetSentDonations{ sender: String },
    GetReceivedDonations { recipient: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetDonationCount {  },
    IsUsernameAvailable { username: String },
    GetAllUsers { },
    GetUserByAddr { address: Addr },
    GetUserByName { username: String },
    WithPermit { permit: Permit, query: QueryWithPermitMsg }
}

// Return a list of donation IDs mapped to the data stored in the donation
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiDonationResponse {
    pub donations: Vec<DonationInfo>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MultiUserResponse {
    pub users: Vec<AlpineUser>
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DonationCountResponse {
    pub count: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UsernameAvailableResponse {
    pub is_available: bool
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AlpineUserResponse {
    pub user: AlpineUser,
}