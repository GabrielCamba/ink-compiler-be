# Ink Compiler Service (Back End)

## Overview
This project is an API designed to manage ink! smart contract compilations and deployments. The API is built using the Rust programming language and utilizes the Rocket web framework. The API is designed to interact with a MongoDB database, which is used to store smart contract code, as well as information related to deployments.

### 🚀 Run app

#### A. With Docker

- ⚠️ Requirements:
  - docker >= 20
  - docker-compose >= 2.15

1. Clone this repository and enter the project folder
```bash
    git clone https://github.com/GabrielCamba/ink-compiler-be.git && cd ink-compiler-be
```
2. Make sure your daemon `docker` is running in your system
3. Run the command `docker-compose up compiler-be`
4. Your application should now be running on the port 8000. You can call this API using the following base URL: `http://localhost:8000`

> ✋ To stop the application, run the command `docker-compose down` or press `Ctrl + C` in the terminal where the application is running.

#### B. Local Stack

- ⚠️ Requirements:
  - mongodb database

  A. Install MongoDB locally by following the [instructions on mongodb.com](https://www.mongodb.com/docs/manual/installation/)

  B. Use a MongoDB instance in the cloud, such as [MongoDB Atlas](https://www.mongodb.com/cloud/atlas)

Either way, you will need to specify the URI of your MongoDB instance in the `.env` file. The URI should be in the following [format](https://www.prisma.io/dataguide/mongodb/connection-uris#a-quick-overview)

> 🚨🚨🚨 It is necessary to specify the variables int the `.env` to run the application.
>   - `MONGOURI=<The URI of your MongoDB instance>`

1. Clone this repository and enter the project folder
```bash
    git clone https://github.com/GabrielCamba/ink-compiler-be.git && cd ink-compiler-be
```
2. Install Rust by following the instructions on [rustup.rs](https://rustup.rs/)
3. Run `cargo build --release`
4. Install nightly version `rustup toolchain install nightly`, set it as default `rustup default nightly`, and update with `rustup update`
5. Install cargo contract following the [instructions](https://github.com/paritytech/cargo-contract)
6. Run the API executing: `./target/release/compiler-be`

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
| `tx_hash`      | `string` | **Optional**. The transaction hash of the deployment transaction. |
| `date`      | `string` | **Required**. The date of the deployment or upload. |
| `contract_type`      | `string` | **Required**. The type of smart contract. |
| `external_abi`      | `string` | **Optional**. The external ABI of the smart contract. |

Request body example:

```json
{
    "contract_name": "Test Token",
    "contract_address": "5Dsykc2KUHcziwcTgZkHxyDDTotBJbGNh3BakfZ5PdDGMzfn",
    "network": "Rococo",
    "code_id": "5a4ce58af5294a73b22b5c6bf1b1a8886972598925ddee77c3a591ced4bae78b",
    "user_address": "ZA9WeQNb3QKmqvNi1szndDMchQ66npnDFXpjWuKayXQpriW",
    "tx_hash": "0x481c66073400c0d24a4105fa7a82d47957485235ef10aaf1ef0635bece103e2a",
    "date": "2021-09-30T15:00:00Z",
    "contract_type": "psp22"
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
Returns all the smart contract deployments for a given user, optionally filtered by network and contract address.

```http
  GET /deployments?{user_address}&{network}&{contract_address}
```

| Parameter | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `user_address`      | `string` | **Required**. The wallet address of the deployer. |
| `network`      | `string` | **Optional**. The network where the smart contracts were deployed. |
| `contract_address`      | `string` | **Optional**. The address of the deployed smart contract. |


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

## Testing

To run the tests, run the following command

```bash
  docker-compose run compiler-be-test 
```

## Contributing
If you would like to contribute to this project, please feel free to submit a pull request.

## License

Copyright 2023 Protofire

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

<http://www.apache.org/licenses/LICENSE-2.0>

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.