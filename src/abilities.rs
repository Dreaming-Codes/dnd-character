use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct AbilityScore {
    pub score: u8,
    pub proficiency: bool,
}

impl AbilityScore {
    pub fn new(score: u8, proficiency: bool) -> Self {
        Self {
            score,
            proficiency,
        }
    }
    /// Returns the modifier of the ability score
    /// if you want to add the proficiency bonus, pass it as an argument otherwise pass 0
    pub fn modifier(&self, proficiency_bonus: u8) -> i8 {
        ((self.score as i8 - 10) as f32 / 2f32).floor() as i8 + if self.proficiency { proficiency_bonus as i8 } else { 0 }
    }
}

/// The key is the index of the ability from https://www.dnd5eapi.co/api/ability-scores
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Ability(pub AbilityScore);

#[derive(Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Abilities(pub HashMap<String, Ability>);