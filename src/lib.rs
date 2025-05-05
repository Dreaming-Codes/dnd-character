#[cfg(feature = "api")]
pub mod api;

pub mod abilities;
pub mod classes;

use abilities::AbilityScore;
use anyhow::bail;
use lazy_static::lazy_static;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::abilities::Abilities;
use crate::classes::Classes;

#[cfg(feature = "serde")]
mod abilities_score_serde {
    use crate::abilities::Abilities;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn serialize<S>(
        abilities: &Rc<RefCell<Abilities>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        abilities.borrow().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Rc<RefCell<Abilities>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let abilities = Abilities::deserialize(deserializer)?;
        Ok(Rc::new(RefCell::new(abilities)))
    }
}

#[cfg(feature = "serde")]
mod classes_serde {
    use crate::abilities::Abilities;
    use crate::classes::Classes;
    use serde::de::Error;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn serialize<S>(classes: &Classes, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        classes.serialize(serializer)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
        abilities_ref: &Rc<RefCell<Abilities>>,
    ) -> Result<Classes, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First deserialize into a serde_json::Value
        let value = serde_json::Value::deserialize(deserializer)
            .map_err(|e| D::Error::custom(format!("Failed to deserialize classes: {}", e)))?;

        // Use the custom deserializer that takes the shared abilities reference
        Classes::deserialize_with_abilities(value, abilities_ref.clone())
            .map_err(|e| D::Error::custom(format!("Failed to deserialize with abilities: {}", e)))
    }
}

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
#[cfg_attr(feature = "serde", serde(from = "CharacterDeserializeHelper"))]
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

    #[cfg_attr(feature = "serde", serde(with = "abilities_score_serde"))]
    pub abilities_score: Rc<RefCell<Abilities>>,

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

#[cfg(feature = "serde")]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CharacterDeserializeHelper {
    name: String,
    age: u16,
    race_index: String,
    subrace_index: String,
    alignment_index: String,
    description: String,
    background_index: String,
    background_description: String,
    experience_points: u32,
    money: u32,
    abilities_score: Abilities,
    hp: u16,
    #[serde(default = "default_hit_dice")]
    hit_dice_result: u16,
    inventory: HashMap<String, u16>,
    other: Vec<String>,
    #[serde(default)]
    classes: serde_json::Value,
}

#[cfg(feature = "serde")]
impl From<CharacterDeserializeHelper> for Character {
    fn from(helper: CharacterDeserializeHelper) -> Self {
        // Create the shared abilities reference
        let abilities_score = Rc::new(RefCell::new(helper.abilities_score));

        // Deserialize classes with the shared abilities reference
        let classes =
            match Classes::deserialize_with_abilities(helper.classes, abilities_score.clone()) {
                Ok(classes) => classes,
                Err(_) => Classes::default(),
            };

        Self {
            classes,
            name: helper.name,
            age: helper.age,
            race_index: helper.race_index,
            subrace_index: helper.subrace_index,
            alignment_index: helper.alignment_index,
            description: helper.description,
            background_index: helper.background_index,
            background_description: helper.background_description,
            experience_points: helper.experience_points,
            money: helper.money,
            abilities_score,
            hp: helper.hp,
            hit_dice_result: helper.hit_dice_result,
            inventory: helper.inventory,
            other: helper.other,
        }
    }
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
        // Create the shared abilities reference
        let abilities_score = Rc::new(RefCell::new(Abilities::default()));

        // Create classes with the default implementation
        let mut classes = Classes::new(main_class);

        // Update all class properties to use the shared abilities reference
        for class in classes.0.values_mut() {
            class.1.abilities_modifiers = abilities_score.clone();
        }

        Self {
            classes,
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

            abilities_score,
            hp: 0,
            hit_dice_result: 0,
            other: vec![],
        }
    }

    pub fn class_armor(&self) -> i8 {
        // Get the first class and its name
        let first_class = self.classes.0.iter().next().unwrap();
        let class_name = first_class.0.as_str();

        let abilities_score = self.abilities_score.borrow();

        // Calculate the base armor class based on the class type
        let mut base = match class_name {
            "monk" => {
                10 + abilities_score.dexterity.modifier(0) + abilities_score.wisdom.modifier(0)
            }
            "sorcerer" => 13 + abilities_score.dexterity.modifier(0),
            "barbarian" => {
                10 + abilities_score.dexterity.modifier(0)
                    + abilities_score.constitution.modifier(0)
            }
            _ => 10 + abilities_score.dexterity.modifier(0),
        };

        // Check if the character has the "Fighting Style: Defense" feature
        let has_defense_style = first_class
            .1
            .1
            .fighting_style
            .as_ref()
            .map(|s| s.contains("defense"))
            .unwrap_or(false)
            || first_class
                .1
                .1
                .additional_fighting_style
                .as_ref()
                .map(|s| s.contains("defense"))
                .unwrap_or(false);

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
        let constitution_modifier = self.abilities_score.borrow().constitution.modifier(0);

        (constitution_modifier as i32)
            .saturating_mul(self.level().into())
            .saturating_add(self.hit_dice_result.into())
            .max(0) as u16
    }
}
