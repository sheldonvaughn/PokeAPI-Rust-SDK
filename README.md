![ferris_pokemon](https://github.com/user-attachments/assets/9b99964a-2a99-4805-96df-c0b29c83f7df)
# PokeAPI Rust SDK

A Rust SDK for the [PokeAPI](https://pokeapi.co/), providing easy access to Pokémon data. For when you gotta catch 'em all. With type safety.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Testing](#testing)
- [Sandbox](#sandbox)
- [Tools and Libraries](#tools-and-libraries)

## Features

- Retrieve detailed information about Pokémon by ID or name.
- Fetch data about Pokémon Generations by ID or name.
- Asynchronous API calls using `async`/`await`.
- Comprehensive error handling with custom error types.
- Autopagination
- Built in examples for a true sandbox experience. Run the client and test the functions.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
pokeapi_sdk = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Usage

Retrieving a Pokémon by name

```rust
use pokeapi_sdk::PokeApiClient;
use tokio;

#[tokio::main]
async fn main() {
    let client = PokeApiClient::default();

    // Retrieve a Pokémon by name
    match client.get_pokemon_by_name("pikachu").await {
        Ok(pokemon) => {
            println!("Name: {}", pokemon.name);
            println!("ID: {}", pokemon.id);
            println!("Height: {}", pokemon.height);
            println!("Weight: {}", pokemon.weight);
        }
        Err(e) => eprintln!("Error retrieving Pokémon: {}", e),
    }
}
```

Listing Pokèmon with Pagination and Autopagination

```rust
use pokeapi_sdk::PokeApiClient;
use tokio;

#[tokio::main]
async fn main() {
    let client = PokeApiClient::default();

    // List first 10 Pokémon without autopagination
    let pokemon_list = client.list_pokemon(Some(10), None, false).await.unwrap();
    println!("First 10 Pokémon:");
    for pokemon in &pokemon_list {
        println!("- {}", pokemon.name);
    }

    // Autopaginate to get all Pokémon (BECAUSE SOMETIMES YOU JUST GOTTA CATCH 'EM ALL)
    // Use responsibly
    let all_pokemon = client.list_pokemon(None, None, true).await.unwrap();
    println!("Total Pokémon fetched: {}", all_pokemon.len());
}
```
## Testing

This library contains a suite of unit tests to ensure each function both works and fails as expected. 

Run the unit tests with:

```bash
cargo run --example main
```

## Sandbox

Try it yourself! I've included code snippets you can explore, tinker with, or delete entirely in https://github.com/sheldonvaughn/rust_pokeapi/blob/main/examples/main.rs

Run the code with:

```bash
cargo run --example main
```

## Tools and Libraries

- **Rust**
  
  The programming language used to develop this SDK, offering performance and safety.

- **Reqwest**
  
  A powerful, ergonomic HTTP client for making API requests.

- **Tokio**
  
  An asynchronous runtime for Rust, enabling high-performance networking.

- **Serde**
  
  A framework for serializing and deserializing Rust data structures efficiently and generically.

- **Thiserror**
  
  A library for deriving error types, simplifying error handling in Rust.

- **Log**
  
  Provides a logging facade, allowing for configurable log output.

- **Env_logger**
  
  A logger implementation for Rust that works with the `log` crate, useful for debugging.

---



