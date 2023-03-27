# Ink Compiler Service (Back End)

This project is a Rust-based API for creating and managing smart contracts written in Ink!. It utilizes the Rocket framework and several other Rust crates.


## Setup

1. Clone this repository
2. Install Rust by following the instructions on [rustup.rs](https://rustup.rs/)
3. Run `cargo build --release`
4. Install nightly version `rustup toolchain install nightly`, set it as default `rustup default nightly`, and update with `rustup update`
5. Install cargo contract following the [instructions](https://github.com/paritytech/cargo-contract)
6. Install MongoDB by following the [instructions on mongodb.com](https://www.mongodb.com/docs/manual/installation/)
7. Create a .env file in the root of the project and add the following environment variables:
    - `MONGOURI=<The URI of your MongoDB instance>`
    - `CARGO=<The cargo location>` (You can get it by running `whereis cargo` command)
8. Run the API executing: `./target/release/compiler-be` created in step 3


## Project Structure

The project consists of four main modules:
- `api`: contains the API endpoints for creating and managing contracts
- `models`: contains the data models used by the API
- `repository`: contains the repository for interacting with the database
- `utils`: contains various utility functions used by the API

## API Endpoints

The API has a single endpoint:
- `POST /contract`: accepts a JSON payload representing a contract and returns the compiled contract if already exists or it compiles it, stores in the database and returns it.

## Usage

`POST /contract`: The request body must be a JSON object with the following fields:
- `address`: The wallet address of the request sender
- `code`: The smart contract code written in Ink! in plain text
- `features`: The smart contract standard and some open brush modifiers that would be needed to be imported by the Cargo.toml file


The response will be a JSON object with the following fields:

- `code_id`: The unique identifier of the smart contract
- `metadata`: The smart contract generated metadata
- `wasm`: The web assembly code obtained from compiling the smart contract
