#![cfg(feature = "api")]

use dnd_character::api::classes::ChoosableCustomLevelFeatureOption;
use dnd_character::Character;

#[tokio::test]
async fn get_level_features(){
    let mut dnd_character = Character::new("cleric".to_string(), "a".to_string(), 16,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string());

    dnd_character.add_experience(90000);

    let features = dnd_character.get_features(true).await.expect("Error in API Request");

    assert_eq!(features.iter().filter(|feature| feature.starts_with("destroy-undead-cr-")).count(), 1);

    let mut dnd_character = Character::new("bard".to_string(), "a".to_string(), 16,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string());

    dnd_character.add_experience(90000);

    let features = dnd_character.get_features(true).await.expect("Error in API Request");

    assert_eq!(features.iter().filter(|feature| feature.starts_with("song-of-rest-")).count(), 1);

    let features = dnd_character.get_features(false).await.expect("Error in API Request");

    assert!(!features.contains(&"druidic".to_string()));
}

#[tokio::test]
async fn primal_champion(){
    let mut dnd_character = Character::new("barbarian".to_string(), "a".to_string(), 19,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string());

    dnd_character.classes.0.iter_mut().next().unwrap().1.set_level(20).await.expect("Failed to set level");

    assert_eq!(dnd_character.abilities_score.strength.score, 4);
    assert_eq!(dnd_character.abilities_score.constitution.score, 4);
}

#[tokio::test]
async fn choosable_custom_level_feature_option_serialization() {
    let index = ChoosableCustomLevelFeatureOption::PactOfTheTome.as_index_str();
    
    assert_eq!(index, "pact-of-the-tome");

    if let Some(ChoosableCustomLevelFeatureOption::PactOfTheTome) = ChoosableCustomLevelFeatureOption::from_index_str(index) {
        // success, do nothing
    } else {
        assert!(false, "Failed to deserialize");
    }
}