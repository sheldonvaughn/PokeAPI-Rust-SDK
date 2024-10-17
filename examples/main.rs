use pokeapi_sdk::{PokeApiClient, PokeApiError};
use tokio;

#[tokio::main]
async fn main() -> Result<(), PokeApiError> {
    // Initialize the client
    let client = PokeApiClient::default();

    // Retrieve a Pokémon by ID
    match client.get_pokemon_by_id(25).await {
        Ok(pokemon) => {
            println!("Retrieved Pokémon by ID:");
            println!("Name: {}", pokemon.name);
            println!("ID: {}", pokemon.id);
            println!("Height: {}", pokemon.height);
            println!("Weight: {}", pokemon.weight);
            println!("-------------------------");
        }
        Err(e) => {
            eprintln!("Error retrieving Pokémon by ID:");
            handle_error(e, "Pokémon");
        }
    }

    // Retrieve a Pokémon by name (intentionally incorrect to trigger error)
    match client.get_pokemon_by_name("notreal").await {
        Ok(pokemon) => {
            println!("Retrieved Pokémon by Name:");
            println!("Name: {}", pokemon.name);
            println!("ID: {}", pokemon.id);
            println!("Height: {}", pokemon.height);
            println!("Weight: {}", pokemon.weight);
            println!("-------------------------");
        }
        Err(e) => {
            eprintln!("Error retrieving Pokémon by Name:");
            handle_error(e, "Pokémon");
        }
    }

    // Retrieve a Generation by ID
    match client.get_generation_by_id(1).await {
        Ok(generation) => {
            println!("Retrieved Generation by ID:");
            println!("Generation Name: {}", generation.name);
            println!("Main Region: {}", generation.main_region.name);
            println!("-------------------------");
        }
        Err(e) => {
            eprintln!("Error retrieving Generation by ID:");
            handle_error(e, "Generation");
        }
    }

    // Retrieve a Generation by name
    match client.get_generation_by_name("generation-i").await {
        Ok(generation) => {
            println!("Retrieved Generation by Name:");
            println!("Generation Name: {}", generation.name);
            println!("Main Region: {}", generation.main_region.name);
            println!("-------------------------");
        }
        Err(e) => {
            eprintln!("Error retrieving Generation by Name:");
            handle_error(e, "Generation");
        }
    }

    // List first 10 Pokémon without autopagination
    let pokemon_list = client.list_pokemon(Some(10), None, false).await?;
    println!("First 10 Pokémon:");
    for pokemon in &pokemon_list {
        println!("- {}", pokemon.name);
    }
    println!("-------------------------");

    // Autopaginate to get all Pokémon (catch 'em all!)
    // Be cautious: This may take time and consume resources
    let all_pokemon = client.list_pokemon(None, None, true).await?;
    println!(
        "Total Pokémon fetched with autopagination: {}",
        all_pokemon.len()
    );
    println!("List of all Pokémon:");
    for pokemon in &all_pokemon {
        println!("- {}", pokemon.name);
    }

    // List all generations (autopagination not necessary as there are few)
    let generations = client.list_generations(None, None, false).await?;
    println!("Generations:");
    for gen in &generations {
        println!("- {}", gen.name);
    }
    println!("-------------------------");

    Ok(())
}

/// Handles errors and provides user-friendly messages.
fn handle_error(error: PokeApiError, resource: &str) {
    if error.is_not_found() {
        eprintln!("{} not found.", resource);
        eprintln!("Please check if the name or ID is correct.");
    } else if let Some(status) = error.status_code() {
        if (500..=599).contains(&status) {
            eprintln!(
                "Server error (status code {}) when accessing {}.",
                status, resource
            );
            eprintln!("Please try again later.");
        } else {
            eprintln!(
                "API error (status code {}) when accessing {}.",
                status, resource
            );
        }
    } else {
        match error {
            PokeApiError::RequestError(err) => {
                eprintln!("Network request error: {}", err);
                eprintln!("Please check your internet connection.");
            }
            PokeApiError::DeserializationError(err) => {
                eprintln!("Failed to parse the response for {}: {}", resource, err);
                eprintln!("This might be due to an unexpected response format.");
            }
            _ => {
                eprintln!("An unexpected error occurred: {}", error);
            }
        }
    }
}
