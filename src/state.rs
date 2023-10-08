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

// Define an Alpine user as a username and wallet address
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AlpineUser {
    pub username: String,
    pub address: Addr
}

impl AlpineUser {
    // Generate an Alpine user, defining their address and a blank username
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

    // Create an empty Alpine user
    pub fn empty() -> AlpineUser {
        AlpineUser { username: String::from(""), address: Addr::unchecked("") }
    }
}

// Return an AlpineUser from a query with the username
pub fn find_alpine_username(storage: &dyn Storage, username: String) -> Result<AlpineUser, ContractError> {
    let state = read_state(storage).load()?;
    for user in state.users{
        if user.username == username{
            return Ok(user)
        }
    };
    Err(ContractError::UserNotFound { user: username.to_string() })
}

// Get an Alpine user by their wallet address
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


// pub struct AlpineContract<'a> {
//     pub donation_count: Item<'a, u64>,
//     pub donations_sender: Keymap<'a, Addr, DonationInfo>,
//     pub donations_recipient: Keymap<'a, Addr, DonationInfo>,
//     // Create a data structure which maps registered usernames to user objects
//     pub usernames: Keymap<'a, String, AlpineUser>,
//     // Create a data structure which maps registered addresses to user objects
//     pub addresses: Keymap<'a, Addr, AlpineUser>
// }

// impl<'a> Donation for AlpineContract<'a> { }

// impl Default for AlpineContract<'static> {
//     fn default() -> Self {
//         Self::new(
//             "num_donations",
//             "donations_sender",
//             "donations_recipient",
//             "usernames",
//             addresses: "addresses"
//         )
//     }
// }

// impl<'a> AlpineContract<'a> {
//     // On contract instantiation, create all of the relevant data structures
//     fn new(
//         donation_count_key: &'a [u8],
//         donations_sender: &'a [u8],
//         donations_recipient: &'a [u8],
//         usernames: &'a [u8],
//         addresses: &'a [u8]
//     ) -> Self {
//         Self {
//             donation_count: Item::new(donation_count_key),
//             donations_sender: Keymap::new(donations_sender),
//             donations_recipient: Keymap::new(donations_recipient),
//             usernames: Keymap::new(usernames),
//             addresses: Keymap::new(addresses),
//         }
//     }

// Return the number of donations
pub fn donation_count(storage: &dyn Storage) -> StdResult<u64> {
    let state = read_state(storage).load()?;
    Ok(state.donation_count)
}

// Check if a username is taken regardless of username casing
pub fn contains_username(storage: &dyn Storage, username: String) -> Result<bool, ContractError> {
    let state = read_state(storage).load()?;
    for user in state.users{
        if user.username.to_lowercase() == username.to_lowercase(){
            return Ok(true)
        }
    }
    return Ok(false)
}
