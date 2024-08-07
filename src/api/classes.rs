use std::collections::HashMap;
use super::shared::schema;
use cynic::http::ReqwestExt;
use reqwest::Client;
use crate::api::shared::ApiError;
use cynic::{impl_scalar, QueryBuilder};
use futures::StreamExt;
use lazy_static::lazy_static;
use serde_json::json;
use crate::api::classes::CustomLevelFeatureType::Ignored;
use crate::classes::{Class, Classes, UsableSlots};

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

#[derive(cynic::QueryFragment, Debug, Copy, Clone)]
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

impl Into<UsableSlots> for LevelSpellcasting {
    fn into(self) -> UsableSlots {
        UsableSlots {
            level_1: self.spell_slots_level_1.unwrap_or(0) as u8,
            level_2: self.spell_slots_level_2.unwrap_or(0) as u8,
            level_3: self.spell_slots_level_3.unwrap_or(0) as u8,
            level_4: self.spell_slots_level_4.unwrap_or(0) as u8,
            level_5: self.spell_slots_level_5.unwrap_or(0) as u8,
            level_6: self.spell_slots_level_6.unwrap_or(0) as u8,
            level_7: self.spell_slots_level_7.unwrap_or(0) as u8,
            level_8: self.spell_slots_level_8.unwrap_or(0) as u8,
            level_9: self.spell_slots_level_9.unwrap_or(0) as u8,
        }
    }
}

#[derive(cynic::QueryVariables, Debug)]
pub struct LevelFeaturesQueryVariables {
    pub class: Option<StringFilter>,
    pub level: Option<LevelFilter>
}

#[derive(serde::Serialize, Debug)]
pub struct LevelFilter {
    pub gte: Option<u8>,
    pub lte: Option<u8>
}

impl_scalar!(LevelFilter, schema::IntFilter);

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "LevelFeaturesQueryVariables")]
pub struct LevelFeaturesQuery {
    #[arguments(class: $ class, level: $level )]
    pub features: Option<Vec<Feature>>,
}

#[derive(cynic::QueryFragment, Debug)]
pub struct Feature {
    pub index: String,
}

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
    /// https://www.dnd5eapi.co/api/features/fighter-fighting-style
    FighterFightingStyle,
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

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ChoosableCustomLevelFeatureOption {
    StrengthPlusOne,
    DexterityPlusOne,
    ConstitutionPlusOne,
    IntelligencePlusOne,
    WisdomPlusOne,
    CharismaPlusOne,

    PactOfTheChain,
    PactOfTheBlade,
    PactOfTheTome,

    FighterFightingStyleArchery,
    FighterFightingStyleDefense,
    FighterFightingStyleDueling,
    FighterFightingStyleGreatWeaponFighting,
    FighterFightingStyleProtection,
    FighterFightingStyleTwoWeaponFighting,

    RangerFightingStyleArchery,
    RangerFightingStyleDefense,
    RangerFightingStyleDueling,
    RangerFightingStyleTwoWeaponFighting,

    BardProficiencyStrength,
    BardProficiencyDexterity,
    BardProficiencyConstitution,
    BardProficiencyIntelligence,
    BardProficiencyWisdom,
    BardProficiencyCharisma,

    FightingStyleDefense,
    FightingStyleDueling,
    FightingStyleGreatWeaponFighting,
    FightingStyleProtection
}

impl ChoosableCustomLevelFeatureOption {
    #[cfg(feature = "serde")]
    pub fn as_index_str(&self) -> &str {
        serde_variant::to_variant_name(self).unwrap()
    }

    #[cfg(feature = "serde")]
    pub fn from_index_str(index: &str) -> Option<ChoosableCustomLevelFeatureOption> {
        #[derive(serde::Deserialize)]
        struct Helper {
            value: ChoosableCustomLevelFeatureOption
        }

        let json = json!({
            "value": index
        });

        serde_json::from_value::<Helper>(json).map(|helper| helper.value).ok()
    }
}

impl ChoosableCustomLevelFeature {
    #[cfg(feature = "serde")]
    pub fn as_index_str(&self) -> &str {
        serde_variant::to_variant_name(self).unwrap()
    }

    pub fn to_options(&self) -> Vec<Vec<ChoosableCustomLevelFeatureOption>> {
        use ChoosableCustomLevelFeatureOption::*;

        match self {
            ChoosableCustomLevelFeature::AbilityScoreImprovement => {
                let ability_names = vec![StrengthPlusOne, DexterityPlusOne, ConstitutionPlusOne, IntelligencePlusOne, WisdomPlusOne, CharismaPlusOne];

                vec![
                    ability_names.clone(),
                    ability_names,
                ]
            }
            ChoosableCustomLevelFeature::WarlockPact => {
                vec![
                    vec![PactOfTheChain, PactOfTheBlade, PactOfTheTome]
                ]
            }
            ChoosableCustomLevelFeature::AdditionalFighterFightingStyle | ChoosableCustomLevelFeature::FighterFightingStyle => {
                vec![
                    vec![
                        FighterFightingStyleArchery,
                        FighterFightingStyleDefense,
                        FighterFightingStyleDueling,
                        FighterFightingStyleGreatWeaponFighting,
                        FighterFightingStyleProtection,
                        FighterFightingStyleTwoWeaponFighting
                    ]
                ]
            }
            ChoosableCustomLevelFeature::RangerFightingStyle => {
                vec![
                    vec![
                        RangerFightingStyleArchery,
                        RangerFightingStyleDefense,
                        RangerFightingStyleDueling,
                        RangerFightingStyleTwoWeaponFighting
                    ]
                ]
            }
            ChoosableCustomLevelFeature::BonusBardProficiency => {
                let ability_names = vec![BardProficiencyStrength, BardProficiencyDexterity, BardProficiencyConstitution, BardProficiencyIntelligence, BardProficiencyWisdom, BardProficiencyCharisma];

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
                        FightingStyleDefense,
                        FightingStyleDueling,
                        FightingStyleGreatWeaponFighting,
                        FightingStyleProtection
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
            "ranger-fighting-style" => Some(Choosable(RangerFightingStyle)),
            "fighter-fighting-style" => Some(Choosable(RangerFightingStyle)),
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
            | "favored-enemy-3-enemies" | "feral-instinct" | "feral-senses" | "fighter-fighting-style-archery" | "fighter-fighting-style-protection"
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


impl Classes {
    pub(super) async fn new_day(&mut self) {
        futures::stream::iter(self.0.values_mut())
            .for_each_concurrent(None, |class| class.new_day())
            .await;
    }
}

impl Class {
    pub(super) async fn new_day(&mut self) {
        use crate::classes::ClassSpellCasting::*;

        let index = self.index().to_string();

        if let Some(spell_casting) = &mut self.1.spell_casting {
            match spell_casting {
                KnowledgePrepared {pending_preparation, spells_prepared_index, .. } | AlreadyKnowPrepared { pending_preparation, spells_prepared_index, .. } => {
                    *pending_preparation = true;
                    spells_prepared_index.clear();
                }
                KnowledgeAlreadyPrepared {usable_slots, ..} => {
                    if let Ok(Some(spellcasting_slots)) = get_spellcasting_slots(index.as_str(), self.1.level).await {
                        *usable_slots = spellcasting_slots.into();
                    }
                }
            }
        }
    }

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
        get_spellcasting_slots(self.index(), self.1.level).await
    }

    pub async fn set_level(&mut self, new_level: u8) -> Result<Vec<ChoosableCustomLevelFeature>, ApiError> {
        let op = LevelFeaturesQuery::build(LevelFeaturesQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
            level: Some(LevelFilter{gte: Some(self.1.level), lte: Some(new_level)}),
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
                            self.1.abilities_modifiers.strength.score += 4;
                            self.1.abilities_modifiers.dexterity.score += 4;
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
            level: Some(LevelFilter{gte: Some(from_level.unwrap_or(0)), lte: Some(self.1.level)}),
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

    pub fn apply_option(&mut self, option: ChoosableCustomLevelFeatureOption) {
        use ChoosableCustomLevelFeatureOption::*;

        match option {
            StrengthPlusOne | DexterityPlusOne | ConstitutionPlusOne | IntelligencePlusOne | WisdomPlusOne | CharismaPlusOne => {
                self.increase_score(option)
            }
            BardProficiencyStrength | BardProficiencyDexterity | BardProficiencyConstitution | BardProficiencyIntelligence | BardProficiencyWisdom | BardProficiencyCharisma => {
                self.set_proficiency(option)
            }
            PactOfTheChain | PactOfTheBlade | PactOfTheTome => {
                println!("Pact of the Chain, Blade or Tome not yet implemented");
            }
            FighterFightingStyleArchery | FighterFightingStyleDefense | FighterFightingStyleDueling | FighterFightingStyleGreatWeaponFighting
            | FighterFightingStyleProtection | FighterFightingStyleTwoWeaponFighting | RangerFightingStyleArchery | RangerFightingStyleDefense
            | RangerFightingStyleDueling | RangerFightingStyleTwoWeaponFighting | FightingStyleDefense | FightingStyleDueling
            | FightingStyleGreatWeaponFighting | FightingStyleProtection => {
                if self.1.fighting_style.is_none() {
                    self.1.fighting_style.replace(option.as_index_str().to_string());
                } else {
                    self.1.additional_fighting_style.replace(option.as_index_str().to_string());
                }
            }
        }
    }

    fn increase_score(&mut self, option: ChoosableCustomLevelFeatureOption) {
        match option {
            ChoosableCustomLevelFeatureOption::StrengthPlusOne => {
                self.1.abilities_modifiers.strength.score += 1;
            }
            ChoosableCustomLevelFeatureOption::DexterityPlusOne => {
                self.1.abilities_modifiers.dexterity.score += 1;
            }
            ChoosableCustomLevelFeatureOption::ConstitutionPlusOne => {
                self.1.abilities_modifiers.constitution.score += 1;
            }
            ChoosableCustomLevelFeatureOption::IntelligencePlusOne => {
                self.1.abilities_modifiers.intelligence.score += 1;
            }
            ChoosableCustomLevelFeatureOption::WisdomPlusOne => {
                self.1.abilities_modifiers.wisdom.score += 1;
            }
            ChoosableCustomLevelFeatureOption::CharismaPlusOne => {
                self.1.abilities_modifiers.charisma.score += 1;
            }
            _ => {}
        }
    }

    fn set_proficiency(&mut self, option: ChoosableCustomLevelFeatureOption) {
        match option {
            ChoosableCustomLevelFeatureOption::BardProficiencyStrength => {
                self.1.abilities_modifiers.strength.proficiency = true;
            }
            ChoosableCustomLevelFeatureOption::BardProficiencyDexterity => {
                self.1.abilities_modifiers.dexterity.proficiency = true;
            }
            ChoosableCustomLevelFeatureOption::BardProficiencyConstitution => {
                self.1.abilities_modifiers.constitution.proficiency = true;
            }
            ChoosableCustomLevelFeatureOption::BardProficiencyIntelligence => {
                self.1.abilities_modifiers.intelligence.proficiency = true;
            }
            ChoosableCustomLevelFeatureOption::BardProficiencyWisdom => {
                self.1.abilities_modifiers.wisdom.proficiency = true;
            }
            ChoosableCustomLevelFeatureOption::BardProficiencyCharisma => {
                self.1.abilities_modifiers.charisma.proficiency = true;
            }
            _ => {}
        }

    }
}

pub async fn get_spellcasting_slots(index: &str, level: u8) -> Result<Option<LevelSpellcasting>, ApiError> {
    let op = SpellcastingQuery::build(SpellcastingQueryVariables {
        index: Some(format!("{}-{}", index, level))
    });

    let spellcasting_slots = Client::new()
        .post("https://www.dnd5eapi.co/graphql")
        .run_graphql(op).await?
        .data.ok_or(ApiError::Schema)?
        .level.ok_or(ApiError::Schema)?
        .spellcasting;

    Ok(spellcasting_slots)
}
