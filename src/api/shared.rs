use std::collections::HashMap;
use std::fmt;
use cynic::http::{CynicReqwestError, ReqwestExt};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::abilities::Abilities;
use crate::api::classes::LevelSpellcasting;
use crate::Character;
use crate::classes::Classes;

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

#[derive(Debug)]
pub enum NewError {
    CheckError(CheckError),
    ApiError(ApiError),
}

impl fmt::Display for NewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The ability isn't present in the character's abilities")
    }
}

impl std::error::Error for NewError {}

impl From<ApiError> for NewError {
    fn from(e: ApiError) -> Self {
        Self::ApiError(e)
    }
}

impl From<CynicReqwestError> for NewError {
    fn from(e: CynicReqwestError) -> Self {
        Self::ApiError(e.into())
    }
}

mod race_query {
    use cynic::http::ReqwestExt;
    use reqwest::Client;
    use crate::api::shared::ApiError;
    use cynic::QueryBuilder;
    use crate::Character;
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
                .post("https://www.dnd5eapi.co/graphql")
                .run_graphql(op).await?
                .data.ok_or(ApiError::Schema)?
                .race.ok_or(ApiError::Schema)?
                .speed;

            Ok(speed)
        }
    }
}


impl Character {
    pub async fn new(main_class: String, name: String, age: u16, race_index: String, subrace_index: String, alignment_index: String, description: String, background_index: String, background_description: String) -> Result<Self, NewError> {
        Ok(Self {
            classes: Classes::new(main_class),
            name,
            age,
            race_index,
            subrace_index,
            alignment_index,
            description,
            background_index,
            background_description,
            experience_points: 0,
            money: 0,
            inventory: Vec::new(),

            abilities_score: Abilities::new().await?,
            armor_class: 0,
            hp: 0,
            max_hp: 0,
            other: vec![],
        })
    }

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

    pub async fn rich_print(&self) -> Result<String, ApiError> {
        let spellcasting_slots = self.get_spellcasting_slots().await?;

        let mut character = json!(self);

        if !spellcasting_slots.is_empty() {
            character["spellcasting_slots"] = json!(spellcasting_slots);
        }

        Ok(character.to_string())
    }
}
