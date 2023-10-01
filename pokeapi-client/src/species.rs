use serde::{Deserialize, Serialize};
use serde_json::Value;

impl SpeciesResponse {
    pub fn get_name<'a>(&'a self, language: &'static str) -> Option<&'a str> {
        self.names
            .iter()
            .find(|n| n.language.name == language)
            .map(|n| n.name.as_str())
    }

    pub fn get_description(&self, language: &'static str) -> Option<String> {
        self.flavor_text_entries
            .iter()
            .find(|t| t.language.name == language)
            .map(|t| {
                t.flavor_text
                    .replace("\n.", ".")
                    .replace("\n", " ")
                    .replace(".\u{000c}", ". ")
                    .replace('\u{000c}', "")
            })
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesResponse {
    #[serde(rename = "base_happiness")]
    pub base_happiness: Option<i64>,
    #[serde(rename = "capture_rate")]
    pub capture_rate: i64,
    pub color: super::Resource,
    #[serde(rename = "egg_groups")]
    pub egg_groups: Vec<super::Resource>,
    #[serde(rename = "evolution_chain")]
    pub evolution_chain: EvolutionChain,
    #[serde(rename = "evolves_from_species")]
    pub evolves_from_species: Value,
    #[serde(rename = "flavor_text_entries")]
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    #[serde(rename = "form_descriptions")]
    pub form_descriptions: Vec<Value>,
    #[serde(rename = "forms_switchable")]
    pub forms_switchable: bool,
    #[serde(rename = "gender_rate")]
    pub gender_rate: i64,
    pub genera: Vec<Genera>,
    pub generation: super::Resource,
    #[serde(rename = "growth_rate")]
    pub growth_rate: super::Resource,
    pub habitat: Option<super::Resource>,
    #[serde(rename = "has_gender_differences")]
    pub has_gender_differences: bool,
    #[serde(rename = "hatch_counter")]
    pub hatch_counter: Option<i64>,
    pub id: i64,
    #[serde(rename = "is_baby")]
    pub is_baby: bool,
    #[serde(rename = "is_legendary")]
    pub is_legendary: bool,
    #[serde(rename = "is_mythical")]
    pub is_mythical: bool,
    pub name: String,
    pub names: Vec<Name>,
    pub order: i64,
    #[serde(rename = "pal_park_encounters")]
    pub pal_park_encounters: Vec<PalParkEncounter>,
    #[serde(rename = "pokedex_numbers")]
    pub pokedex_numbers: Vec<PokedexNumber>,
    pub shape: Option<super::Resource>,
    pub varieties: Vec<Variety>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionChain {
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlavorTextEntry {
    #[serde(rename = "flavor_text")]
    pub flavor_text: String,
    pub language: super::Resource,
    pub version: super::Resource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genera {
    pub genus: String,
    pub language: super::Resource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub language: super::Resource,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PalParkEncounter {
    pub area: super::Resource,
    #[serde(rename = "base_score")]
    pub base_score: i64,
    pub rate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PokedexNumber {
    #[serde(rename = "entry_number")]
    pub entry_number: i64,
    pub pokedex: super::Resource,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variety {
    #[serde(rename = "is_default")]
    pub is_default: bool,
    pub pokemon: super::Resource,
}
