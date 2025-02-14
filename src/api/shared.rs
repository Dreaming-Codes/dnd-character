use cynic::http::CynicReqwestError;
use std::collections::HashMap;

use crate::api::classes::LevelSpellcasting;
use crate::Character;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] CynicReqwestError),
    #[error("Schema error")]
    Schema,
    #[error("Toml serialization error")]
    TomlError(#[from] toml::ser::Error),
}

//noinspection RsCompileErrorMacro
#[cynic::schema("dnd5eapi")]
pub(super) mod schema {}

#[derive(Debug)]
pub enum CheckError {
    InvalidRace,
    InvalidClass,
    InvalidBackground,
    InvalidAlignment,
    InvalidAbilities,
}

mod race_query {
    use super::schema;
    use crate::api::shared::ApiError;
    use crate::{Character, GRAPHQL_API_URL};
    use cynic::http::ReqwestExt;
    use cynic::QueryBuilder;
    use reqwest::Client;

    #[derive(cynic::QueryVariables, Debug)]
    struct SpeedQueryVariables {
        pub index: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SpeedQueryVariables")]
    struct SpeedQuery {
        #[arguments(index: $index)]
        pub race: Option<RaceSpeed>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Race")]
    struct RaceSpeed {
        pub speed: i32,
    }

    impl Character {
        pub async fn get_base_speed(&self) -> Result<i32, ApiError> {
            let op = SpeedQuery::build(SpeedQueryVariables {
                index: self.race_index.clone(),
            });

            let speed = Client::new()
                .post(GRAPHQL_API_URL.as_str())
                .run_graphql(op)
                .await?
                .data
                .ok_or(ApiError::Schema)?
                .race
                .ok_or(ApiError::Schema)?
                .speed;

            Ok(speed)
        }
    }
}

impl Character {
    pub async fn get_spellcasting_slots(
        &self,
    ) -> Result<HashMap<String, LevelSpellcasting>, ApiError> {
        let mut spellcasting_slots = HashMap::new();
        for class in self.classes.0.iter() {
            let spellcasting_slots_class = class.1.get_spellcasting_slots().await?;
            if let Some(spellcasting_slots_class) = spellcasting_slots_class {
                spellcasting_slots.insert(class.0.clone(), spellcasting_slots_class);
            }
        }
        Ok(spellcasting_slots)
    }

    pub async fn get_features(&self, passive: bool) -> Result<Vec<String>, ApiError> {
        let mut features = Vec::new();
        for class in self.classes.0.iter() {
            let features_class = class.1.get_levels_features(None, passive).await?;
            features.extend(features_class);
        }
        Ok(features)
    }

    #[cfg(feature = "serde")]
    pub async fn rich_print(&self) -> Result<String, ApiError> {
        let spellcasting_slots = self.get_spellcasting_slots().await?;
        let features = self.get_features(true).await?;

        // Convert `self` to TOML
        let mut character = toml::Value::try_from(self)?;

        if !spellcasting_slots.is_empty() {
            character.as_table_mut().unwrap().insert(
                "spellcasting_slots".to_string(),
                toml::Value::try_from(spellcasting_slots)?,
            );
        }

        if !features.is_empty() {
            character
                .as_table_mut()
                .unwrap()
                .insert("features".to_string(), toml::Value::try_from(features)?);
        }

        Ok(toml::to_string_pretty(&character)?)
    }

    /// Call this method every day to reset daily vars
    pub async fn new_day(&mut self) {
        self.hp = self.max_hp;
        self.classes.new_day().await;
    }
}
