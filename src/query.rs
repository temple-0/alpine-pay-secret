use cosmwasm_std::{entry_point, StdError, CanonicalAddr};
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    Binary, 
    Deps, 
    Env, 
    StdResult, 
    to_binary,
    Addr
};
use secret_toolkit_permit::{Permit, validate};

use crate::msg::{
    QueryMsg, 
    MultiDonationResponse, 
    UsernameAvailableResponse,
    MultiUserResponse,
    AlpineUserResponse, 
    DonationCountResponse, QueryWithPermitMsg
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
        QueryMsg::GetDonationCount {  } => to_binary(&get_donation_count(deps)?),
        QueryMsg::IsUsernameAvailable { username } => to_binary(&is_username_available(deps, username)?),
        QueryMsg::GetAllUsers {  } => to_binary(&get_all_users(deps)?),
        QueryMsg::GetUserByAddr { address } => to_binary(&get_user_by_addr(deps, address)?),
        QueryMsg::GetUserByName { username } => to_binary(&get_user_by_name(deps, username)?),
        QueryMsg::WithPermit { permit, query } => to_binary(&permit_query(deps, permit, query)?)
    }
}

fn permit_query(deps: Deps, permit: Permit, query: QueryWithPermitMsg) -> StdResult<MultiDonationResponse> {
    // Validate permit
    let state = read_state(deps.storage).load()?;
    validate(
        deps,
        "revoked_permits",
        &permit,
        state.contract_address, 
        None
    )?;
    let public_key = permit.signature.pub_key;
    let signer_address = public_key.canonical_address();

    match query {
        QueryWithPermitMsg::GetReceivedDonations { recipient } => get_received_donations(deps, recipient, signer_address),
        QueryWithPermitMsg::GetSentDonations { sender } => get_sent_donations(deps, sender, signer_address)
    }
}

fn get_donation_count(deps: Deps) -> StdResult<DonationCountResponse> {
    let count = donation_count(deps.storage)?;
    Ok(DonationCountResponse { count })
}

fn get_sent_donations(deps: Deps, sender: String, signer_address: CanonicalAddr) -> StdResult<MultiDonationResponse> {
    let state = read_state(deps.storage).load()?;
    let sender_user = find_alpine_username(deps.storage, sender).unwrap();

    // Validate that permit signer is the same as the queried address
    if signer_address != deps.api.addr_canonicalize(sender_user.address.as_str())? {
        return Err(StdError::GenericErr { msg: "Address mismatch".to_string() });
    }

    let mut sent_donations: Vec<DonationInfo> = vec![];

    for donation in state.donations {
        if donation.sender == sender_user {
            sent_donations.append(&mut vec![donation]);
        }
    }

    Ok(MultiDonationResponse { donations: sent_donations })
}

fn get_received_donations(deps: Deps, recipient: String, signer_address: CanonicalAddr) -> StdResult<MultiDonationResponse> {
    let state = read_state(deps.storage).load()?;
    let recipient_user = find_alpine_username(deps.storage, recipient).unwrap();

    // Validate that permit signer is the same as the queried address
    if signer_address != deps.api.addr_canonicalize(recipient_user.address.as_str())? {
        return Err(StdError::GenericErr { msg: "Address mismatch".to_string() });
    }

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
