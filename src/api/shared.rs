use std::collections::HashMap;
use cynic::http::{CynicReqwestError};

use serde_json::json;
use crate::api::classes::LevelSpellcasting;
use crate::Character;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] CynicReqwestError),
    #[error("Schema error")]
    Schema,
}

//noinspection RsCompileErrorMacro
#[cynic::schema("dnd5eapi")]
pub(super) mod schema {}

#[derive(Debug)]
pub enum CheckError{
    InvalidRace,
    InvalidClass,
    InvalidBackground,
    InvalidAlignment,
    InvalidAbilities
}

mod race_query {
    use cynic::http::ReqwestExt;
    use reqwest::Client;
    use crate::api::shared::ApiError;
    use cynic::QueryBuilder;
    use crate::{Character, GRAPHQL_API_URL};
    use super::schema;

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
                index: self.race_index.clone()
            });

            let speed = Client::new()
                .post(GRAPHQL_API_URL.as_str())
                .run_graphql(op).await?
                .data.ok_or(ApiError::Schema)?
                .race.ok_or(ApiError::Schema)?
                .speed;

            Ok(speed)
        }
    }
}


impl Character {
    pub async fn get_spellcasting_slots(&self) -> Result<HashMap<String, LevelSpellcasting>, ApiError> {
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

        let mut character = json!(self);

        if !spellcasting_slots.is_empty() {
            character["spellcasting_slots"] = json!(spellcasting_slots);
        }

        if !features.is_empty() {
            character["features"] = json!(features);
        }

        Ok(character.to_string())
    }

    /// Call this method every day to reset daily vars
    pub async fn new_day(&mut self) {
        self.hp = self.max_hp;
        self.classes.new_day().await;
    }
}
