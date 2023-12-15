use cynic::http::{ReqwestExt};
use cynic::QueryBuilder;
use reqwest::Client;
use crate::{Abilities, Ability};
use crate::abilities::AbilityScore;
use crate::api::shared::ApiError;

use super::shared::schema;

impl Ability {
    pub async fn full_name(index: String) -> Result<String, ApiError> {
        let op = AbilityFullNameQuery::build(AbilityFullNameQueryVariables {
            index
        });
        Ok(Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .ability_score.ok_or(ApiError::Schema)?
            .full_name)
    }
}

#[derive(cynic::QueryVariables, Debug)]
struct AbilityFullNameQueryVariables {
    pub index: String,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "AbilityFullNameQueryVariables")]
struct AbilityFullNameQuery {
    #[arguments(index: $ index)]
    pub ability_score: Option<GraphQlFullNameAbilityScore>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "AbilityScore")]
struct GraphQlFullNameAbilityScore {
    #[cynic(rename = "full_name")]
    pub full_name: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "Query")]
struct AbilitiesQuery {
    pub ability_scores: Option<Vec<GraphQlAbilityScore>>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "AbilityScore")]
struct GraphQlAbilityScore {
    pub index: String,
}

impl Abilities {
    pub async fn new() -> Result<Self, ApiError> {
        let op = AbilitiesQuery::build(());

        let abilities = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .ability_scores.ok_or(ApiError::Schema)?;

        Ok(Self(abilities.iter().map(|ability| (ability.index.clone(),Ability(AbilityScore::new(0, false)))).collect()))
    }
}
