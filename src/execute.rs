use cosmwasm_std::{
    Addr,
    coins,
    Decimal,
    DepsMut,
    MessageInfo,
    Env,
    StdResult,
    Response,
    entry_point,
    BankMsg
};
use cw2::{set_contract_version, get_contract_version};

use crate::{
    msg::{
        InstantiateMsg,
        MigrateMsg,
        ExecuteMsg
    }, 
    error::ContractError,
    state::{
        AlpineUser,
        DonationInfo,
        increment_donations,
        find_alpine_username,
        update_donations,
        get_user_by_address,
        read_state,
        update_state,
        USERNAMES,
        ADDRESSES
    }
};

#[cfg(not(feature = "library"))]
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:alpine-pay";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg
) -> StdResult<Response> {
    // set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION);
    Ok(Response::default())
}

#[entry_point]
pub fn migrate(
    deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg
) -> Result<Response, ContractError> {
    // let ver = get_contract_version(deps.storage);
    // ensure_eq!(ver.contract, CONTRACT_NAME, ContractError::IncorrectContractName { contract_name: String::from(CONTRACT_NAME) });
    // set_contract_version(deps.storage, ver.contract, ver.version.clone())?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SendDonation { sender, recipient, message } => send_donation(deps, _env, info, sender, recipient, message),
        // With register we can authenticate the user here, whereas with SendDonation it's a bit more complex and done later
        ExecuteMsg::RegisterUser { user, username } => {
            if info.sender != user.address {
                return Err(ContractError::InvalidWalletAddress { address: user.address.to_string() })
            }
            register_user(deps, _env, user, username)
        }
    }
}

// Send a donation to the designated user
fn send_donation(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo,
    sender: String,
    recipient: String, 
    message: String
) -> Result<Response, ContractError> {
    // Verify that there's a recipient
    if recipient.is_empty() {
        return Err(ContractError::EmptyUsername {})
    }

    // Verify that funds are attached
    if info.funds.is_empty() || info.funds[0].amount.to_string() == String::from("0") {
        return Err(ContractError::NoDonation{})
    }

    // Get an Alpine user for the sender. This technically allows a user to send if they're unregistered
    let sender_user = match sender.is_empty() {
        true => AlpineUser::new(deps.as_ref(), info.sender.clone(), None)?,
        false => find_alpine_username(deps.storage, sender)?
    };

    // Authenticate the sender
    if info.sender != sender_user.address {
        return Err(ContractError::InvalidWalletAddress { address: sender_user.address.to_string() })
    }

    // Validate that the donation message isn't too long
    if message.len() > 250 {
        return Err(ContractError::DonationMessageTooLong {  })
    }

    // Find the recipient user by their username
    let recipient_user = find_alpine_username(deps.storage, recipient)?;

    // Build out the donation message
    let donation = DonationInfo {
        sender: sender_user,
        recipient: recipient_user,
        amount: info.funds,
        message: message,
        timestamp: Some(env.block.time)
    };

    // Update the donations and set the new donation's ID
    let id = increment_donations(deps.storage)?;

    update_donations(deps.storage, donation.clone(), id)?;

    let total_donation_amount = donation.amount.clone()[0].amount;
    let donation_fee = Decimal::percent(3) * donation.amount.clone()[0].amount;
    let recipient_donation = &coins((total_donation_amount - donation_fee).u128(), donation.amount.clone()[0].denom.clone());
    let commission = &coins(donation_fee.u128(), donation.amount.clone()[0].denom.clone());

    // Forward the funds to the relevant wallet address
    let recipient_bank_msg = BankMsg::Send {
        to_address: donation.recipient.address.to_string(),
        amount: recipient_donation.clone()
    };

    // Take 3% donation fee to Alpine admin address
    let fee_bank_msg = BankMsg::Send { 
        to_address: Addr::unchecked("osmo1zw5337y7a7ajj2jz4t0teyzcy5dup5k8wjz88a").into_string(), 
        amount: commission.clone()
    };

    let attributes = vec![("sender_address", donation.sender.address.to_string()), ("sender_username", donation.sender.username.to_string()), 
                    ("recipient_address", donation.recipient.address.to_string()), ("recipient_username", donation.recipient.username.to_string()),
                    ("amount", donation.amount[0].amount.to_string()), ("message", donation.message), ("timestamp", env.block.time.to_string()),
                    ("id", id.to_string()) ].into_iter();
    let tx_messages = vec![recipient_bank_msg, fee_bank_msg].into_iter();

    Ok(Response::new().add_messages(tx_messages).add_attributes(attributes))
}

// Register a new Alpine user
fn register_user(
    deps: DepsMut,
    _env: Env,
    mut user: AlpineUser,
    username: String
) -> Result<Response, ContractError> {
    // Validate the username
    let valid_username = match validate_username(username.clone()) {
        Ok(u) => u,
        Err(e) => return Err(e)
    };

    // Verify that the user isn't already registered 
    user = match user.username.is_empty() {
        true => {
            match get_user_by_address(deps.storage, user.address.clone()) {
                Ok(_) => {
                    return Err(ContractError::UserNotFound { user: user.address.clone().to_string() })
                },
                Err(_) => AlpineUser::new(deps.as_ref(), user.address.clone(), None)?
            }
        },
        false => return Err(ContractError::UserAlreadyExists {  } )
    };

    let state = read_state(deps.storage).load()?;

    // Verify that the desired username isn't already taken
    // let searched_username = match USERNAMES.get(deps.storage, &valid_username.clone()) {
    //     Ok(result) => match result {
    //         Some(_) => Err(ContractError::UsernameNotAvailable { username: valid_username.clone() }),
    //         None => Ok(valid_username.clone())
    //     },
    //     Err(e) => Err(ContractError::Std(e))
    // }?;
    let searched_username = match find_alpine_username(deps.storage, username.clone()) {
        Ok(alpine_user) => Err(ContractError::UsernameNotAvailable { username: username.clone() }),
        Err(e) => Ok(username)
    }?;

    // Set the user's username, then save them to the contract
    user.username = searched_username.clone();

    USERNAMES.insert(deps.storage, &searched_username, &user)?;
    ADDRESSES.insert(deps.storage, &user.address.clone(), &user)?;
    update_state(deps.storage);
    
    Ok(Response::new().add_attribute("username", user.username))
}

// Validate that the user's username is accepted
fn validate_username(username: String) -> Result<String, ContractError> {
    // Users can't register with an empty username.
    if username.is_empty() {
        return Err(ContractError::EmptyUsername {})
    }

    // Users can't create a name with more than 32 characters
    if username.len() > 32 {
        return Err(ContractError::InvalidUsername { 
            username,
            reason: String::from("must be shorter than 33 characters")
        })
    }

    // Verify that only alphanumeric characters, dashes, and underscores are used to mitigate the risk of injection attacks
    for c in username.chars() {
        if !(c.is_ascii_alphabetic() || c.is_numeric() || c == '-' || c == '_') {
            return Err(ContractError::InvalidUsername { 
                username,
                reason: String::from("only alphanumeric, underscores, and dashes are allowed")
            })
        }
    }

    Ok(username)
}
