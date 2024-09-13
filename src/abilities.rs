use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
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

pub const ABILITY_NAMES: [&str; 6] = ["strength", "dexterity", "constitution", "intelligence", "wisdom", "charisma"];

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Abilities {
    pub strength: AbilityScore,
    pub dexterity: AbilityScore,
    pub constitution: AbilityScore,
    pub intelligence: AbilityScore,
    pub wisdom: AbilityScore,
    pub charisma: AbilityScore,
}
