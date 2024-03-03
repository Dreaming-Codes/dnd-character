use cynic::http::ReqwestExt;
use cynic::QueryBuilder;
use reqwest::Client;
use crate::classes::{Class};
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
pub struct Spell {
    pub index: String,
    pub level: i32,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct StringFilter(pub String);

impl Class {
    pub async fn get_spells(&self) -> Result<Vec<String>, ApiError> {
        let op = SpellsQuery::build(SpellsQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
        });

        let spells = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .spells.ok_or(ApiError::Schema)?;

        Ok(spells.iter().map(|spell| spell.index.clone()).collect())
    }
}