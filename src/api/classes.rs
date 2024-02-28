use std::collections::HashMap;
use super::shared::schema;
use cynic::http::ReqwestExt;
use reqwest::Client;
use crate::api::shared::ApiError;
use cynic::QueryBuilder;
use lazy_static::lazy_static;
use crate::abilities::ABILITY_NAMES;
use crate::api::classes::CustomLevelFeatureType::Ignored;
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

#[derive(cynic::QueryFragment, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ChoosableCustomLevelFeature {
    /// Ask the user to spend 2 points in any ability score
    AbilityScoreImprovement,
    /// https://www.dnd5eapi.co/api/features/pact-boon
    WarlockPact,
    /// https://www.dnd5eapi.co/api/features/additional-fighting-style
    AdditionalFighterFightingStyle,
    /// https://www.dnd5eapi.co/api/features/ranger-fighting-style
    RangerFightingStyle,
    /// https://www.dnd5eapi.co/api/features/bonus-proficiencies
    BonusBardProficiency,
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
    PaladinFightingStyle
}

impl ChoosableCustomLevelFeature {
    #[cfg(feature = "serde")]
    pub fn as_index_str(&self) -> &str {
        serde_variant::to_variant_name(self).unwrap()
    }

    pub fn to_options(&self) -> Vec<Vec<String>> {
        match self {
            ChoosableCustomLevelFeature::AbilityScoreImprovement => {
                let ability_names = ABILITY_NAMES.iter().map(|ability| format!("{}+1", ability)).collect::<Vec<String>>();

                vec![
                    ability_names.clone(),
                    ability_names,
                ]
            }
            ChoosableCustomLevelFeature::WarlockPact => {
                vec![
                    vec!["pact-of-the-chain".to_string(), "pact-of-the-blade".to_string(), "pact-of-the-tome".to_string()]
                ]
            }
            ChoosableCustomLevelFeature::AdditionalFighterFightingStyle => {
                vec![
                    vec![
                        "fighter-fighting-style-archery".to_string(),
                        "fighter-fighting-style-defense".to_string(),
                        "fighter-fighting-style-dueling".to_string(),
                        "fighter-fighting-style-great-weapon-fighting".to_string(),
                        "fighter-fighting-style-protection".to_string(),
                        "fighter-fighting-style-two-weapon-fighting".to_string()
                    ]
                ]
            }
            ChoosableCustomLevelFeature::RangerFightingStyle => {
                vec![
                    vec![
                        "ranger-fighting-style-archery".to_string(),
                        "ranger-fighting-style-defense".to_string(),
                        "ranger-fighting-style-dueling".to_string(),
                        "ranger-fighting-style-two-weapon-fighting".to_string()
                    ]
                ]
            }
            ChoosableCustomLevelFeature::BonusBardProficiency => {
                let ability_names = ABILITY_NAMES.iter().map(|ability| format!("{}-proficiency", ability)).collect::<Vec<String>>();

                vec![
                    ability_names.clone(),
                    ability_names.clone(),
                    ability_names,
                ]
            }
            ChoosableCustomLevelFeature::MultiplyTwoSkillProficiency => {
                // TODO: Implement this
                vec![vec![]]
            }
            ChoosableCustomLevelFeature::ChooseTwoSpellForAnyClass => {
                // TODO: Implement this
                vec![vec![]]
            }
            ChoosableCustomLevelFeature::ChooseOne6thLevelSpellFromWarlockList => {
                // TODO: Implement this when other warlock features are implemented
                vec![vec![]]
            }
            ChoosableCustomLevelFeature::PaladinFightingStyle => {
                vec![
                    vec![
                        "fighting-style-defense".to_string(),
                        "fighting-style-dueling".to_string(),
                        "fighting-style-great-weapon-fighting".to_string(),
                        "fighting-style-protection".to_string()
                    ]
                ]
            }
        }
    }
}

pub enum SheetLevelFeatureType {
    /// https://www.dnd5eapi.co/api/features/primal-champion
    PrimalChampion,
}

pub enum CustomLevelFeatureType {
    Choosable(ChoosableCustomLevelFeature),
    Sheet(SheetLevelFeatureType),
    Passive,
    Ignored
}

impl CustomLevelFeatureType {
    pub fn identify(index: String) -> Option<CustomLevelFeatureType> {
        use ChoosableCustomLevelFeature::*;
        use CustomLevelFeatureType::*;
        use SheetLevelFeatureType::*;
        match index.as_str() {
            // Ignore all subclass choices since we have only one subclass per class
            "bard-college" | "divine-domain" | "monastic-tradition" | "sacred-oath" | "ranger-archetype" | "sorcerous-origin" | "druid-circle" | "primal-path" | "martial-archetype" | "otherworldly-patron" => Some(Ignored),
            "pact-boon" => Some(Choosable(WarlockPact)),
            "additional-fighting-style" => Some(Choosable(AdditionalFighterFightingStyle)),
            "beast-spells" => Some(Ignored),
            "bonus-proficiencies" => Some(Choosable(BonusBardProficiency)),
            "bonus-proficiency" => Some(Passive),
            "additional-magical-secrets" | "bonus-cantrip" => Some(Ignored),
            "channel-divinity-1-rest" | "channel-divinity-2-rest" | "channel-divinity-3-rest" => Some(Ignored),
            "magical-secrets-1" | "magical-secrets-2" | "magical-secrets-3" => Some(Choosable(ChooseTwoSpellForAnyClass)),
            "mystic-arcanum-6th-level" | "mystic-arcanum-7th-level" | "mystic-arcanum-8th-level" | "mystic-arcanum-9th-level" => Some(Choosable(ChooseOne6thLevelSpellFromWarlockList)),
            "paladin-fighting-style" => Some(Choosable(PaladinFightingStyle)),
            "primal-champion" => Some(Sheet(PrimalChampion)),
            // TODO: Implement https://www.dnd5eapi.co/api/features/diamond-soul
            "diamond-soul" => Some(Passive),
            "arcane-recovery" | "archdruid" | "aura-improvements" | "aura-of-courage" | "aura-of-devotion" | "aura-of-protection"
            | "blessed-healer" | "blindsense" | "brutal-critical-1-dice" | "brutal-critical-2-dice" | "brutal-critical-3-dice"
            | "danger-sense" | "dark-ones-blessing" | "dark-ones-own-luck" | "defensive-tactics" | "defensive-tactics-steel-will"
            | "defensive-tactics-escape-the-horde" | "defensive-tactics-multiattack-defense" | "destroy-undead-cr-1-or-below"
            | "destroy-undead-cr-2-or-below" | "destroy-undead-cr-3-or-below" | "destroy-undead-cr-4-or-below" | "destroy-undead-cr-1-2-or-below"
            | "disciple-of-life" | "divine-health" | "dragon-ancestor-black---acid-damage" | "dragon-ancestor-blue---lightning-damage"
            | "dragon-ancestor-brass---fire-damage" | "dragon-ancestor-bronze---lightning-damage" | "dragon-ancestor-copper---acid-damage"
            | "dragon-ancestor-gold---fire-damage" | "dragon-ancestor-green---poison-damage" | "dragon-ancestor-red---fire-damage"
            | "dragon-ancestor-silver---cold-damage" | "dragon-ancestor-white---cold-damage" | "druid-lands-stride" | "druid-timeless-body"
            | "druidic" | "elusive" | "empowered-evocation" | "elemental-affinity" | "fast-movement" | "favored-enemy-1-type" | "favored-enemy-2-types"
            | "favored-enemy-3-enemies" | "feral-instinct" | "feral-senses" | "fighter-fighting-style" | "fighter-fighting-style-archery"
            | "fighter-fighting-style-defense" | "fighter-fighting-style-dueling" | "fighter-fighting-style-great-weapon-fighting"
            | "fighter-fighting-style-two-weapon-fighting" | "fighting-style-defense" | "fighting-style-dueling"
            | "fighting-style-great-weapon-fighting" | "foe-slayer" | "hurl-through-hell" | "improved-critical" | "improved-divine-smite"
            | "indomitable-1-use" | "indomitable-2-uses" | "indomitable-3-uses" | "indomitable-might" | "ki-empowered-strikes" | "jack-of-all-trades"
            | "martial-arts" | "monk-evasion" | "monk-timeless-body" | "natural-explorer-1-terrain-type" | "natural-explorer-2-terrain-types"
            | "natural-explorer-3-terrain-types" | "purity-of-body" | "purity-of-spirit" | "natures-sanctuary" | "natures-ward"
            | "sculpt-spells" | "ranger-lands-stride" | "relentless-rage" | "reliable-talent" | "remarkable-athlete" | "rogue-evasion" | "superior-critical"
            | "superior-inspiration" | "supreme-healing" | "supreme-sneak" | "survivor" | "thiefs-reflexes" | "thieves-cant"
            | "tongue-of-the-sun-and-moon" | "tranquility" | "unarmored-movement-1" | "unarmored-movement-2" | "use-magic-device"
            | "superior-hunters-defense" | "superior-hunters-defense-evasion" | "wild-shape-cr-1-2-or-below-no-flying-speed"
            | "wild-shape-cr-1-4-or-below-no-flying-or-swim-speed" | "wild-shape-cr-1-or-below" | "ki" | "monk-unarmored-defense"
            | "perfect-self" | "slippery-mind" | "mindless-rage" | "barbarian-unarmored-defense"
            | "divine-intervention-improvement" | "persistent-rage" | "evocation-savant" | "potent-cantrip" | "second-story-work" => Some(Passive),
            x if x.starts_with("bard-expertise-") || x.starts_with("rogue-expertise-") => Some(Choosable(MultiplyTwoSkillProficiency)),
            x if x.starts_with("spellcasting-") => Some(Ignored),
            // Ignore all eldritch invocations since they are unlocked using invocation known table
            x if x.starts_with("eldritch-invocation-") => Some(Ignored),
            x if x.contains("ability-score-improvement") => Some(Choosable(AbilityScoreImprovement)),
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

    pub async fn set_level(&mut self, new_level: u8) -> Result<Vec<ChoosableCustomLevelFeature>, ApiError> {
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
            CustomLevelFeatureType::identify(feature.index.clone())
        }).for_each(|feature| {
            match feature {
                CustomLevelFeatureType::Passive => {}
                CustomLevelFeatureType::Choosable(feature) => {
                    pending_features.push(feature);
                }
                CustomLevelFeatureType::Sheet(feature) => {
                    match feature {
                        SheetLevelFeatureType::PrimalChampion => {
                            self.1.abilities_addon.strength.score += 4;
                            self.1.abilities_addon.dexterity.score += 4;
                        }
                    }
                }
                Ignored => {}
            }
        });

        self.1.level = new_level;

        Ok(pending_features)
    }

    pub async fn get_levels_features(&self, from_level: Option<u8>, passive: bool) -> Result<Vec<String>, ApiError> {
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
            match CustomLevelFeatureType::identify(feature.index.clone()) {
                None => {
                    true
                }
                Some(custom_type) => {
                    match custom_type {
                        CustomLevelFeatureType::Passive => passive,
                        _ => false
                    }
                }
            }
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
