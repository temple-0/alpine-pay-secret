
#[cfg(test)]
mod alpine_user_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Addr, DepsMut };

    use crate::execute::{execute, instantiate};
    use crate::msg::{
        ExecuteMsg,
        QueryMsg,
        MultiUserResponse,
        AlpineUserResponse, InstantiateMsg
    };
    use crate::query::query;
    use crate::state::{read_state, update_state};
    use crate::{
        error::ContractError,
        state::AlpineUser,
        msg::UsernameAvailableResponse
    };

    // A basic utility function to setup the contract so we don't have to do this every time
    fn setup_contract(deps: DepsMut<'_>) {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        instantiate(deps, mock_env(), info, msg).unwrap();
    }

    // Attempt to create a user with an invalid wallet address. Should error out
    #[test]
    fn create_user_invalid_addr() {
        let deps = mock_dependencies();
        let test_address = Addr::unchecked("");
        let create_test_user = AlpineUser::new(
            deps.as_ref(),
            test_address.clone(),
            None
        ).unwrap_err();

        assert_eq!(create_test_user, ContractError::InvalidWalletAddress { address: test_address.to_string() });
    }

    // Attempt to create a new user without registering them. This uses an empty username. Should be successful
    #[test]
    fn create_user_anonymous_success() {
        let deps = mock_dependencies();
        let test_addr = String::from("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh");
        
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(test_addr.clone()),
            None
        ).unwrap();

        assert_eq!(test_user.address, test_addr);
        assert_eq!(test_user.username, "");
    }

    // Attempt to create a user with a (relatively complex) valid username. Should result in success
    #[test]
    fn create_user_with_name_success() {
        let deps = mock_dependencies();
        let test_addr = String::from("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh");
        let test_username = String::from("this-Is-A-Valid_Username1234");
        
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(test_addr.clone()),
            Some(test_username.clone())
        ).unwrap();

        assert_eq!(test_user.address, test_addr);
        assert_eq!(test_user.username, test_username);
    }

    // Register a user with an empty username. Should error out
    #[test]
    fn save_username_empty_username() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: String::from("")
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::EmptyUsername {});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_too_long() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("ThisUsernameIsTooLongAndWillCauseAnError");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("must be shorter than 33 characters")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_backslash() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("\\backslashesareforbidden/");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_spaces() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("spaces are not supported");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_encoded() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("Nice%20Try");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_encoded_greekletter() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("Σ");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Check if an unregistered username is available. Should return true.
    #[test]
    fn is_username_available_true() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();

        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("alpine_user_1") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, true);
    }

    // Check if an registered username is available. Should return false.
    #[test]
    fn is_username_available_false() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("alpine_user_1") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, false);
    }

    // Check if a username is available. Technically the username is unregistered, but the only difference
    // is casing. Should return false.
    #[test]
    fn is_username_available_false_case_insensitive() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("ALPINE_USER_1") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, false);
    }

    // Attempt to register a user with a taken username. Should error out.
    #[test]
    fn save_username_unavailable() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            None
        ).unwrap();

        let username = String::from("alpine_user_1");
        let msg = ExecuteMsg::RegisterUser {
            user: new_user.clone(),
            username: username.clone(),
        };
        let info = mock_info(new_user.address.as_str(), &[]);
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::UsernameNotAvailable { username });
    }

    // Attempt to save a user with an unregistered username. Should be successful
    #[test]
    fn save_username_success() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: String::from("alpine_user_1")
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(_res.attributes[0].value, "alpine_user_1");
    }
    
    // Obtain a list of all saved usernames
    #[test]
    fn get_usernames(){
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        state.users.append(&mut vec![test_user]);
        update_state(&mut deps.storage).save(&state).unwrap();

        // Save User Two
        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            None
        ).unwrap();
        state.users.append(&mut vec![new_user]);
        update_state(&mut deps.storage).save(&state).unwrap();

        // Save User Three
        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            None
        ).unwrap();
        state.users.append(&mut vec![new_user]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = QueryMsg::GetAllUsers { };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let users: MultiUserResponse  = from_binary(&res).unwrap();
        assert_eq!(users.users.len(), 3)
    }

    // Attempt to register a new user whose username prior to this was empty
    #[test]
    fn change_username_from_anonymous(){
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        let mut test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let msg = ExecuteMsg::RegisterUser {
            user: test_user.clone(),
            username: String::from("Anonymous")
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        test_user.username = String::from("Anonymous");
        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: String::from("aline_user_1")
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(res.to_string(), "Address Already Registered")
    }

    // Try to grab a user with a bad address. Technically this works because we're not on-chain.
    #[test]
    fn get_user_by_bad_address(){
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        
        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        // Junk user
        let junk_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jj"),
            Some(String::from(""))
        ).unwrap();

        let msg = QueryMsg::GetUserByAddr{ address: junk_user.address.clone() };
        query(deps.as_ref(), mock_env(), msg).unwrap_err();
    }

    // Try to grab a user with a good address. Results successful
    #[test]
    fn get_user_by_good_address(){
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();
        

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = QueryMsg::GetUserByAddr{ address: Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, test_user);
    }

    // Try to grab a user with a nonexistent username. Results successful, user object is empty
    #[test]
    fn get_user_by_nonexistent_username(){
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // Create an empty user for the assert
        let empty_user = AlpineUser::empty();

        let msg = QueryMsg::GetUserByName{ username: String::from("alpine_user_1") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, empty_user);
    }

    // Try to grab a user with a valid username. Results successful
    #[test]
    fn get_user_by_good_username(){
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        state.users.append(&mut vec![test_user.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = QueryMsg::GetUserByName{ username: String::from("alpine_user_1") };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, test_user);
    }
}

// A set of tests for donations
#[cfg(test)]
mod donation_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, coins, MessageInfo, Addr, DepsMut, Binary, Deps};
    use secret_toolkit_permit::{Permit, PermitParams, TokenPermissions, PermitSignature, PubKey, validate};

    use crate::execute::{execute, instantiate};
    use crate::msg::{
        ExecuteMsg,
        QueryMsg,
        MultiDonationResponse, DonationCountResponse, InstantiateMsg, QueryWithPermitMsg,
    };
    use crate::query::query;
    use crate::state::{read_state, update_state, find_alpine_username, DonationInfo};
    use crate::{
        error::ContractError,
        state::AlpineUser
    };

    const ADDRESS: &str = "secret1h3jx4rjkry20pctnzfj7ek8t4v4zaev2rn0rk2";
    const PUBLIC_KEY: &str = "Ajc/LuFt+czBL/9kgFuvvQsdzUfR1dt2h1rrAt74/tbP";
    const SIGNATURE: &str = "CpBEE52NP09JHAHEqLbWsAmu2GDJrIx6XX4G6uztkaMLiBNL61PyIHk9W3yTfhYWQ0vw+QH4q0HOWfEYWbZSoQ==";
    const CONTRACT_ADDRESS: &str = "cosmos2contract";

    // A basic utility function to setup the contract so we don't have to do this every time
    fn setup_contract(deps: DepsMut<'_>) {
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        instantiate(deps, mock_env(), info, msg).unwrap();
    }

    fn get_sent_donations(deps: Deps, sender: String) -> MultiDonationResponse {
        let state = read_state(deps.storage).load().unwrap();
        let sender_user = find_alpine_username(deps.storage, sender).unwrap();
        let mut sent_donations: Vec<DonationInfo> = vec![];
    
        for donation in state.donations {
            if donation.sender == sender_user {
                sent_donations.append(&mut vec![donation]);
            }
        }
    
        MultiDonationResponse { donations: sent_donations }
    }
    
    fn get_received_donations(deps: Deps, recipient: String) -> MultiDonationResponse {
        let state = read_state(deps.storage).load().unwrap();
        let recipient_user = find_alpine_username(deps.storage, recipient).unwrap();
        let mut received_donations: Vec<DonationInfo> = vec![];
    
        for donation in state.donations {
            if donation.recipient == recipient_user {
                received_donations.append(&mut vec![donation]);
            }
        }
    
        MultiDonationResponse { donations: received_donations }
    }

    fn query_with_permit(deps: Deps, query: QueryWithPermitMsg) -> MultiDonationResponse {
        let permit = Permit {
            params: PermitParams { 
                allowed_tokens: vec![CONTRACT_ADDRESS.to_string()], 
                permit_name: "test".to_owned(), 
                chain_id: "secret-4".to_owned(), 
                permissions: vec![TokenPermissions::Balance]
            },
            signature: PermitSignature { 
                pub_key: PubKey {
                    r#type: "tendermint/PubKeySecp256k1".to_string(),
                    value: Binary::from_base64(PUBLIC_KEY).unwrap(),
                },
                signature: Binary::from_base64(SIGNATURE).unwrap() 
            },
        };

        // Validate permit
        let state = read_state(deps.storage).load().unwrap();
        validate(
            deps,
            "revoked_permits",
            &permit,
            state.contract_address, 
            None
        ).unwrap();
        //Don't grab the sender's address here because it won't canonicalize right in tests
        //Because of that, testing that the user address is validated correctly requires integration tests
        match query {
            QueryWithPermitMsg::GetReceivedDonations { recipient } => get_received_donations(deps, recipient),
            QueryWithPermitMsg::GetSentDonations { sender } => get_sent_donations(deps, sender)
        }
    }

    // Validate that instantiation is succesful
    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());

        // let donation_count = contract.get_donation_count(deps.as_ref()).unwrap();
        let response = query(deps.as_ref(), mock_env(), QueryMsg::GetDonationCount {  }).unwrap();
        let donation_count: DonationCountResponse = from_binary(&response).unwrap();
        assert_eq!(0, donation_count.count);
    }

    // Attempt to send a donation to a user which doesn't exist. Should error out
    #[test]
    fn send_donation_to_nonexistent_recipient() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let invalid_user: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("nonexistent_user"))
        ).unwrap();

        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        state.users.append(&mut vec![alpine_user_a.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: invalid_user.username.clone()
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::UserNotFound { user: invalid_user.username });
    }

    // Attempt to send a donation without any currency attached
    #[test]
    fn send_no_dono() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info =  MessageInfo {
            sender: deps.as_mut().api.addr_validate(alpine_user_a.address.as_str()).unwrap(),
            funds: Vec::new()
        };

        state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: alpine_user_b.username
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::NoDonation{ });
    }

    // Attempt to send a donation which technically has currency attached, but the amount is 0. Should error out
    #[test]
    fn send_no_dono_amount() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(0, "earth"));
        state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: alpine_user_b.username
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::NoDonation{ });
    }

    // Attempt to send a donation with a message that's too long. Should error out
    #[test]
    fn send_too_long_message() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();
        let donation_message: String = String::from("This message is really long. In fact, it's actually too long for you to use it in our app.\
                    We shouldn't allow users to send a message that's this long. There's no reason to send a message that's this long. If I was a \
                    content creator and I was constantly having people send me giant messages like this for like $3, I would not only hate this app, \
                    but I would also begin to dislike my fans.");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));

        state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: alpine_user_b.username
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::DonationMessageTooLong {  });
    }

    // Obtain a list of multiple sent donations and validate length. Should return success.
    #[test]
    fn get_multiple_sent_donations() {
        let mut deps = mock_dependencies();
        setup_contract(deps.as_mut());
        let mut state = read_state(&deps.storage).load().unwrap();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(ADDRESS),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            Some(String::from("USER_D"))
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone(), alpine_user_c.clone(), alpine_user_d.clone()]);
        update_state(&mut deps.storage).save(&state).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_b.username
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_c.username
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_d.username
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();


        
        let sent_donations: MultiDonationResponse = query_with_permit(deps.as_ref(), QueryWithPermitMsg::GetSentDonations { sender: alpine_user_a.username.clone() });
        assert_eq!(3, sent_donations.donations.len());
    }

    // // Obtain a list of multiple sent donations and validate that they're sorted in the correct order. Should return success.
    // #[test]
    // fn get_multiple_sent_donations_sorted(){
    //     let mut deps = mock_dependencies();
    //     setup_contract(deps.as_mut());
    //     let mut state = read_state(&deps.storage).load().unwrap();
    //     let donation_message: String = String::from("henlo :)");
    //     let alpine_user_a: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
    //         Some(String::from("USER_A"))
    //     ).unwrap();
    //     let alpine_user_b: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
    //         Some(String::from("USER_B")) 
    //     ).unwrap();
    //     let alpine_user_c: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
    //         Some(String::from("USER_C"))
    //     ).unwrap();
    //     let alpine_user_d: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
    //         Some(String::from("USER_D"))
    //     ).unwrap();
    //     let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
    //     state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone(), alpine_user_c.clone(), alpine_user_d.clone()]);
    //     update_state(&mut deps.storage).save(&state).unwrap();

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone() + "1", 
    //         sender: alpine_user_a.username.clone(),
    //         recipient: alpine_user_b.username
    //     };
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone() + "2", 
    //         sender: alpine_user_a.username.clone(),
    //         recipient: alpine_user_c.username
    //     };
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone() + "3", 
    //         sender: alpine_user_a.username.clone(),
    //         recipient: alpine_user_d.username
    //     };
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
    //     let msg = QueryMsg::GetSentDonations { sender: alpine_user_a.username.clone() };
    //     let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    //     let sent_donations: MultiDonationResponse = from_binary(&res).unwrap();
    //     assert_eq!(donation_message.clone() + "1", sent_donations.donations[0].message);
    //     assert_eq!(donation_message.clone() + "2", sent_donations.donations[1].message);
    //     assert_eq!(donation_message.clone() + "3", sent_donations.donations[2].message);

    // }

    // // Obtain a list of multiple received donations and validate the length. Should return success.
    // #[test]
    // fn get_multiple_received_donations() {
    //     let mut deps = mock_dependencies();
    //     setup_contract(deps.as_mut());
    //     let mut state = read_state(&deps.storage).load().unwrap();
    //     let donation_message: String = String::from("henlo :)");
    //     let alpine_user_a: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
    //         Some(String::from("USER_A"))
    //     ).unwrap();
    //     let alpine_user_b: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
    //         Some(String::from("USER_B")) 
    //     ).unwrap();
    //     let alpine_user_c: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
    //         Some(String::from("USER_C"))
    //     ).unwrap();
    //     let alpine_user_d: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
    //         Some(String::from("USER_D"))
    //     ).unwrap();
    //     state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone(), alpine_user_c.clone(), alpine_user_d.clone()]);
    //     update_state(&mut deps.storage).save(&state).unwrap();


    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone(), 
    //         sender: alpine_user_b.username.clone(),
    //         recipient: alpine_user_a.username.clone()
    //     };
    //     let info = mock_info(alpine_user_b.address.as_str(), &coins(1000, "earth"));
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone(), 
    //         sender: alpine_user_c.username,
    //         recipient: alpine_user_a.username.clone()
    //     };
    //     let info = mock_info(alpine_user_c.address.as_str(), &coins(1000, "earth"));
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone(), 
    //         sender: alpine_user_a.username.clone(),
    //         recipient: alpine_user_b.username.clone()
    //     };
    //     let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
    //     let msg = QueryMsg::GetReceivedDonations { recipient: alpine_user_a.username.clone() };
    //     let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    //     let received_donations: MultiDonationResponse = from_binary(&res).unwrap();
    //     assert_eq!(2, received_donations.donations.len());
    // }

    // // Obtain a list of multiple received donations and validate that they're sorted in the correct order. Should return success.
    // #[test]
    // fn get_multiple_received_donations_sorted() {
    //     let mut deps = mock_dependencies();
    //     setup_contract(deps.as_mut());
    //     let mut state = read_state(&deps.storage).load().unwrap();
    //     let donation_message: String = String::from("henlo :)");
    //     let alpine_user_a: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
    //         Some(String::from("USER_A"))
    //     ).unwrap();
    //     let alpine_user_b: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
    //         Some(String::from("USER_B")) 
    //     ).unwrap();
    //     let alpine_user_c: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
    //         Some(String::from("USER_C"))
    //     ).unwrap();
    //     let alpine_user_d: AlpineUser = AlpineUser::new(
    //         deps.as_ref(),
    //         Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
    //         Some(String::from("USER_D"))
    //     ).unwrap();

    //     state.users.append(&mut vec![alpine_user_a.clone(), alpine_user_b.clone(), alpine_user_c.clone(), alpine_user_d.clone()]);
    //     update_state(&mut deps.storage).save(&state).unwrap();

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone() + "1", 
    //         sender: alpine_user_b.username.clone(),
    //         recipient: alpine_user_a.username.clone()
    //     };
    //     let info = mock_info(alpine_user_b.address.as_str(), &coins(1000, "earth"));
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone() + "2", 
    //         sender: alpine_user_c.username.clone(),
    //         recipient: alpine_user_a.username.clone()
    //     };
    //     let info = mock_info(alpine_user_c.address.as_str(), &coins(1000, "earth"));
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    //     let msg = ExecuteMsg::SendDonation { 
    //         message: donation_message.clone() + "3", 
    //         sender: alpine_user_c.username.clone(),
    //         recipient: alpine_user_a.username.clone()
    //     };
    //     let info = mock_info(alpine_user_c.address.as_str(), &coins(1000, "earth"));
    //     let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
    //     let msg = QueryMsg::GetReceivedDonations { recipient: alpine_user_a.username.clone() };
    //     let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    //     let received_donations: MultiDonationResponse = from_binary(&res).unwrap();
    //     assert_eq!(donation_message.clone() + "1", received_donations.donations[0].message);
    //     assert_eq!(donation_message.clone() + "2", received_donations.donations[1].message);
    //     assert_eq!(donation_message.clone() + "3", received_donations.donations[2].message);
    // }
}

// Define a set of integration tests that use our entry points instead of internal calls
#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_info, mock_env}, 
        Addr, 
        from_binary
    };

    use crate::{
        msg::{InstantiateMsg, MigrateMsg, ExecuteMsg, QueryMsg, MultiUserResponse},
        state::AlpineUser, execute::{instantiate, migrate, execute}, query::query
    };
    // use entry::{ instantiate, migrate, query, execute };

    // Validate that instantiation works from the client's perspective
    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    // Validate that migration works from the client's perspective
    #[test]
    fn proper_migration() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = MigrateMsg {};
        let res = migrate(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len())
    }

    // Validate that execution works from the client's perspective
    #[test]
    fn successful_execute() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1tpsscvhaddf36gjvnyjhtwsyempptupypngxzs"),
            Some(String::from(""))
        ).unwrap();
        let msg = ExecuteMsg::RegisterUser {
            user: alpine_user_a.clone(),
            username: String::from("a_tester")
        };
        let info = mock_info(alpine_user_a.address.as_str(), &[]);
        
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    }

    // Validate that queries work from the client's perspective
    #[test]
    fn successful_query() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1tpsscvhaddf36gjvnyjhtwsyempptupypngxzs"),
            Some(String::from(""))
        ).unwrap();
        let msg = ExecuteMsg::RegisterUser {
            user: alpine_user_a.clone(),
            username: String::from("a_tester")
        };
        let info = mock_info(alpine_user_a.address.as_str(), &[]);
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::GetAllUsers { };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let users: MultiUserResponse  = from_binary(&res).unwrap();
        assert_eq!(users.users.len(), 1)
    }
}