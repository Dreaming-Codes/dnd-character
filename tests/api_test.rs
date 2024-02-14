#![cfg(feature = "api")]

use cynic::http::ReqwestExt;
use reqwest::Client;
use cynic::QueryBuilder;
use dnd_character::abilities::{Abilities, Ability, AbilityScore};
use dnd_character::Character;

//noinspection RsCompileErrorMacro
#[cynic::schema("dnd5eapi")]
mod schema {}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "Query")]
struct AbilitiesQuery {
    pub ability_scores: Option<Vec<GraphQlAbilityScore>>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(graphql_type = "AbilityScore")]
struct GraphQlAbilityScore {
    #[cynic(rename = "full_name")]
    pub full_name: String,
    pub index: String,
}

async fn get_abilities() -> Vec<GraphQlAbilityScore> {
    Client::new()
        .post("https://www.dnd5eapi.co/graphql")
        .run_graphql(AbilitiesQuery::build(())).await.expect("Error in API Request")
        .data.expect("Error in API Request")
        .ability_scores.expect("Error in API Request")
}

#[tokio::test]
async fn ability_api_test(){
    let abilities = get_abilities().await;

    let tasks = abilities.iter().map(|ability| {
        let ability = ability.clone();
        async move {
            assert_eq!(ability.full_name, Ability::full_name(ability.index).await.expect("Error in API Request"));
        }
    });

    futures::future::join_all(tasks).await;
}

#[tokio::test]
async fn abilities_new_test(){
    let remote_abilities = get_abilities().await;
    let local_abilities = Abilities::new().await.expect("Error in API Request");

    assert_eq!(remote_abilities.len(), local_abilities.0.len());
}

#[tokio::test]
async fn get_level_features(){
    let mut dnd_character = Character::new("cleric".to_string(), "a".to_string(), 16,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string()).await.expect("Error in API Request");

    dnd_character.add_experience(90000);

    let features = dnd_character.get_features().await.expect("Error in API Request");

    assert_eq!(features.iter().filter(|feature| feature.contains("destroy-undead-cr-4-or-below")).count(), 1);
}
