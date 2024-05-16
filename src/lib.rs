#[cfg(feature = "api")]
pub mod api;

pub mod abilities;
pub mod classes;

use std::fmt;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
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
    pub max_hp: u16,

    pub inventory: Vec<String>,

    pub other: Vec<String>,
}

const LEVELS: [u32; 19] = [300, 900, 2_700, 6_500, 14_000, 23_000, 34_000, 48_000, 64_000, 85_000, 100_000, 120_000, 140_000, 165_000, 195_000, 225_000, 265_000, 305_000, 355_000];

impl Character {
    pub fn new(main_class: String, name: String, age: u16, race_index: String, subrace_index: String, alignment_index: String, description: String, background_index: String, background_description: String) -> Self {
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
            inventory: Vec::new(),

            abilities_score: Abilities::default(),
            hp: 0,
            max_hp: 0,
            other: vec![],
        }
    }
    
    pub fn class_armor(&self) -> i8 {
        match self.classes.0.iter().next().unwrap().0.as_str() {
            "monk" => {
                10 + self.abilities_score.dexterity.modifier(0) + self.abilities_score.wisdom.modifier(0)
            }
            _ => {
                10 + self.abilities_score.dexterity.modifier(0)
            }
        }
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
