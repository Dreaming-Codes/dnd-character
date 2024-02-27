#![cfg(feature = "api")]

use dnd_character::Character;

#[tokio::test]
async fn get_level_features(){
    let mut dnd_character = Character::new("cleric".to_string(), "a".to_string(), 16,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string()).await.expect("Error in API Request");

    dnd_character.add_experience(90000);

    let features = dnd_character.get_features(true).await.expect("Error in API Request");

    assert_eq!(features.iter().filter(|feature| feature.starts_with("destroy-undead-cr-")).count(), 1);

    let mut dnd_character = Character::new("bard".to_string(), "a".to_string(), 16,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string()).await.expect("Error in API Request");

    dnd_character.add_experience(90000);

    let features = dnd_character.get_features(true).await.expect("Error in API Request");

    assert_eq!(features.iter().filter(|feature| feature.starts_with("song-of-rest-")).count(), 1);

    let features = dnd_character.get_features(false).await.expect("Error in API Request");

    assert!(!features.contains(&"druidic".to_string()));
}
