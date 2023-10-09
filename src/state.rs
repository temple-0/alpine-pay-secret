use cosmwasm_std::{Addr, Timestamp, Deps, Storage, StdResult};
use cosmwasm_storage::{Singleton, singleton, ReadonlySingleton, singleton_read};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::error::ContractError;

const STATE_KEY: &[u8] = b"state";

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct State{
    pub donation_count: u64,
    pub users: Vec<AlpineUser>,
    pub donations: Vec<DonationInfo>
}

pub fn update_state(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, STATE_KEY)
}

pub fn read_state(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, STATE_KEY)
}

// Increment the number of donations. Only called during donation send
pub fn increment_donations(storage: &mut dyn Storage) -> StdResult<u64> {
    let mut state = read_state(storage).load()?;
    state.donation_count += 1;
    update_state(storage).save(&state)?;
    Ok(state.donation_count)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AlpineUser {
    pub username: String,
    pub address: Addr
}

impl AlpineUser {
    pub fn new(deps: Deps, address: Addr, username: Option<String>) -> Result<AlpineUser, ContractError> {
        let address = match deps.api.addr_validate(address.as_str()) {
            Ok(addr) => addr,
            Err(_) => return Err(ContractError::InvalidWalletAddress { address: address.to_string() })
        };
        
        let username = match username {
            Some(name) => name,
            None => String::from("")
        };
        
        Ok(AlpineUser { username, address })
    }

    pub fn empty() -> AlpineUser {
        AlpineUser { username: String::from(""), address: Addr::unchecked("") }
    }
}

pub fn find_alpine_username(storage: &dyn Storage, username: String) -> Result<AlpineUser, ContractError> {
    let state = read_state(storage).load()?;
    for user in state.users {
        if user.username == username {
            return Ok(user)
        }
    };
    Err(ContractError::UserNotFound { user: username.to_string() })
}

pub fn get_user_by_address(storage: &dyn Storage, address: Addr) -> Result<AlpineUser, ContractError> {
    let state = read_state(storage).load()?;
    for user in state.users{
        if user.address == address{
            return Ok(user)
        }
    };
    Err(ContractError::UserNotFound { user: address.to_string() })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DonationInfo {
    pub id: u64,
    pub sender: AlpineUser,
    pub recipient: AlpineUser,
    pub amount: Vec<cosmwasm_std::Coin>,
    pub message: String,
    pub timestamp: Option<Timestamp>
}

impl DonationInfo {

}

pub fn update_donations(storage: &mut dyn Storage, donation: DonationInfo) -> Result<DonationInfo, ContractError> {
    let mut state = read_state(storage).load()?;
    for dono in state.donations.clone(){
        if dono.id == donation.id{
            return Err(ContractError::Unauthorized {  })
        }
    }
    state.donations.append(&mut vec![donation.clone()]);
    state.donation_count += 1;
    update_state(storage).save(&state)?;
    Ok(donation)
}

pub fn donation_count(storage: &dyn Storage) -> StdResult<u64> {
    let state = read_state(storage).load()?;
    Ok(state.donation_count)
}

pub fn contains_username(storage: &dyn Storage, username: String) -> Result<bool, ContractError> {
    let state = read_state(storage).load()?;
    for user in state.users{
        if user.username.to_lowercase() == username.to_lowercase(){
            return Ok(true)
        }
    }
    return Ok(false)
}
