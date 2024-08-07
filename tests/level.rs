use dnd_character::Character;

#[tokio::test]
async fn get_level_features(){
    let mut dnd_character = Character::new("cleric".to_string(), "a".to_string(), 16,"human".to_string(), "human".to_string(), "chaotic-neutral".to_string(), "bard".to_string(), "".to_string(), "".to_string());

    assert_eq!(dnd_character.experience_points(), 0);
    assert_eq!(dnd_character.level(), 1);

    dnd_character.add_experience(1000);

    assert_eq!(dnd_character.experience_points(), 300);
    assert_eq!(dnd_character.level(), 2);
    
    dnd_character.add_experience(100);
    assert_eq!(dnd_character.experience_points(), 400);
    assert_eq!(dnd_character.level(), 2);
    
    dnd_character.add_experience(510);
    assert_eq!(dnd_character.experience_points(), 900);
    assert_eq!(dnd_character.level(), 3);
}
