# Alpine Pay
## Introduction
Alpine is a web application payment platform built on the Secret blockchain; designed to facilitate tipping of content creators using Secret tokens, users can now show their appreciation by making secure donations while including personalized messages to any social media influencer or content creator they wish to support.

Alpine makes tipping content creators fun, simple, visually pleasing, and censorship-resistant. Alpine provides a seamless user experience by providing a Kado widget, so that users who are unfamiliar with the Cscrts ecosystem can easily purchase crypto with fiat currency and use Alpine to send it to their favorite influencer without the hassle of relying on centralized exchanges. Alpine also provides integrations for popular social media platforms such as Twitter, Instagram, TikTok, and others so that users can easily identify their favorite influencer in the app.

# Alpine Pay Core Contract
## Introduction
The Alpine Pay Core Contract provides all of the core functionality for Alpine. It facilitates the creation and processing of tips, ensuring the secure transfer of Secret from the sender to the content creator's wallet address. Additionally, it enables the inclusion of personalized messages, allowing users to express their sentiments to the content creators.

The Alpine Pay Core Contract does not facilitate the storage of social media data due to the lack of confidentiality of data stored on the blockchain. It also does not facilitate the purchase of crypto with fiat currency. These features are provided by the Alpine Pay Frontend.

## Donation Fee Acknowledgement
By using the Alpine Pay Core Contract, you acknowledge and agree that Alpine will charge a fee of 3% on any donations received by a user ("Donation Fee"). The Donation Fee is deducted from the total amount of each donation before it is credited to the user's account. Please note that the Donation Fee is subject to change, and Alpine reserves the right to modify the fee structure with prior notice. Any changes to the Donation Fee will be communicated through an update to the README.

By continuing to use the platform or service, you indicate your acceptance of the Donation Fee and any updates or modifications to the fee structure.

## Usage
All usage of the Alpine Pay Core Contract assumes that you have a proper development environment set up for a Cscrts chain. **The following documentation will assume that you are using Secret in the Mainnet environment**, but technically you can use other Cscrts chains which are compatible with the CosmWasm code used in the Alpine Pay Core Contract, such as Juno. For more information on setting up your development environment, see the documentation for your chosen chain.

### Instantiation
The first step of using the Alpine Pay Core Contract is to deploy it and instantiate it. The instantiation message for this contract takes no arguments.
1. Set the client configuration for Secret by running:
```
# secretcli config node https://rpc.pulsar.scrttestnet.com
# secretcli config chain-id pulsar-3
secretcli config node https://rpc.secret.express
secretcli config chain-id secret-4  
```

2. Navigate to the project root directory and build/optimize your code using
```
docker run --rm -v "$(pwd)":/contract \
     --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
     --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   enigmampc/secret-contract-optimizer
```
3. Next, store your compiled code on the testnet blockchain and save the id of your code in an environment variable to use later.
```
secretcli tx compute store contract.wasm.gz -y --gas auto --gas-adjustment 1.3 --from <your-wallet-name> 
```
4. Verify success by listing the code and verifying the last code ID was created by your wallet address
```
secretcli query compute query list-code
```
4. Instantiate the contract so that it can actually be used.
```
secretcli tx compute instantiate $id '{}' --from <your-secret-wallet-name> --label "migrate to scrt" -y -b block 
```
5. Grab the address of the contract.
```
address=$(secretcli query compute query list-contract-by-code $id)
```
### Migration
If you want to update the code of an Alpine Core Contract deployment, then you'll need to migrate it. Migration can only be done from the `admin` address defined in the instantiation section. Additionally, this section assumes that you still have the address of the contract saved in the `$address` environment variable on your terminal.
1. Navigate to the `contracts/alpine-pay` directory and build/optimize your code using
```
docker run --rm -v "$(pwd)":/contract \                       
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  enigmampc/secret-contract-optimizer
```
2. Next, store your compiled code on the testnet blockchain and save the id of your code in an environment variable to use later.
```
id=$(secretcli tx compute store artifacts/alpine_pay.wasm  --from <your-secret-wallet-name> --gas-prices 0.1uscrt --gas auto --gas-adjustment 1.3 -y --output json -b block | jq -r '.logs[0].events[-1].attributes[1].value')
```
3. Migrate the contract address to the new code ID.
```
secretcli tx compute migrate $address $id '{ }' --from <your-secret-wallet-name> --gas-prices 0.1uscrt --gas-adjustment 1.3 --gas auto -b block -y
```
4. To verify that the transaction was successful, you can run the following command.
```
secretcli query compute query contract-history $address
```
### Register a User
Alpine allows users to register a username associated with their wallet address. This feature makes it easy to communicate with other users, because there's no need to memorize or copy a complicated wallet address. 
1. Verify that your desired username is available.
```
secretcli q compute query <contract-address> '{"is_username_available":{"username":"<your-desired-username>"}}'
```
A username which is already taken should return `is_available: false`

2. Register your user.
```
secretcli tx compute execute $address '{"register_user":{"user":{"address":"<your-secret-wallet-address>", "username":""}, "username":"<your-desired-username>"}}' \
    --from <your-secret-wallet-name> -b block
```
3. Verify that registration was successful.
```
secretcli q compute query $address '{"get_user_by_name": {"username":"<your-chosen-username>"}}'
```
The output of this should return your address and chosen username.
### Send a Donation
The primary functionality of the Core Contract from the perspective of most users is sending donations. This functionality assumes that there are at least two users registered, as you can't send a donation to yourself.
1. Get a list of all users so that you can find who you want to send a donation to.
```
secretcli q compute query $address '{"get_all_users": { }}'
```
2. Find the username of the user that you want to send the user to. Then send them a donation.
```
secretcli tx compute execute $address '{"send_donation":{"sender":"<your-username>", "recipient":"<recipient-username>", "message":"<your-message-text>"}}' --from <your-secret-wallet-name> --amount <your-desired-donation-amount> -b block
```
### Verify Send Success
1. First, generate a document to sign which conforms to SNIP-24 standards
```
 echo '{
    "chain_id": "<your-chain-id>",
    "account_number": "0",
    "sequence": "0",
    "msgs": [
        {
            "type": "query_permit",
            "value": {
                "permit_name": "test",
                "allowed_tokens": [
                    "<contract-address>"
                ],
                "permissions": ["balance"]
            }
        }
    ],
    "fee": {
        "amount": [
            {
                "denom": "uscrt",
                "amount": "0"
            }
        ],
        "gas": "1"
    },
    "memo": ""
}' > ./permit.json
```
2. Generate a signed document 
```
secretd tx sign-doc ./permit.json --from <your-wallet-name> > ./sig.json
```
3. Verify that your donation was sent successfully, wrapping the actual donation query in a permit query
```
secretcli q compute query $address '{"with_permit":{"query":{"get_sent_donations":{"sender":"<your-username>"}},"permit":{"params":{"permit_name":"test","allowed_tokens":[<your-contract-address>],"chain_id":"<your-chain-id>","permissions":["balance"]},"signature":<entirety-of-sig.json-file>}}}'
```
### Get a List of Donations Sent to You
From the perspective of a content creator, the biggest function in the Core Contract is viewing the donations that they've received. This assumes that you're already registered.
1. First, generate a document to sign which conforms to SNIP-24 standards
```
 echo '{
    "chain_id": "<your-chain-id>",
    "account_number": "0",
    "sequence": "0",
    "msgs": [
        {
            "type": "query_permit",
            "value": {
                "permit_name": "test",
                "allowed_tokens": [
                    "<contract-address>"
                ],
                "permissions": ["balance"]
            }
        }
    ],
    "fee": {
        "amount": [
            {
                "denom": "uscrt",
                "amount": "0"
            }
        ],
        "gas": "1"
    },
    "memo": ""
}' > ./permit.json
```
2. Generate a signed document 
```
secretd tx sign-doc ./permit.json --from <your-wallet-name> > ./sig.json
```
3. Query all of the donations sent to you
```
secretcli q compute query $address '{"with_permit":{"query":{"get_received_donations":{"recipient":"<your-username>"}},"permit":{"params":{"permit_name":"test","allowed_tokens":[<your-contract-address>],"chain_id":"<your-chain-id>","permissions":["balance"]},"signature":<entirety-of-sig.json-file>}}}'
```
### Supporting Functionality
In addition to the main functions of the contract, there are a few other functions which support our web application. These typically wouldn't be used if you're using the CLI, but they could be interesting regardless.
- Obtain a count of all donations sent through the Core Contract.
```
secretcli q compute query $address '{"get_num_donations":{ }}'
```
- Get a user by their wallet address
```
secretcli q compute query $address '{"get_user_by_address": {"username":"<user-secret-wallet-address>"}}'
```
