use cosmwasm_std::{Addr, Timestamp, Deps, Storage, StdResult, Response};
use cosmwasm_storage::{Singleton, singleton, ReadonlySingleton, singleton_read};
use schemars::JsonSchema;
use secret_toolkit_storage::Keymap;
use serde::{Serialize, Deserialize};

use crate::error::ContractError;

const STATE_KEY: &[u8] = b"state";
const DONATION_COUNT_KEY: &[u8] = b"donation_count";    // todo: do we need this?
const DONATIONS_KEY: &[u8] = b"donations";
const USERNAMES_KEY: &[u8] = b"usernames";
const ADDRESSES_KEY: &[u8] = b"addresses";
pub static DONATIONS: Keymap<u64, DonationInfo> = Keymap::new(DONATIONS_KEY);
// Create a data structure which maps registered usernames to user objects
pub static USERNAMES: Keymap<String, AlpineUser> = Keymap::new(USERNAMES_KEY);
// Create a data structure which maps registered addresses to user objects
pub static ADDRESSES: Keymap<Addr, AlpineUser>  = Keymap::new(ADDRESSES_KEY);

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct State{
    pub donation_count: u64,
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

    let alpine_user = match USERNAMES.get(storage, &username) {
        Some(user) => user,
        None => return Err(ContractError::UserNotFound { user: username })
    };

    Ok(alpine_user)
}

// Get an Alpine user by their wallet address
pub fn get_user_by_address(storage: &dyn Storage, address: Addr) -> Result<AlpineUser, ContractError> {
    let state = read_state(storage).load()?;

    let alpine_user = match ADDRESSES.get(storage, &address) {
        Some(user) => user,
        None => return Err(ContractError::UserNotFound { user: address.to_string() })
    };

    Ok(alpine_user)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DonationInfo {
    pub sender: AlpineUser,
    pub recipient: AlpineUser,
    pub amount: Vec<cosmwasm_std::Coin>,
    pub message: String,
    pub timestamp: Option<Timestamp>
}

impl DonationInfo {

}

pub fn update_donations(storage: &mut dyn Storage, donation: DonationInfo, id: u64) -> Result<DonationInfo, ContractError> {
    match DONATIONS.get(storage, &id){
        Some(_) => { DONATIONS.insert(storage, &id, &donation); }
        None => return Err(ContractError::Unauthorized {  })
    }
    update_state(storage);
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
    let usernames = USERNAMES.paging_keys(
        storage,
        0,
        u32::MAX,
    )?;
    let search_result: bool = usernames.contains(&username.to_lowercase());

    Ok(search_result)
}
