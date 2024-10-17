# PokeAPI SDK

A Rust SDK for the [PokeAPI](https://pokeapi.co/), providing easy access to Pokémon data.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)


## Features

- Retrieve detailed information about Pokémon by ID or name.
- Fetch data about Pokémon Generations.
- Asynchronous API calls using `async`/`await`.
- Comprehensive error handling with custom error types.
- Autopagination

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
pokeapi_sdk = "0.1.0"
```

## Usage

Retrieving a Pokémon by ID

```rust
use pokeapi_sdk::PokeApiClient;
use tokio;

#[tokio::main]
async fn main() {
    let client = PokeApiClient::default();

    // Retrieve a Pokémon by ID
    match client.get_pokemon_by_id(25).await {
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

Listing Pokèmon with Pagination

**PokeAPI uses the same functions with no parameter to return paginated results**

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

