use cosmwasm_std::entry_point;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Binary, 
    Deps, 
    Env, 
    StdResult, 
    to_binary,
    Addr,
    Timestamp
};

use crate::msg::{
    QueryMsg, 
    MultiDonationResponse, 
    UsernameAvailableResponse,
    MultiUserResponse,
    AlpineUserResponse, 
    DonationCountResponse
};
use crate::state::{ 
    AlpineUser, 
    DonationInfo, 
    donation_count,
    find_alpine_username, 
    read_state,
    contains_username, self, ADDRESSES, get_user_by_address, USERNAMES, DONATIONS
};
// use crate::traits::DonationQuery;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSentDonations { sender } => to_binary(&get_sent_donations(deps, sender)?),
        QueryMsg::GetReceivedDonations { recipient } => to_binary(&get_received_donations(deps, recipient)?),
        QueryMsg::GetDonationCount {  } => to_binary(&get_donation_count(deps)?),
        QueryMsg::IsUsernameAvailable { username } => to_binary(&is_username_available(deps, username)?),
        QueryMsg::GetAllUsers {  } => to_binary(&get_all_users(deps)?),
        QueryMsg::GetUserByAddr { address } => to_binary(&get_user_by_addr(deps, address)?),
        QueryMsg::GetUserByName { username } => to_binary(&get_user_by_name(deps, username)?)
    }
}

// Get a count of all the donations
fn get_donation_count(deps: Deps) -> StdResult<DonationCountResponse> {
    let count = donation_count(deps.storage)?;
    Ok(DonationCountResponse { count })
}

// Get all of the donations sent by a user
fn get_sent_donations(deps: Deps, sender: String) -> StdResult<MultiDonationResponse> {
    // let state = read_state(deps.storage).load()?;
    // let sender_user = find_alpine_username(deps.storage, sender).unwrap();

    // // Generate a vector of tuples containing the donation and a byte array identifier.
    // let donations: StdResult<Vec<(Vec<_>, _)>> = state
    //     .donations
    //     .idx
    //     .sender
    //     .prefix(sender_user)
    //     .range(deps.storage, None, None, Order::Ascending)
    //     .collect();
    // let donations = sort_donations_by_date(donations?.clone());

    // Ok(MultiDonationResponse{ donations })
    let sender_user = find_alpine_username(deps.storage, sender).unwrap();
    let received_donations: Vec<Result<(u64, DonationInfo), cosmwasm_std::StdError>> = DONATIONS
        .iter(deps.storage)?
        .filter(|d| d.as_ref().unwrap().1.sender == sender_user)
        .collect();
    
    let mut donations: Vec<DonationInfo> = vec![];
    for donation in received_donations {
        let donation = donation?;
        donations.push(donation.1);
    }

    Ok(MultiDonationResponse { donations: vec![] })
}

// Get all of the donations received by a user
fn get_received_donations(deps: Deps, recipient: String) -> StdResult<MultiDonationResponse> {
    // let state = read_state(deps.storage).load()?;
    // let recipient_user = find_alpine_username(deps.storage, recipient).unwrap();

    // // Generate a vector of tuples containing the donation and a byte array identifier
    // let donations: StdResult<Vec<(Vec<_>, _)>> = state
    //     .donations
    //     .idx
    //     .recipient
    //     .prefix(recipient_user)
    //     .range(deps.storage, None, None, Order::Ascending)
    //     .collect();
    // let donations = sort_donations_by_date(donations?.clone());

    // Ok(MultiDonationResponse{ donations })
    let recipient_user = find_alpine_username(deps.storage, recipient).unwrap();
    let received_donations: Vec<Result<(u64, DonationInfo), cosmwasm_std::StdError>> = DONATIONS
        .iter(deps.storage)?
        .filter(|d| d.as_ref().unwrap().1.recipient == recipient_user)
        .collect();
    
    let mut donations: Vec<DonationInfo> = vec![];
    for donation in received_donations {
        let donation = donation?;
        donations.push(donation.1);
    }

    Ok(MultiDonationResponse { donations: vec![] })
}

// Check if a username has already been registered
fn is_username_available(deps: Deps, username: String) -> StdResult<UsernameAvailableResponse> {
    let is_available = !contains_username(deps.storage, username).unwrap();
    Ok(UsernameAvailableResponse { is_available })
}

// Get a list of all registered users
fn get_all_users(deps: Deps) -> StdResult<MultiUserResponse> {
    // let state = read_state(deps.storage).load()?;
    // // Get a list of usernames mapped to their corresponding user
    // let usernames: StdResult<Vec<(String, _)>> = state
    //     .usernames
    //     .prefix_range(deps.storage, None, None, Order::Ascending)
    //     .collect();
    // let usernames = usernames?;

    // // Remove the Alpine user from the vector above, returning just a list of usernames
    // let mut users: Vec<AlpineUser> = Vec::new();
    // for username in usernames{
    //     users.push(username.1);
    // }

    // Ok(MultiUserResponse{ users })
    let user_iter = USERNAMES.iter(deps.storage)?;
    let mut users:  Vec<AlpineUser> = vec![];
    for user in user_iter {
        let user = user?;
        users.push(user.1);
    }

    Ok(MultiUserResponse { users })
}

// Find the corresponding Alpine user for a given wallet address
fn get_user_by_addr(deps: Deps, address: Addr) -> StdResult<AlpineUserResponse>{
    let user = get_user_by_address(deps.storage, address).unwrap();
    Ok(AlpineUserResponse{ user })
}

// Find the corresponding Alpine user for a given username
fn get_user_by_name(deps: Deps, username: String) -> StdResult<AlpineUserResponse> {
    let user = match find_alpine_username(deps.storage, username.clone()) {
        Ok(user) => { user },
        Err(_) => { AlpineUser::empty() }
    };

    Ok(AlpineUserResponse { user })
}

// Route queries to the smart contract
// impl<'a> AlpineContract<'a> {
//     pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
//         match msg {
//             QueryMsg::GetSentDonations{ sender } => to_binary(&self.get_sent_donations(deps, sender)?),
//             QueryMsg::GetReceivedDonations { recipient } => to_binary(&self.get_received_donations(deps, recipient)?),
//             QueryMsg::GetDonationCount {  } => to_binary(&self.get_donation_count(deps)?),
//             QueryMsg::IsUsernameAvailable { username } => to_binary(&self.is_username_available(deps, username)?),
//             QueryMsg::GetAllUsers { } => to_binary(&self.get_all_users(deps)?),
//             QueryMsg::GetUserByAddr { address } => to_binary(&self.get_user_by_addr(deps, address)?),
//             QueryMsg::GetUserByName { username } => to_binary(&self.get_user_by_name(deps, username)?)
//         }
//     }
// }

// Sort donations providing the most recent donation first. This is only used on the backend - not directly reachable
fn sort_donations_by_date(mut donations: Vec<(Vec<u8>, DonationInfo)>) -> Vec<(Vec<u8>, DonationInfo)>{
    donations.sort_by(|a, b| {
        let a_timestamp = match a.1.timestamp{
            Some(time) => time,
            None => Timestamp::from_seconds(0)
        };
        let b_timestamp = match b.1.timestamp{
            Some(time) => time,
            None => Timestamp::from_seconds(0)
        };

        a_timestamp.cmp(&b_timestamp)
    });
    return donations;
}