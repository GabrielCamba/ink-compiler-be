# Ink Compiler Service (Back End)

This project is a Rust-based API for creating and managing smart contracts written in Ink!. It utilizes the Rocket framework and several other Rust crates.

## Setup

1. Clone this repository
2. Install Rust by following the instructions on [rustup.rs](https://rustup.rs/)
3. Run `cargo build --release`
4. Install nightly version `rustup toolchain install nightly`, set it as default `rustup default nightly`, and update with `rustup update`
5. Install cargo contract following the [instructions](https://github.com/paritytech/cargo-contract)
6. Install MongoDB by following the [instructions on mongodb.com](https://www.mongodb.com/docs/manual/installation/)
7. Rename the .env.example file in the root of the project to .env and fill in the following environment variables:
   - `MONGOURI=<The URI of your MongoDB instance>`
8. Run the API executing: `./target/release/compiler-be` created in step 3

## Project Structure

The project consists of four main modules:

- `api`: contains the API endpoints for creating and managing contracts
- `models`: contains the data models used by the API
- `repository`: contains the repository for interacting with the database
- `utils`: contains various utility functions used by the API


## API Reference

#### Send contract to be compiled
Accepts a JSON payload representing a smart contract in plain text, compiles it and returns the compiled contract.

```http
  POST /contract
```

| Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `address` | `string` | **Required**. The wallet address of the request sender |
| `code` | `string` | **Required**. The smart contract code written in Ink! in plain text |
| `features` | `string[]` | **Required**. The smart contract standard and some open brush modifiers that would be needed to be imported by the Cargo.toml file |

Request body example:

```json
{
      "address": "5Dsykc2KUHcziwcTgZkHxyDDTotBJbGNh3BakfZ5PdDGMzfn",
      "code": "#![cfg_attr(not(feature = "std"), no_std)]
               #![feature(min_specialization)]
                     
               #[openbrush::contract]
               pub mod my_psp22 {
                  
                  // imports from openbrush
                  use openbrush::contracts::psp22::*;
                  use openbrush::traits::Storage;

                  #[ink(storage)]
                  #[derive(Default, Storage)]
                  pub struct Contract {
                     #[storage_field]
                     psp22: psp22::Data,
                  }
                  
                  // Section contains default implementation without any modifications
                  impl PSP22 for Contract {}
                  
                  impl Contract {
                     #[ink(constructor)]
                     pub fn new(initial_supply: Balance) -> Self {
                           let mut _instance = Self::default();
                           _instance._mint_to(_instance.env().caller(), initial_supply).expect("Should mint"); 
                        _instance
                     }
                  }
               }",
      "features": ["psp22"]
}
```

Response body example:

```json
{
   "data": {
         "code_id": "5a4ce58af5294a73b22b5c6bf1b1a8886972598925ddee77c3a591ced4bae78b",
         "metadata": "{\n  \"source\": {\n    \"hash\": \"0x481c66073400c0d24a4105fa7a82d47957485235ef10aaf1ef0635bece103e2a\" ...",
         "wasm": [0,97,115,...]
         }
   "error": null
}
```

#### Get contract by code_id
Returns the information of a compiled smart contract given its code_id.

```http
  GET /contract?{code_id}&{wasm}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `code_id`      | `string` | **Required**. The unique identifier of the smart contract. |
| `wasm`      | `bool` | **Optional**. If true, the response will contain the compiled contract in web assembly format. |

Request example:

```http
  GET /contract?code_id=5a4ce58af5294a73b22b5c6bf1b1a8886972598925ddee77c3a591ced4bae78b&wasm=true
```

Response body example:

```json
{
   "data": {
         "code_id": "5a4ce58af5294a73b22b5c6bf1b1a8886972598925ddee77c3a591ced4bae78b",
         "metadata": "{\n  \"source\": {\n    \"hash\": \"0x481c66073400c0d24a4105fa7a82d47957485235ef10aaf1ef0635bece103e2a\" ...",
         "wasm": [0,97,115,...]
         }
   "error": null
}
```

#### Upload contract deployment information
Accepts a JSON payload representing a smart contract deployment and stores it in the database.

```http
  POST /deployments
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `contract_name`      | `string` | **Required**. The name of the smart contract. |
| `contract_address`      | `string` | **Required**. The address of the deployed smart contract. |
| `network`      | `string` | **Required**. The network where the smart contract was deployed. |
| `code_id`      | `string` | **Required**. The unique identifier of the smart contract. |
| `user_address`      | `string` | **Required**. The wallet address of the smart contract deployer. |

Request body example:

```json
{
    "contract_name": "Test Token",
    "contract_address": "5Dsykc2KUHcziwcTgZkHxyDDTotBJbGNh3BakfZ5PdDGMzfn",
    "network": "Rococo",
    "code_id": "5a4ce58af5294a73b22b5c6bf1b1a8886972598925ddee77c3a591ced4bae78b",
    "user_address": "ZA9WeQNb3QKmqvNi1szndDMchQ66npnDFXpjWuKayXQpriW"
}
```

Response body example:

```json
{
    "data": "ok",
    "error": null
}
```

#### Get all contract deployments for a given user
Returns all the smart contract deployments for a given user, optionally filtered by network.

```http
  GET /deployments?{user_address}&{network}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `user_address`      | `string` | **Required**. The wallet address of the deployer. |
| `network`      | `string` | **Optional**. The network where the smart contracts were deployed. |

Request example:

```http
  GET /deployments?user_address=ZA9WeQNb3QKmqvNi1szndDMchQ66npnDFXpjWuKayXQpriW&network=Rococo
```

Response body example:

```json
{
    "data": [
        {
            "contract_name": "Test Token",
            "contract_address": "5Dsykc2KUHcziwcTgZkHxyDDTotBJbGNh3BakfZ5PdDGMzfn",
            "network": "Rococo",
            "code_id": "5a4ce58af5294a73b22b5c6bf1b1a8886972598925ddee77c3a591ced4bae78b",
            "user_address": "ZA9WeQNb3QKmqvNi1szndDMchQ66npnDFXpjWuKayXQpriW"
        }
    ],
    "error": null
}
```