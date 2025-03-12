use std::{
    iter::Sum,
    ops::{Add, AddAssign},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct AbilityScore {
    pub score: u8,
    pub proficiency: bool,
}

impl Add for AbilityScore {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            score: self.score + other.score,
            proficiency: self.proficiency || other.proficiency,
        }
    }
}

impl AddAssign for AbilityScore {
    fn add_assign(&mut self, other: Self) {
        self.score += other.score;
        self.proficiency = self.proficiency || other.proficiency;
    }
}

impl Sum for AbilityScore {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = AbilityScore::default();

        for ability in iter {
            total += ability;
        }

        total
    }
}

impl AbilityScore {
    pub fn new(score: u8, proficiency: bool) -> Self {
        Self { score, proficiency }
    }
    /// Returns the modifier of the ability score
    /// if you want to add the proficiency bonus, pass it as an argument otherwise pass 0
    pub fn modifier(&self, proficiency_bonus: u8) -> i8 {
        ((self.score as i8 - 10) as f32 / 2f32).floor() as i8
            + if self.proficiency {
                proficiency_bonus as i8
            } else {
                0
            }
    }
}

pub const ABILITY_NAMES: [&str; 6] = [
    "strength",
    "dexterity",
    "constitution",
    "intelligence",
    "wisdom",
    "charisma",
];

#[derive(Debug, Default, Clone)]
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

impl Add for Abilities {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            strength: self.strength + other.strength,
            dexterity: self.dexterity + other.dexterity,
            constitution: self.constitution + other.constitution,
            intelligence: self.intelligence + other.intelligence,
            wisdom: self.wisdom + other.wisdom,
            charisma: self.charisma + other.charisma,
        }
    }
}

impl AddAssign for Abilities {
    fn add_assign(&mut self, other: Self) {
        self.strength += other.strength;
        self.dexterity += other.dexterity;
        self.constitution += other.constitution;
        self.intelligence += other.intelligence;
        self.wisdom += other.wisdom;
        self.charisma += other.charisma;
    }
}

impl Sum for Abilities {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut total = Abilities::default();

        for abilities_set in iter {
            total += abilities_set; // Uses AddAssign
        }

        total
    }
}
