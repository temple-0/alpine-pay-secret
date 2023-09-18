use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    Deps,
    Addr, 
    Storage, 
    StdResult,
    Timestamp,
};
use secret_toolkit_storage::{
  Item, 
  Keymap,
};

use crate::traits::Donation;
use crate::error::ContractError;

pub struct AlpineContract<'a> {
    pub donation_count: Item<'a, u64>,
    pub donations_sender: Keymap<'a, Addr, DonationInfo>,
    pub donations_recipient: Keymap<'a, Addr, DonationInfo>,
    // Create a data structure which maps registered usernames to user objects
    pub usernames: Keymap<'a, String, AlpineUser>,
    // Create a data structure which maps registered addresses to user objects
    pub addresses: Keymap<'a, Addr, AlpineUser>
}

impl<'a> Donation for AlpineContract<'a> { }

impl Default for AlpineContract<'static> {
    fn default() -> Self {
        Self::new(
            "num_donations",
            "donations_sender",
            "donations_recipient",
            "usernames",
            addresses: "addresses"
        )
    }
}

impl<'a> AlpineContract<'a> {
    // On contract instantiation, create all of the relevant data structures
    fn new(
        donation_count_key: &'a str,
        donations_sender: &'a str,
        donations_recipient: &'a str,
        usernames: &'a str,
        addresses: &'a str
    ) -> Self {
        Self {
            donation_count: Item::new(donation_count_key),
            donations_sender: Keymap::new(donations_sender),
            donations_recipient: Keymap::new(donations_recipient),
            usernames: Keymap::new(usernames),
            addresses: Keymap::new(addresses),
        }
    }

    // Return the number of donations
    pub fn donation_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.donation_count.may_load(storage)?.unwrap_or_default())
    }

    // Increment the number of donations. Only called during donation send
    pub fn increment_donations(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.donation_count(storage)? + 1;
        self.donation_count.save(storage, &val)?;
        Ok(val)
    }

    // Return an AlpineUser from a query with the username
    pub fn find_alpine_username(&self, storage: &dyn Storage, username: String) -> Result<AlpineUser, ContractError> {
        let mut usernames = self.usernames.paging_keys(
            storage,
            0,
            u32::MAX,
        )?;
        let found = match usernames.iter().any(|&u| u == username){
            true => username,
            false => String::from("")
        };

        // Pull the Alpine user for the designated username
        let alpine_user = match self.usernames.get(storage, &found.clone()) {
            Some(user) => user,
            None => return Err(ContractError::UserNotFound { user: username })
        };

        Ok(alpine_user)
    }

    // Check if a username is taken regardless of username casing
    pub fn contains_username(&self, storage: &dyn Storage, username: String) -> Result<bool, ContractError> {
        let usernames = self.usernames.paging_keys(
            storage,
            0,
            u32::MAX,
        )?;
        let search_result: bool = usernames.contains(&username.to_lowercase());

        Ok(search_result)
    }

    // Get an Alpine user by their wallet address
    pub fn get_user_by_address(&self, storage: &dyn Storage, address: Addr) -> Result<AlpineUser, ContractError> {
        let alpine_user = match self.addresses.get(storage, &address.clone()) {
            Some(user) => user,
            None => return Err(ContractError::UserNotFound { user: address.to_string() })
        };

        Ok(alpine_user)
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DonationInfo {
    pub sender: AlpineUser,
    pub recipient: AlpineUser,
    pub amount: Vec<cosmwasm_std::Coin>,
    pub message: String,
    pub timestamp: Option<Timestamp>
}