use std::collections::HashMap;
use super::shared::schema;
use cynic::http::ReqwestExt;
use reqwest::Client;
use crate::api::shared::ApiError;
use cynic::QueryBuilder;
use lazy_static::lazy_static;
use serde::{Serialize};
use crate::classes::Class;

#[derive(cynic::QueryVariables, Debug)]
struct SpellcastingAbilityQueryVariables {
    pub index: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SpellcastingAbilityQueryVariables")]
struct SpellcastingAbilityQuery {
    #[arguments(index: $ index)]
    pub class: Option<ClassSpellCasting>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Class")]
struct ClassSpellCasting {
    pub spellcasting: Option<ClassSpellcasting>,
}

#[derive(cynic::QueryFragment, Debug)]
struct ClassSpellcasting {
    #[cynic(rename = "spellcasting_ability")]
    pub spellcasting_ability: AbilityScore,
}

#[derive(cynic::QueryFragment, Debug)]
struct AbilityScore {
    pub index: String,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct SpellcastingQueryVariables {
    pub index: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SpellcastingQueryVariables")]
pub struct SpellcastingQuery {
    #[arguments(index: $ index)]
    pub level: Option<Level>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Level {
    pub spellcasting: Option<LevelSpellcasting>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
pub struct LevelSpellcasting {
    #[cynic(rename = "cantrips_known")]
    pub cantrips_known: Option<i32>,
    #[cynic(rename = "spell_slots_level_1")]
    pub spell_slots_level_1: Option<i32>,
    #[cynic(rename = "spell_slots_level_2")]
    pub spell_slots_level_2: Option<i32>,
    #[cynic(rename = "spell_slots_level_3")]
    pub spell_slots_level_3: Option<i32>,
    #[cynic(rename = "spell_slots_level_4")]
    pub spell_slots_level_4: Option<i32>,
    #[cynic(rename = "spell_slots_level_5")]
    pub spell_slots_level_5: Option<i32>,
    #[cynic(rename = "spell_slots_level_6")]
    pub spell_slots_level_6: Option<i32>,
    #[cynic(rename = "spell_slots_level_7")]
    pub spell_slots_level_7: Option<i32>,
    #[cynic(rename = "spell_slots_level_8")]
    pub spell_slots_level_8: Option<i32>,
    #[cynic(rename = "spell_slots_level_9")]
    pub spell_slots_level_9: Option<i32>,
}

#[derive(cynic::QueryVariables, Debug)]
pub struct LevelFeaturesQueryVariables {
    pub class: Option<StringFilter>,
    pub level: Option<IntFilter>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "LevelFeaturesQueryVariables")]
pub struct LevelFeaturesQuery {
    #[arguments(level: $ level, class: $ class)]
    pub features: Option<Vec<Feature>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Feature {
    pub index: String,
}

#[derive(cynic::Scalar, Debug, Clone)]
pub struct IntFilter(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
pub struct StringFilter(pub String);

pub enum CustomLevelFeature {
    /// Ask the user to spend 2 points in any ability score
    AbilityScoreImprovement,
    /// https://www.dnd5eapi.co/api/features/pact-boon
    WarlockPact,
    /// Ignore this feature, since we have only one subclass per class
    SubclassChoice,
    /// https://www.dnd5eapi.co/api/features/additional-fighting-style
    AdditionalFighterFightingStyle,
    /// https://www.dnd5eapi.co/api/features/bonus-proficiencies
    BonusBardProficiency,
    HeavyArmorProficiency,
    /// https://www.dnd5eapi.co/api/features/beast-spells
    /// This feature will not be implemented for now
    /// TODO: Implement
    BeastSpells,
    /// Used for
    /// https://www.dnd5eapi.co/api/features/bard-expertise-1
    /// https://www.dnd5eapi.co/api/features/bard-expertise-2
    /// https://www.dnd5eapi.co/api/features/rogue-expertise-1
    /// https://www.dnd5eapi.co/api/features/rogue-expertise-2
    MultiplyTwoSkillProficiency,
    /// https://www.dnd5eapi.co/api/features/magical-secrets-1
    /// https://www.dnd5eapi.co/api/features/magical-secrets-2
    /// https://www.dnd5eapi.co/api/features/magical-secrets-3
    ChooseTwoSpellForAnyClass,
    /// https://www.dnd5eapi.co/api/features/mystic-arcanum-6th-level
    /// https://www.dnd5eapi.co/api/features/mystic-arcanum-7th-level
    /// https://www.dnd5eapi.co/api/features/mystic-arcanum-8th-level
    /// https://www.dnd5eapi.co/api/features/mystic-arcanum-9th-level
    ChooseOne6thLevelSpellFromWarlockList,
    /// https://www.dnd5eapi.co/api/features/paladin-fighting-style
    PaladinFightingStyle,
    /// https://www.dnd5eapi.co/api/features/primal-champion
    PrimalChampion,
    /// https://www.dnd5eapi.co/api/features/diamond-soul
    ProficiencyInAllSkill,
    /// This is for features already handled by other parts of the code and not needed to be managed as "features"
    Ignored,
}

impl CustomLevelFeature {
    pub fn identify(index: String) -> Option<CustomLevelFeature> {
        use CustomLevelFeature::*;
        match index.as_str() {
            "bard-college" | "divine-domain" | "monastic-tradition" | "sacred-oath" | "ranger-archetype" | "sorcerous-origin" | "druid-circle" | "primal-path" | "martial-archetype" | "otherworldly-patron" => Some(SubclassChoice),
            "pact-boon" => Some(WarlockPact),
            "additional-fighting-style" => Some(AdditionalFighterFightingStyle),
            "beast-spells" => Some(BeastSpells),
            "bonus-proficiencies" => Some(BonusBardProficiency),
            "bonus-proficiency" => Some(HeavyArmorProficiency),
            "additional-magical-secrets" | "bonus-cantrip" => Some(Ignored),
            "channel-divinity-1-rest" | "channel-divinity-2-rest" | "channel-divinity-3-rest" => Some(Ignored),
            "magical-secrets-1" | "magical-secrets-2" | "magical-secrets-3" => Some(ChooseTwoSpellForAnyClass),
            "mystic-arcanum-6th-level" | "mystic-arcanum-7th-level" | "mystic-arcanum-8th-level" | "mystic-arcanum-9th-level" => Some(ChooseOne6thLevelSpellFromWarlockList),
            "paladin-fighting-style" => Some(PaladinFightingStyle),
            "primal-champion" => Some(PrimalChampion),
            "diamond-soul" => Some(ProficiencyInAllSkill),
            x if x.starts_with("bard-expertise-") || x.starts_with("rogue-expertise-") => Some(MultiplyTwoSkillProficiency),
            x if x.starts_with("spellcasting-") => Some(Ignored),
            x if x.contains("ability-score-improvement") => Some(AbilityScoreImprovement),
            _ => None
        }
    }
}


impl Class {
    pub async fn get_spellcasting_ability_index(&self) -> Result<String, ApiError> {
        let op = SpellcastingAbilityQuery::build(SpellcastingAbilityQueryVariables {
            index: Some(self.index().to_string())
        });

        let ability_index = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .class.ok_or(ApiError::Schema)?
            .spellcasting.ok_or(ApiError::Schema)?
            .spellcasting_ability.index;

        Ok(ability_index)
    }

    pub async fn get_spellcasting_slots(&self) -> Result<Option<LevelSpellcasting>, ApiError> {
        let op = SpellcastingQuery::build(SpellcastingQueryVariables {
            index: Some(format!("{}-{}", self.index(), self.1.level))
        });

        let spellcasting_slots = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .level.ok_or(ApiError::Schema)?
            .spellcasting;

        Ok(spellcasting_slots)
    }

    pub async fn set_level(&mut self, new_level: u8) -> Result<Vec<CustomLevelFeature>, ApiError> {
        let op = LevelFeaturesQuery::build(LevelFeaturesQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
            level: Some(IntFilter(format!("{{ gte: {}, lte: {} }}", self.1.level, new_level))),
        });

        let features = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .features.ok_or(ApiError::Schema)?;

        let mut pending_features = vec![];

        features.iter().filter_map(|feature| {
            CustomLevelFeature::identify(feature.index.clone())
        }).for_each(|feature| {
            match feature {
                CustomLevelFeature::BeastSpells | CustomLevelFeature::SubclassChoice | CustomLevelFeature::Ignored => {}
                _ => {
                    pending_features.push(feature);
                }
            }
        });

        self.1.level = new_level;

        Ok(pending_features)
    }

    pub async fn get_levels_features(&self, from_level: Option<u8>) -> Result<Vec<String>, ApiError> {
        let op = LevelFeaturesQuery::build(LevelFeaturesQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
            level: Some(IntFilter(format!("{{ gte: {}, lte: {} }}", from_level.unwrap_or(0), self.1.level))),
        });

        let features = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .features.ok_or(ApiError::Schema)?;

        // Remove all identifiable features
        let mut features: Vec<String> = features.into_iter().filter(|feature| {
            CustomLevelFeature::identify(feature.index.clone()).is_none()
        }).map(|feature| feature.index).collect();

        let features: Vec<String> = {
            lazy_static! {
                static ref CR_REGEX: regex::Regex = regex::Regex::new(r"destroy-undead-cr-([0-9]+(?:-[0-9]+)?)\-or-below").unwrap();
            }

            let mut found = false;

            features.iter_mut().rev().filter(|feature| {
                if CR_REGEX.is_match(feature) {
                    if found {
                        false
                    } else {
                        found = true;
                        true
                    }
                } else {
                    true
                }
            }).map(|feature| feature.clone()).collect()
        };

        lazy_static! {
            static ref DICE_REGEX: regex::Regex = regex::Regex::new(r"^(.+)-d(\d+)$").unwrap();
        }

        let mut grouped_features: HashMap<String, u32> = HashMap::new();
        for feature in &features {
            if let Some(caps) = DICE_REGEX.captures(feature) {
                if caps.len() == 3 {
                    let prefix = caps.get(1).unwrap().as_str().to_string();
                    let dice_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

                    let current_max = grouped_features.entry(prefix).or_insert(0);
                    if dice_value > *current_max {
                        *current_max = dice_value;
                    }
                }
            }
        }

        let features = features.into_iter().filter(|feature| {
            if let Some(caps) = DICE_REGEX.captures(feature) {
                let prefix = caps.get(1).unwrap().as_str();
                let dice_value = caps.get(2).unwrap().as_str().parse::<u32>().expect("Parsing dice value");

                if let Some(&max_dice) = grouped_features.get(prefix) {
                    return dice_value == max_dice;
                }
            }
            true
        }).collect();


        Ok(features)
    }
}
