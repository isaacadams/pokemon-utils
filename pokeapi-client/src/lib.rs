use serde::{Deserialize, Serialize};

pub mod characteristic;
pub mod pokemon;
pub mod species;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    pub name: String,
    pub url: String,
}

const BASE: &str = "https://pokeapi.co/api/v2/";

pub async fn get_pokemon(no: u16) -> Result<pokemon::PokemonResponse, reqwest::Error> {
    let response = reqwest::get(format!("{}/pokemon/{}", BASE, &no)).await?;
    response.json().await
}

pub async fn get_species(no: u16) -> Result<species::SpeciesResponse, reqwest::Error> {
    let response = reqwest::get(format!("{}/pokemon-species/{}", BASE, &no)).await?;
    response.json().await
}

pub async fn get_characteristic(
    no: u16,
) -> Result<characteristic::CharacteristicResponse, reqwest::Error> {
    let response = reqwest::get(format!("{}/characteristic/{}", BASE, &no)).await?;
    response.json().await
}

pub async fn get_all_sprites(no: u16) -> Result<Vec<PokemonSprite>, reqwest::Error> {
    let mut images = Vec::new();
    let response = reqwest::get(format!("{}/pokemon/{}", BASE, &no)).await?;
    let root: serde_json::Value = response.json().await?;
    match root {
        serde_json::Value::Object(root) => {
            for (key, value) in root {
                if &key != "sprites" {
                    continue;
                }

                match value {
                    serde_json::Value::Object(sprites) => {
                        for (key, value) in sprites {
                            extract_string(&mut images, key, value);
                        }
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    };
    Ok(images)
}

pub struct PokemonSprite {
    label: String,
    url: String,
}

impl PokemonSprite {
    pub fn to_slack_message(self) -> serde_json::Value {
        serde_json::json!({
            "type": "context",
            "elements": [
                {
                    "type": "image",
                    "image_url": self.url,
                    "alt_text": self.label
                },
                {
                    "type": "mrkdwn",
                    "text": self.label
                }
            ]
        })
    }
}

pub fn extract_string(buffer: &mut Vec<PokemonSprite>, key: String, value: serde_json::Value) {
    match value {
        serde_json::Value::String(x) => {
            buffer.push(PokemonSprite { label: key, url: x });
        }
        serde_json::Value::Object(o) => {
            for (child_key, value) in o {
                extract_string(buffer, format!("{}/{}", key, child_key), value);
            }
        }
        _ => {}
    };
}

/*

pub fn get_all_pokemon() -> Result<NamedAPIResourceList, reqwest::Error> {
    let response = reqwest::blocking::get(format!("{}/{}?limit=3000", BASE, POKEMON))?;
    response.json()
}
 */

#[cfg(test)]
mod test {

    use super::*;

    /*
    use crate::pokemon::models::pokemon::PokemonResponse;

    pub fn serde_error_peek(json: &str) {
        let text = &json[15500..15650];
        println!("{}", text);
    }

    #[tokio::test]
    async fn testing_broken_pokemon() {
        let response = reqwest::get(format!("{}/pokemon/{}", BASE, &1011))
            .await
            .unwrap();
        let text: PokemonResponse = response.json().await.unwrap();
        //serde_error_peek(response.text().await.unwrap().as_ref());
    } */

    #[tokio::test]
    async fn get_sprites() {
        let sprites = get_all_sprites(1).await.unwrap();
        println!("found {}", sprites.len());
        let sprites = serde_json::Value::Array(
            sprites
                .into_iter()
                .skip(50)
                .take(50)
                .map(|s| s.to_slack_message())
                .collect::<Vec<serde_json::Value>>(),
        );
        std::fs::write(
            "./message.json",
            serde_json::json!({
                "blocks": sprites
            })
            .to_string(),
        );
    }

    #[tokio::test]
    async fn edge_case_pokemon_works() {
        let response = get_pokemon(704).await;
        assert!(response.is_ok());

        let response = get_pokemon(657).await;
        assert!(response.is_ok());

        let response = get_pokemon(899).await;
        assert!(response.is_ok());
        let response = get_species(899).await;
        assert!(response.is_ok());

        let response = get_pokemon(1011).await;
        assert!(response.is_ok());
        let response = get_species(1011).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn pokemon_check_bounds() {
        let response = get_pokemon(0).await;
        assert!(response.is_err());

        /* let response = get_pokemon(1018).await;
        assert!(response.is_err()); */
    }

    /*  #[tokio::test]
    async fn searching_for_breaking_pokemon() {
        for i_u16 in 1_011..=1_200 {
            println!("catching {}...", i_u16);
            let response = get_pokemon(i_u16).await;
            assert!(response.is_ok(), "no {} failed", i_u16);
        }
    } */
}
