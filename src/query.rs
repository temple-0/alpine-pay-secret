use cosmwasm_std::{entry_point, StdError};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Binary, 
    Deps, 
    Env, 
    StdResult, 
    to_binary,
    Addr
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
    contains_username, get_user_by_address, read_state
};

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

fn get_donation_count(deps: Deps) -> StdResult<DonationCountResponse> {
    let count = donation_count(deps.storage)?;
    Ok(DonationCountResponse { count })
}

fn get_sent_donations(deps: Deps, sender: String) -> StdResult<MultiDonationResponse> {
    let state = read_state(deps.storage).load()?;
    let sender_user = find_alpine_username(deps.storage, sender).unwrap();
    let mut sent_donations: Vec<DonationInfo> = vec![];

    for donation in state.donations {
        if donation.sender == sender_user {
            sent_donations.append(&mut vec![donation]);
        }
    }

    Ok(MultiDonationResponse { donations: sent_donations })
}

fn get_received_donations(deps: Deps, recipient: String) -> StdResult<MultiDonationResponse> {
    let state = read_state(deps.storage).load()?;
    let recipient_user = find_alpine_username(deps.storage, recipient).unwrap();
    let mut received_donations: Vec<DonationInfo> = vec![];

    for donation in state.donations {
        if donation.recipient == recipient_user {
            received_donations.append(&mut vec![donation]);
        }
    }

    Ok(MultiDonationResponse { donations: received_donations })
}

fn is_username_available(deps: Deps, username: String) -> StdResult<UsernameAvailableResponse> {
    let is_available = !contains_username(deps.storage, username).unwrap();
    Ok(UsernameAvailableResponse { is_available })
}

fn get_all_users(deps: Deps) -> StdResult<MultiUserResponse> {
    let state = read_state(deps.storage).load()?;
    Ok(MultiUserResponse { users: state.users })
}

fn get_user_by_addr(deps: Deps, address: Addr) -> StdResult<AlpineUserResponse>{
    match get_user_by_address(deps.storage, address) {
        Ok(user) => Ok(AlpineUserResponse { user }),
        Err(e) => Err(StdError::GenericErr { msg: e.to_string() })
    }
}

fn get_user_by_name(deps: Deps, username: String) -> StdResult<AlpineUserResponse> {
    let user = match find_alpine_username(deps.storage, username.clone()) {
        Ok(user) => { user },
        Err(_) => { AlpineUser::empty() }
    };

    Ok(AlpineUserResponse { user })
}
