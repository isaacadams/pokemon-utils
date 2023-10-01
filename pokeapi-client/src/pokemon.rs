use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokemonResponse {
    //pub abilities: Vec<Ability>,
    #[serde(rename = "base_experience")]
    pub base_experience: Option<i64>,
    //pub forms: Vec<Form>,
    //#[serde(rename = "game_indices")]
    //pub game_indices: Vec<Index>,
    pub height: i64,
    #[serde(rename = "held_items")]
    pub held_items: Vec<Value>,
    pub id: u16,
    #[serde(rename = "is_default")]
    pub is_default: bool,
    #[serde(rename = "location_area_encounters")]
    pub location_area_encounters: String,
    pub moves: Vec<Move>,
    pub name: String,
    pub order: i64,
    //#[serde(rename = "past_types")]
    //pub past_types: Vec<Value>,
    //pub species: Species,
    pub sprites: Sprites,
    //pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sprites {
    #[serde(rename = "back_default")]
    pub back_default: Option<String>,
    #[serde(rename = "back_female")]
    pub back_female: Option<String>,
    #[serde(rename = "back_shiny")]
    pub back_shiny: Option<String>,
    #[serde(rename = "back_shiny_female")]
    pub back_shiny_female: Option<String>,
    #[serde(rename = "front_default")]
    pub front_default: Option<String>,
    #[serde(rename = "front_female")]
    pub front_female: Option<String>,
    #[serde(rename = "front_shiny")]
    pub front_shiny: Option<String>,
    #[serde(rename = "front_shiny_female")]
    pub front_shiny_female: Option<String>,
    //pub other: Other,
    //pub versions: Versions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    #[serde(rename = "move")]
    pub move_field: super::Resource,
    #[serde(rename = "version_group_details")]
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionGroupDetail {
    #[serde(rename = "level_learned_at")]
    pub level_learned_at: i64,
    #[serde(rename = "move_learn_method")]
    pub move_learn_method: super::Resource,
    #[serde(rename = "version_group")]
    pub version_group: super::Resource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "base_stat")]
    pub base_stat: i64,
    pub effort: i64,
    pub stat: super::Resource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Type {
    pub slot: i64,
    #[serde(rename = "type")]
    pub type_field: super::Resource,
}
