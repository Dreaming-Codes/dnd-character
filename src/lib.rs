#[cfg(feature = "api")]
pub mod api;

pub mod abilities;
pub mod classes;

use abilities::AbilityScore;
use anyhow::{anyhow, bail};
use api::classes::ChoosableCustomLevelFeatureOption;
use lazy_static::lazy_static;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;

use crate::abilities::Abilities;
use crate::classes::Classes;

lazy_static! {
    pub static ref GRAPHQL_API_URL: String = std::env::var("DND_GRAPHQL_API_URL")
        .unwrap_or_else(|_| "https://www.dnd5eapi.co/graphql/2014".to_string());
}

#[derive(Debug)]
pub struct UnexpectedAbility;

impl fmt::Display for UnexpectedAbility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The ability isn't present in the character's abilities")
    }
}

impl std::error::Error for UnexpectedAbility {}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Character {
    /// Indexes from https://www.dnd5eapi.co/api/classes/
    pub classes: Classes,
    pub name: String,
    pub age: u16,
    /// Index from https://www.dnd5eapi.co/api/races/
    pub race_index: String,
    /// Index from https://www.dnd5eapi.co/api/subraces/
    pub subrace_index: String,
    /// Index from https://www.dnd5eapi.co/api/alignments/
    pub alignment_index: String,
    /// Physical description
    pub description: String,
    /// Index from https://www.dnd5eapi.co/api/backgrounds/
    pub background_index: String,
    /// Background description
    pub background_description: String,

    experience_points: u32,

    pub money: u32,

    pub abilities_score: Abilities,

    //Health related stuff
    pub hp: u16,
    #[serde(default = "default_hit_dice")]
    pub hit_dice_result: u16,

    pub inventory: HashMap<String, u16>,

    pub other: Vec<String>,
}

/// For parsing legacy support
fn default_hit_dice() -> u16 {
    12
}

#[cfg(feature = "utoipa")]
pub mod utoipa_addon {
    use utoipa::openapi::OpenApi;
    use utoipa::{Modify, PartialSchema, ToSchema};

    pub struct ApiDocDndCharacterAddon;

    impl Modify for ApiDocDndCharacterAddon {
        fn modify(&self, openapi: &mut OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.schemas.insert(
                    super::classes::ClassProperties::name().to_string(),
                    super::classes::ClassProperties::schema(),
                );
                components.schemas.insert(
                    super::classes::ClassSpellCasting::name().to_string(),
                    super::classes::ClassSpellCasting::schema(),
                );
                components.schemas.insert(
                    super::classes::Class::name().to_string(),
                    super::classes::Class::schema(),
                );
                components
                    .schemas
                    .insert(super::Classes::name().to_string(), super::Classes::schema());
                components.schemas.insert(
                    super::classes::UsableSlots::name().to_string(),
                    super::classes::UsableSlots::schema(),
                );
                components.schemas.insert(
                    super::Abilities::name().to_string(),
                    super::Abilities::schema(),
                );
                components.schemas.insert(
                    super::abilities::AbilityScore::name().to_string(),
                    super::abilities::AbilityScore::schema(),
                );
                components.schemas.insert(
                    super::Character::name().to_string(),
                    super::Character::schema(),
                );
            }
        }
    }
}

const LEVELS: [u32; 19] = [
    300, 900, 2_700, 6_500, 14_000, 23_000, 34_000, 48_000, 64_000, 85_000, 100_000, 120_000,
    140_000, 165_000, 195_000, 225_000, 265_000, 305_000, 355_000,
];

impl Character {
    pub fn new(
        main_class: String,
        name: String,
        age: u16,
        race_index: String,
        subrace_index: String,
        alignment_index: String,
        description: String,
        background_index: String,
        background_description: String,
    ) -> Self {
        Self {
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
            inventory: HashMap::new(),

            abilities_score: Abilities::default(),
            hp: 0,
            hit_dice_result: 0,
            other: vec![],
        }
    }

    pub fn class_armor(&self) -> i8 {
        // Get the first class and its name
        let first_class = self.classes.0.iter().next().unwrap();
        let class_name = first_class.0.as_str();

        // Calculate the base armor class based on the class type
        let mut base = match class_name {
            "monk" => {
                10 + self.abilities_score.dexterity.modifier(0)
                    + self.abilities_score.wisdom.modifier(0)
            }
            _ => 10 + self.abilities_score.dexterity.modifier(0),
        };

        // Check if the character has the "Fighting Style: Defense" feature
        let has_defense_style = first_class.1 .1.fighting_style
            == Some(
                ChoosableCustomLevelFeatureOption::FightingStyleDefense
                    .as_index_str()
                    .to_string(),
            );

        // Add bonus if the character has the defense fighting style
        if has_defense_style {
            base += 1;
        }

        base
    }

    /// Return current level of the character
    pub fn level(&self) -> u8 {
        LEVELS
            .iter()
            .filter(|&&x| x <= self.experience_points)
            .count() as u8
            + 1
    }

    /// Returns the experience points of the character
    pub fn experience_points(&self) -> u32 {
        self.experience_points
    }

    /// Returns the number of levels the character has earned
    /// this means that you should add the returned value to a class level (this must be done manually to permit multiclassing)
    /// # Arguments
    /// * `experience` - The experience points to add to the character
    pub fn add_experience(&mut self, experience: u32) -> u8 {
        //Save the level before adding experience
        let previous_level = self.level();

        // Limit the experience gotten to the experience needed to reach the next level
        let experience_to_add = LEVELS
            .get(self.level() as usize - 1)
            .map_or(experience, |&next_level_points| {
                (next_level_points - self.experience_points).min(experience)
            });

        //Add the experience
        self.experience_points += experience_to_add;

        //Save the level after adding experience
        let current_level = self.level();

        //Return the number of levels earned
        current_level - previous_level
    }

    pub fn remove_item(
        &mut self,
        item: &str,
        amount: Option<u16>,
    ) -> anyhow::Result<(), anyhow::Error> {
        if let Some(quantity) = self.inventory.get_mut(item) {
            let quantity_to_remove = amount.unwrap_or(*quantity);

            if *quantity <= quantity_to_remove {
                self.inventory.remove(item);
            } else {
                *quantity -= quantity_to_remove;
            }
        } else {
            bail!("Item not found")
        }

        Ok(())
    }

    pub fn add_item(&mut self, item: &str, amount: u16) {
        if let Some(quantity) = self.inventory.get_mut(item) {
            *quantity += amount;
        } else {
            self.inventory.insert(item.to_string(), amount);
        }
    }

    pub fn alter_item_quantity(&mut self, item: &str, amount: i32) -> anyhow::Result<()> {
        match amount.cmp(&0) {
            Ordering::Greater => {
                self.add_item(item, amount as u16);
                Ok(())
            }
            Ordering::Less => self.remove_item(item, Some(amount.unsigned_abs() as u16)),
            Ordering::Equal => {
                bail!("Cannot alter quantity to 0")
            }
        }
    }

    /// Calculate the maximum HP of the character based on constitution modifier and hit dice result
    pub fn max_hp(&self) -> u16 {
        let constitution_ability: AbilityScore = self
            .classes
            .0
            .values()
            .map(|class| class.1.abilities_modifiers.constitution.clone())
            .sum::<AbilityScore>()
            + self.abilities_score.constitution.clone();

        let constitution_modifier = constitution_ability.modifier(0);

        (constitution_modifier as i32)
            .saturating_mul(self.level().into())
            .saturating_add(self.hit_dice_result.into())
            .max(0) as u16
    }
}
