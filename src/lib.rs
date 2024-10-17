//! # PokeAPI SDK
//!
//! A Rust SDK for the [PokeAPI](https://pokeapi.co/), providing easy access to Pokémon data.
//!
//! ## Example
//!
//! ```no_run
//! use pokeapi_sdk::PokeApiClient;
//! use tokio;
//! ```

pub mod client;
pub mod error;
pub mod models;

#[cfg(test)]
mod tests;

pub use client::PokeApiClient;
pub use error::PokeApiError;
