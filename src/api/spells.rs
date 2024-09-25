use cynic::http::ReqwestExt;
use cynic::QueryBuilder;
use reqwest::Client;
use crate::classes::{Class, ClassSpellCasting};
use super::shared::{ApiError, schema};

#[derive(cynic::QueryVariables, Debug)]
pub struct SpellsQueryVariables {
    pub class: Option<StringFilter>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SpellsQueryVariables")]
pub struct SpellsQuery {
    #[arguments(class: $class)]
    pub spells: Option<Vec<Spell>>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Spell {
    pub index: String,
    pub level: i32,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct StringFilter(pub String);

impl Class {
    /// Returns the spells that the class can cast
    /// If it's a knowladge based class it will return the spells that the character can know
    /// If it's a prepared based class it will return the spells that the character can prepare
    pub async fn get_spells(&self) -> Result<Vec<Spell>, ApiError> {
        let op = SpellsQuery::build(SpellsQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
        });

        let spells = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .spells.ok_or(ApiError::Schema)?;

        Ok(spells)
    }

    pub async fn get_ready_spells(&self) -> Result<Vec<Vec<String>>, ApiError> {
        match &self.1.spell_casting {
            None => {
                Ok(Vec::new())
            }
            Some(spell_casting) => {
                match spell_casting {
                    ClassSpellCasting::KnowledgePrepared { .. } => {
                        Ok(Vec::new())
                    }
                    ClassSpellCasting::AlreadyKnowPrepared { spells_prepared_index, .. } => {
                        Ok(spells_prepared_index.clone())
                    }
                    ClassSpellCasting::KnowledgeAlreadyPrepared { .. } => {
                        Ok(Vec::new())
                    }
                }
            }
        }
    }
}
