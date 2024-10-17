use crate::error::PokeApiError;
use crate::models::{Generation, NamedAPIResource, NamedAPIResourceList, Pokemon};
use log::debug;
use reqwest::Client;

/// Client for interacting with the PokeAPI.
#[derive(Clone)]
pub struct PokeApiClient {
    base_url: String,
    client: Client,
}

impl Default for PokeApiClient {
    /// Creates a new `PokeApiClient` with default settings.
    fn default() -> Self {
        PokeApiClient {
            base_url: "https://pokeapi.co/api/v2/".to_string(),
            client: Client::builder()
                .user_agent("pokeapi-sdk-rust/0.1.0")
                .build()
                .expect("Failed to build HTTP client"),
        }
    }
}

impl PokeApiClient {
    /// Creates a new `PokeApiClient` with a custom base URL.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL for the PokeAPI.
    pub fn with_base_url<S: Into<String>>(base_url: S) -> Self {
        PokeApiClient {
            base_url: base_url.into(),
            ..Default::default()
        }
    }

    /// Retrieves a Pokémon by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID (`u32`) of the Pokémon.
    pub async fn get_pokemon_by_id(&self, id: u32) -> Result<Pokemon, PokeApiError> {
        let endpoint = format!("pokemon/{}", id);
        self.get_resource(&endpoint).await
    }

    /// Retrieves a Pokémon by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name (`&str` or `String`) of the Pokémon.
    pub async fn get_pokemon_by_name<S>(&self, name: S) -> Result<Pokemon, PokeApiError>
    where
        S: AsRef<str>,
    {
        let endpoint = format!("pokemon/{}", name.as_ref());
        self.get_resource(&endpoint).await
    }

    /// Retrieves a Generation by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID (`u32`) of the Generation.
    pub async fn get_generation_by_id(&self, id: u32) -> Result<Generation, PokeApiError> {
        let endpoint = format!("generation/{}", id);
        self.get_resource(&endpoint).await
    }

    /// Retrieves a Generation by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name (`&str` or `String`) of the Generation.
    pub async fn get_generation_by_name<S>(&self, name: S) -> Result<Generation, PokeApiError>
    where
        S: AsRef<str>,
    {
        let endpoint = format!("generation/{}", name.as_ref());
        self.get_resource(&endpoint).await
    }

    /// Retrieves a list of Pokémon with optional pagination.
    ///
    /// # Arguments
    ///
    /// * `limit` - Optional limit for the number of resources per page.
    /// * `offset` - Optional offset from the start of the list.
    /// * `autopaginate` - Whether to automatically retrieve all pages.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use pokeapi_sdk::PokeApiClient;
    /// # #[tokio::main]
    /// # async fn main() {
    /// let client = PokeApiClient::default();
    /// let pokemon_list = client.list_pokemon(Some(100), None, false).await.unwrap();
    /// println!("Retrieved {} Pokémon", pokemon_list.len());
    /// # }
    /// ```
    pub async fn list_pokemon(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        autopaginate: bool,
    ) -> Result<Vec<NamedAPIResource>, PokeApiError> {
        self._get_paginated_resources("pokemon", limit, offset, autopaginate)
            .await
    }

    /// Retrieves a list of generations with optional pagination.
    ///
    /// # Arguments
    ///
    /// * `limit` - Optional limit for the number of resources per page.
    /// * `offset` - Optional offset from the start of the list.
    /// * `autopaginate` - Whether to automatically retrieve all pages.
    pub async fn list_generations(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        autopaginate: bool,
    ) -> Result<Vec<NamedAPIResource>, PokeApiError> {
        self._get_paginated_resources("generation", limit, offset, autopaginate)
            .await
    }

    /// Internal helper function to retrieve paginated resources.
    async fn _get_paginated_resources(
        &self,
        endpoint: &str,
        limit: Option<u32>,
        offset: Option<u32>,
        autopaginate: bool,
    ) -> Result<Vec<NamedAPIResource>, PokeApiError> {
        let mut results = Vec::new();
        let mut next_url = Some({
            let mut url = format!("{}{}", self.base_url, endpoint);
            let mut query_params = Vec::new();
            if let Some(limit) = limit {
                query_params.push(format!("limit={}", limit));
            }
            if let Some(offset) = offset {
                query_params.push(format!("offset={}", offset));
            }
            if !query_params.is_empty() {
                url = format!("{}?{}", url, query_params.join("&"));
            }
            url
        });

        while let Some(url) = next_url {
            debug!("Requesting URL: {}", url);

            let response = self
                .client
                .get(&url)
                .send()
                .await
                .map_err(PokeApiError::RequestError)?;

            let status = response.status();
            debug!("Response status: {}", status);

            if !status.is_success() {
                return Err(PokeApiError::ApiError {
                    status: status.as_u16(),
                    url,
                });
            }

            let page: NamedAPIResourceList = response
                .json()
                .await
                .map_err(PokeApiError::DeserializationError)?;

            results.extend(page.results);

            if autopaginate {
                next_url = page.next;
            } else {
                break;
            }
        }

        Ok(results)
    }

    /// Generic method to fetch a resource from a given endpoint.
    async fn get_resource<T>(&self, endpoint: &str) -> Result<T, PokeApiError>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let url = format!("{}{}", self.base_url, endpoint);
        debug!("Requesting URL: {}", url);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(PokeApiError::RequestError)?;

        let status = response.status();
        debug!("Response status: {}", status);

        if !status.is_success() {
            return Err(PokeApiError::ApiError {
                status: status.as_u16(),
                url,
            });
        }

        let data = response
            .json::<T>()
            .await
            .map_err(PokeApiError::DeserializationError)?;

        Ok(data)
    }
}
