use serde::{Deserialize, Serialize};

pub fn get_english_description(descriptions: &[Description]) -> Option<&str> {
    descriptions
        .iter()
        .find(|d| d.language.name == "en")
        .map(|d| d.description.as_str())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CharacteristicResponse {
    pub descriptions: Vec<Description>,
    #[serde(rename = "gene_modulo")]
    pub gene_modulo: i64,
    #[serde(rename = "highest_stat")]
    pub highest_stat: super::Resource,
    pub id: i64,
    #[serde(rename = "possible_values")]
    pub possible_values: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub description: String,
    pub language: super::Resource,
}
