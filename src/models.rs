use serde::Deserialize;

/// Represents a named API resource with a URL.
#[derive(Deserialize, Debug)]
pub struct NamedApiResource {
    pub name: String,
    pub url: String,
}

/// Represents a list of named API resources (paginated).
#[derive(Deserialize, Debug)]
pub struct NamedApiResourceList {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<NamedApiResource>,
}

/// Represents a Pokémon.
#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub base_experience: u32,
    pub height: u32,
    pub weight: u32,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<NamedApiResource>,
    #[serde(rename = "game_indices")]
    pub game_indices: Vec<VersionGameIndex>,
    pub held_items: Vec<PokemonHeldItem>,
    #[serde(rename = "location_area_encounters")]
    pub location_area_encounters: String,
    pub moves: Vec<PokemonMove>,
    pub sprites: PokemonSprites,
    pub species: NamedApiResource,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonType>,
}

/// Represents a Pokémon's ability.
#[derive(Deserialize, Debug)]
pub struct PokemonAbility {
    #[serde(rename = "is_hidden")]
    pub is_hidden: bool,
    pub slot: u8,
    pub ability: NamedApiResource,
}

/// Represents a version game index.
#[derive(Deserialize, Debug)]
pub struct VersionGameIndex {
    #[serde(rename = "game_index")]
    pub game_index: u16,
    pub version: NamedApiResource,
}

/// Represents an item that a Pokémon holds.
#[derive(Deserialize, Debug)]
pub struct PokemonHeldItem {
    pub item: NamedApiResource,
    pub version_details: Vec<PokemonHeldItemVersion>,
}

/// The details of the different versions in which the item is held.
#[derive(Deserialize, Debug)]
pub struct PokemonHeldItemVersion {
    pub version: NamedApiResource,
    pub rarity: u8,
}

/// Represents a Pokémon move.
#[derive(Deserialize, Debug)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub move_field: NamedApiResource,
    #[serde(rename = "version_group_details")]
    pub version_group_details: Vec<PokemonMoveVersion>,
}

/// Represents the version group details of how a Pokémon can learn a move.
#[derive(Deserialize, Debug)]
pub struct PokemonMoveVersion {
    #[serde(rename = "move_learn_method")]
    pub move_learn_method: NamedApiResource,
    #[serde(rename = "version_group")]
    pub version_group: NamedApiResource,
    pub level_learned_at: u8,
}

/// Represents a set of sprites used to depict a Pokémon in the game.
#[derive(Deserialize, Debug)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    // Additional sprite fields can be added here.
}

/// Represents a Pokémon stat.
#[derive(Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: NamedApiResource,
    pub effort: u8,
    pub base_stat: u8,
}

/// Represents a Pokémon type.
#[derive(Deserialize, Debug)]
pub struct PokemonType {
    pub slot: u8,
    #[serde(rename = "type")]
    pub type_field: NamedApiResource,
}

/// Represents a Pokémon Generation.
#[derive(Deserialize, Debug)]
pub struct Generation {
    pub id: u32,
    pub name: String,
    pub abilities: Vec<NamedApiResource>,
    pub names: Vec<Name>,
    pub main_region: NamedApiResource,
    pub moves: Vec<NamedApiResource>,
    #[serde(rename = "pokemon_species")]
    pub pokemon_species: Vec<NamedApiResource>,
    pub types: Vec<NamedApiResource>,
    #[serde(rename = "version_groups")]
    pub version_groups: Vec<NamedApiResource>,
}

/// Represents a localized name.
#[derive(Deserialize, Debug)]
pub struct Name {
    pub name: String,
    pub language: NamedApiResource,
}
