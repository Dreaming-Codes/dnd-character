use std::collections::{HashMap};
use std::hash::Hash;
use serde::{Deserialize, Serialize};
use crate::abilities::Abilities;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ClassSpellCasting {
    // Wizard
    // Ask the user to prepare spells at the start of the day
    KnowledgePrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_index: Vec<Vec<String>>,
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_prepared_index: Vec<Vec<String>>,
        /// If the user has already prepared spells for the day
        pending_preparation: bool,
    },
    // Cleric, Paladin, Druid
    // Ask the user to prepare spells at the start of the day
    AlreadyKnowPrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_prepared_index: Vec<Vec<String>>,
        /// If the user has already prepared spells for the day
        pending_preparation: bool,
    },
    // Bard, Ranger, Warlock
    // No need to ask anything, at the start of the day
    KnowledgeAlreadyPrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_index: Vec<Vec<String>>,
        usable_slots: UsableSlots,
    },
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct UsableSlots {
    pub level_1: u8,
    pub level_2: u8,
    pub level_3: u8,
    pub level_4: u8,
    pub level_5: u8,
    pub level_6: u8,
    pub level_7: u8,
    pub level_8: u8,
    pub level_9: u8,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ClassProperties {
    /// The level of the class
    pub level: u8,
    /// Index from https://www.dnd5eapi.co/api/subclasses/
    pub subclass: Option<String>,
    /// Indexes from https://www.dnd5eapi.co/api/spells/
    pub spell_casting: Option<ClassSpellCasting>,
    pub fighting_style: Option<String>,
    pub additional_fighting_style: Option<String>,
    pub abilities_modifiers: Abilities,
}

/// The key is the index of the class from https://www.dnd5eapi.co/api/classes
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Class(String, pub ClassProperties);

impl Hash for Class {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for Class {}

impl Class {
    pub fn index(&self) -> &str {
        &self.0
    }

    pub fn hit_dice(&self) -> u8 {
        match self.index() {
            "barbarian" => 12,
            "bard" => 8,
            "cleric" => 8,
            "druid" => 8,
            "fighter" => 10,
            "monk" => 8,
            "paladin" => 10,
            "ranger" => 10,
            "rogue" => 8,
            "sorcerer" => 6,
            "warlock" => 8,
            "wizard" => 6,
            // For unknown classes we will use the minimum hit dice
            _ => 6,
        }
    }
}

#[derive(Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Classes(pub HashMap<String, Class>);


impl Classes {
    pub fn new(class_index: String) -> Self {
        let mut classes = Self::default();

        let spell_casting = match class_index.as_str() {
            "cleric" | "paladin" | "druid" => {
                Some(ClassSpellCasting::AlreadyKnowPrepared {
                    spells_prepared_index: Vec::new(),
                    pending_preparation: true,
                })
            }
            _ => {
                None
            }
        };

        let class_properties = ClassProperties {
            spell_casting,
            ..ClassProperties::default()
        };

        classes.0.insert(class_index.clone(), Class(class_index, class_properties));
        classes
    }
}
