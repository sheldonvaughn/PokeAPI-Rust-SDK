use crate::client::PokeApiClient;
use crate::error::PokeApiError;

#[tokio::test]
async fn test_get_pokemon_by_id_success() {
    let client = PokeApiClient::default();
    let result = client.get_pokemon_by_id(25).await;
    assert!(result.is_ok());
    let pokemon = result.unwrap();
    assert_eq!(pokemon.name, "pikachu");
    assert_eq!(pokemon.id, 25);
}

#[tokio::test]
async fn test_get_pokemon_by_name_success() {
    let client = PokeApiClient::default();
    let result = client.get_pokemon_by_name("pikachu").await;
    assert!(result.is_ok());
    let pokemon = result.unwrap();
    assert_eq!(pokemon.name, "pikachu");
    assert_eq!(pokemon.id, 25);
}

#[tokio::test]
async fn test_get_pokemon_not_found() {
    let client = PokeApiClient::default();
    let result = client.get_pokemon_by_name("unknown_pokemon").await;
    assert!(matches!(
        result,
        Err(PokeApiError::ApiError { status: 404, .. })
    ));
}

#[tokio::test]
async fn test_get_generation_by_id_success() {
    let client = PokeApiClient::default();
    let result = client.get_generation_by_id(1).await;
    assert!(result.is_ok());
    let generation = result.unwrap();
    assert_eq!(generation.name, "generation-i");
    assert_eq!(generation.id, 1);
}

#[tokio::test]
async fn test_get_generation_by_name_success() {
    let client = PokeApiClient::default();
    let result = client.get_generation_by_name("generation-i").await;
    assert!(result.is_ok());
    let generation = result.unwrap();
    assert_eq!(generation.name, "generation-i");
    assert_eq!(generation.id, 1);
}

#[tokio::test]
async fn test_get_generation_not_found() {
    let client = PokeApiClient::default();
    let result = client.get_generation_by_name("unknown_generation").await;
    assert!(matches!(
        result,
        Err(PokeApiError::ApiError { status: 404, .. })
    ));
}

#[tokio::test]
async fn test_list_pokemon_with_limit() {
    let client = PokeApiClient::default();
    let result = client.list_pokemon(Some(10), None, false).await;
    assert!(result.is_ok());
    let pokemon_list = result.unwrap();
    assert_eq!(pokemon_list.len(), 10);
}

#[tokio::test]
async fn test_list_pokemon_autopaginate() {
    let client = PokeApiClient::default();
    let result = client.list_pokemon(None, None, true).await;
    assert!(result.is_ok());
    let pokemon_list = result.unwrap();
    assert!(pokemon_list.len() > 1000); // Assuming there are over 1000 Pokémon
}

#[tokio::test]
async fn test_list_generations() {
    let client = PokeApiClient::default();
    let result = client.list_generations(None, None, false).await;
    assert!(result.is_ok());
    let generations = result.unwrap();
    assert!(generations.len() > 0);
}
