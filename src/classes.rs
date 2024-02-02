use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ClassSpellCasting {
    // Wizard
    KnowledgePrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_index: Vec<String>,
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_prepared_index: Vec<String>,
    },
    // Cleric, Paladin, Druid
    AlreadyKnowPrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_prepared_index: Vec<String>,
    },
    // Bard, Ranger, Warlock
    KnowledgeAlreadyPrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_index: Vec<String>,
        used_slots: UsableSlots,
    },
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
struct UsableSlots {
    level_1: u8,
    level_2: u8,
    level_3: u8,
    level_4: u8,
    level_5: u8,
    level_6: u8,
    level_7: u8,
    level_8: u8,
    level_9: u8,
}

impl Default for UsableSlots {
    fn default() -> Self {
        Self {
            level_1: 0,
            level_2: 0,
            level_3: 0,
            level_4: 0,
            level_5: 0,
            level_6: 0,
            level_7: 0,
            level_8: 0,
            level_9: 0,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct ClassProperties {
    pub level: u8,
    /// Index from https://www.dnd5eapi.co/api/subclasses/
    pub subclass: Option<String>,
    /// Indexes from https://www.dnd5eapi.co/api/spells/
    pub spell_casting: Option<ClassSpellCasting>,
}

impl Default for ClassProperties {
    fn default() -> Self {
        Self {
            level: 1,
            subclass: None,
            spell_casting: None,
        }
    }
}

/// The key is the index of the class from https://www.dnd5eapi.co/api/classes
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
}

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Classes(pub HashMap<String, Class>);


impl Classes {
    pub fn new(class_index: String) -> Self {
        let mut classes = Self::default();
        classes.0.insert(class_index.clone(), Class(class_index, ClassProperties::default()));
        classes
    }
}
