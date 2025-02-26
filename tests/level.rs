use dnd_character::Character;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChatPendingChoice {
    pub index: String,
    pub choices: Vec<Vec<String>>,
}

fn ability_score_improvement() -> ChatPendingChoice {
    ChatPendingChoice {
        index: "abilityScoreImprovement".to_string(),
        choices: vec![
            vec![
                "strength-plus-one".to_string(),
                "dexterity-plus-one".to_string(),
                "constitution-plus-one".to_string(),
                "intelligence-plus-one".to_string(),
                "wisdom-plus-one".to_string(),
                "charisma-plus-one".to_string(),
            ],
            vec![
                "strength-plus-one".to_string(),
                "dexterity-plus-one".to_string(),
                "constitution-plus-one".to_string(),
                "intelligence-plus-one".to_string(),
                "wisdom-plus-one".to_string(),
                "charisma-plus-one".to_string(),
            ],
        ],
    }
}

/// Test helper to level up a character and verify the expected pending choices
async fn test_level_up(
    character: &mut Character,
    target_level: u8,
    expected_choices: &HashMap<u8, Vec<ChatPendingChoice>>,
) -> Result<(), String> {
    let current_level = character.level();

    if target_level <= current_level {
        return Err(format!(
            "Target level {} must be greater than current level {}",
            target_level, current_level
        ));
    }

    // XP thresholds for each level (0-indexed array, level 1 is at index 0)
    let xp_thresholds = [
        0, 300, 900, 2700, 6500, 14000, 23000, 34000, 48000, 64000, 85000, 100000, 120000, 140000,
        165000, 195000, 225000, 265000, 305000, 355000,
    ];

    // Calculate required XP to reach target level
    let required_xp = xp_thresholds[target_level as usize - 1] - character.experience_points();

    // Add experience to reach the target level
    let levels_gained = character.add_experience(required_xp);

    // Verify we reached the expected level
    if character.level() != target_level {
        return Err(format!(
            "Failed to reach target level {}. Current level: {}",
            target_level,
            character.level()
        ));
    }

    // Get the class to check for choosable features
    let class = character
        .classes
        .0
        .iter_mut()
        .next()
        .ok_or_else(|| "Failed to get character class".to_string())?;

    // Get choosable features for the new level
    let choosable_features = class
        .1
        .set_level(class.1 .1.level + levels_gained)
        .await
        .map_err(|_| "Failed to get choosable features".to_string())?;

    // Convert features to ChatPendingChoice format
    let pending_choices = choosable_features
        .iter()
        .map(|feature| ChatPendingChoice {
            index: feature.as_index_str().to_string(),
            choices: feature
                .to_options()
                .iter()
                .map(|option| {
                    option
                        .iter()
                        .map(|option| option.as_index_str().to_string())
                        .collect::<Vec<String>>()
                })
                .collect(),
        })
        .collect::<Vec<ChatPendingChoice>>();

    match expected_choices.get(&target_level) {
        Some(expected) => {
            assert_eq!(
            pending_choices, *expected,
            "Pending choices at level {} don't match expected choices. \nGot: {:?}\nExpected: {:?}",
            target_level, pending_choices, expected
        );
        }
        None => {
            assert!(
                pending_choices.is_empty(),
                "Expected no pending choices at level {}, but got: {:?}",
                target_level,
                pending_choices
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_character_progression_to_level_20() {
    // Create a new character
    let mut character = Character::new(
        "barbarian".to_string(),
        "Test Character".to_string(),
        16,
        "human".to_string(),
        "human".to_string(),
        "chaotic-neutral".to_string(),
        "bard".to_string(),
        "".to_string(),
        "".to_string(),
    );

    // Verify initial state
    assert_eq!(character.experience_points(), 0);
    assert_eq!(character.level(), 1);

    // Define expected pending choices for each level
    // This can be customized based on the class and expected features
    let mut expected_choices: HashMap<u8, Vec<ChatPendingChoice>> = HashMap::new();

    expected_choices.insert(5, vec![ability_score_improvement()]);
    expected_choices.insert(9, vec![ability_score_improvement()]); // Adding level 9
    expected_choices.insert(13, vec![ability_score_improvement()]);
    expected_choices.insert(17, vec![ability_score_improvement()]);
    expected_choices.insert(20, vec![ability_score_improvement()]);

    // Test progression through all levels
    for level in 2..=20 {
        let result = test_level_up(&mut character, level, &expected_choices).await;
        assert!(
            result.is_ok(),
            "Failed to level up to level {}: {:?}",
            level,
            result.err()
        );

        // Verify we're at the expected level
        assert_eq!(
            character.level(),
            level,
            "Character should be at level {} but is at level {}",
            level,
            character.level()
        );

        // Print level-up information for debugging
        println!(
            "Level {}: XP = {}, Next level at: {}",
            level,
            character.experience_points(),
            match level {
                20 => "Max level".to_string(),
                _ => format!("{} XP", get_xp_for_level(level + 1)),
            }
        );
    }

    // Verify we reached level 20
    assert_eq!(character.level(), 20, "Character should reach level 20");
}

/// Helper function to get XP required for a specific level
fn get_xp_for_level(level: u8) -> u32 {
    match level {
        1 => 0,
        2 => 300,
        3 => 900,
        4 => 2700,
        5 => 6500,
        6 => 14000,
        7 => 23000,
        8 => 34000,
        9 => 48000,
        10 => 64000,
        11 => 85000,
        12 => 100000,
        13 => 120000,
        14 => 140000,
        15 => 165000,
        16 => 195000,
        17 => 225000,
        18 => 265000,
        19 => 305000,
        20 => 355000,
        _ => 355000, // Cap at level 20
    }
}
