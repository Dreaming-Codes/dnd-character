use crate::abilities::Abilities;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ClassSpellCasting {
    // Wizard
    // Ask the user to prepare spells at the start of the day
    //
    // TODO: Add slots and consume them instead of removing from prepared
    // TODO: daily chosable spells = inteligence + level
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
    //
    // TODO: Add slots and consume them instead of removing from prepared
    // TODO: cleric/druid daily chosable spells = WISDOM + (level/2)
    // TODO: paladin daily chosable spells = CHARISMA + (level/2)
    AlreadyKnowPrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_prepared_index: Vec<Vec<String>>,
        /// If the user has already prepared spells for the day
        pending_preparation: bool,
    },
    // Bard, Ranger, Warlock, (Sorcerer?)
    // No need to ask anything, at the start of the day
    KnowledgeAlreadyPrepared {
        /// Indexes from https://www.dnd5eapi.co/api/spells/
        spells_index: Vec<Vec<String>>,
        usable_slots: UsableSlots,
    },
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UsableSlots {
    pub cantrip_slots: u8,
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

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ClassProperties {
    /// The level of the class
    pub level: u8,
    /// Index from https://www.dnd5eapi.co/api/subclasses/
    pub subclass: Option<String>,
    /// Indexes from https://www.dnd5eapi.co/api/spells/
    pub spell_casting: Option<ClassSpellCasting>,
    pub fighting_style: Option<String>,
    pub hunters_prey: Option<String>,
    pub defensive_tactics: Option<String>,
    pub additional_fighting_style: Option<String>,
    pub multiattack: Option<String>,
    pub superior_hunters_defense: Option<String>,
    pub abilities_modifiers: Abilities,
}

/// The key is the index of the class from https://www.dnd5eapi.co/api/classes
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Classes(pub HashMap<String, Class>);

impl Classes {
    pub fn new(class_index: String) -> Self {
        let mut classes = Self::default();

        let spell_casting = match class_index.as_str() {
            "cleric" | "paladin" | "druid" => Some(ClassSpellCasting::AlreadyKnowPrepared {
                spells_prepared_index: Vec::new(),
                pending_preparation: true,
            }),
            "ranger" | "bard" | "warlock" => Some(ClassSpellCasting::KnowledgeAlreadyPrepared {
                spells_index: Vec::new(),
                usable_slots: UsableSlots::default(),
            }),
            _ => None,
        };

        let class_properties = ClassProperties {
            spell_casting,
            ..ClassProperties::default()
        };

        classes
            .0
            .insert(class_index.clone(), Class(class_index, class_properties));
        classes
    }
}
