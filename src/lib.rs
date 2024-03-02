#[cfg(feature = "api")]
pub mod api;

pub mod abilities;
pub mod classes;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde::Deserializer;

use crate::abilities::{Abilities};
use crate::classes::Classes;

#[derive(Debug)]
pub struct UnexpectedAbility;

impl fmt::Display for UnexpectedAbility {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The ability isn't present in the character's abilities")
    }
}

impl std::error::Error for UnexpectedAbility {}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Character {
    /// Indexes from https://www.dnd5eapi.co/api/classes/
    pub classes: Rc<RefCell<Classes>>,
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

    pub abilities_score: Rc<RefCell<Abilities>>,

    //Health related stuff
    pub hp: u16,
    pub max_hp: u16,

    pub inventory: Vec<String>,

    pub armor_class: u8,

    pub other: Vec<String>,
}

impl<'de> Deserialize<'de> for Character {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        // Deserialize the Character struct normally
        #[derive(Deserialize)]
        struct CharacterHelper {
            classes: Classes,
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
            max_hp: u16,
            inventory: Vec<String>,
            armor_class: u8,
            other: Vec<String>,
        }

        let helper = CharacterHelper::deserialize(deserializer)?;

        // Create the Character struct and wrap the `classes` field in an Rc
        let character = Character {
            classes: Rc::new(RefCell::new(helper.classes)),
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
            abilities_score: Rc::new(RefCell::new(helper.abilities_score)),
            hp: helper.hp,
            max_hp: helper.max_hp,
            inventory: helper.inventory,
            armor_class: helper.armor_class,
            other: helper.other,
        };

        // Iterate over the classes and set the `character` field
        for class in character.classes.borrow_mut().0.iter_mut() {
            class.1.1.set_parent(Rc::downgrade(&character.abilities_score));
        }

        Ok(character)
    }
}

const LEVELS: [u32; 19] = [300, 900, 2_700, 6_500, 14_000, 23_000, 34_000, 48_000, 64_000, 85_000, 100_000, 120_000, 140_000, 165_000, 195_000, 225_000, 265_000, 305_000, 355_000];

impl Character {
    pub fn new(main_class: String, name: String, age: u16, race_index: String, subrace_index: String, alignment_index: String, description: String, background_index: String, background_description: String) -> Self {
        let character = Self {
            classes: Rc::new(RefCell::new(Classes::new(main_class))),
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
            inventory: Vec::new(),

            abilities_score: Rc::new(RefCell::new(Abilities::default())),
            armor_class: 0,
            hp: 0,
            max_hp: 0,
            other: vec![],
        };

        for class in character.classes.borrow_mut().0.iter_mut() {
            class.1.1.set_parent(Rc::downgrade(&character.abilities_score));
        }

        character
    }

    /// Return current level of the character
    pub fn level(&self) -> u8 {
        LEVELS.iter().filter(|&&x| x <= self.experience_points).count() as u8 + 1
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

        //Add the experience
        self.experience_points += experience;

        //Save the level after adding experience
        let current_level = self.level();

        //Return the number of levels earned
        current_level - previous_level
    }
}
