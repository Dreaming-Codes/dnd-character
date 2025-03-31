use super::shared::schema;
use crate::GRAPHQL_API_URL;
use crate::api::classes::CustomLevelFeatureType::Ignored;
use crate::api::shared::ApiError;
use crate::classes::{Class, Classes, UsableSlots};
use cynic::http::ReqwestExt;
use cynic::{QueryBuilder, impl_scalar};
use futures::StreamExt;
use lazy_static::lazy_static;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;

#[derive(cynic::QueryVariables, Debug)]
struct SpellcastingAbilityQueryVariables {
    pub index: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    graphql_type = "Query",
    variables = "SpellcastingAbilityQueryVariables"
)]
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
            cantrip_slots: self.cantrips_known.unwrap_or(0) as u8,
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
    pub level: Option<LevelFilter>,
}

#[derive(serde::Serialize, Debug)]
pub struct LevelFilter {
    pub gt: Option<u8>,
    pub gte: Option<u8>,
    pub lte: Option<u8>,
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
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/hunters-prey
    HuntersPrey,
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/defensive-tactics
    DefensiveTactics,
    /// https://www.dnd5eapi.co/api/features/pact-boon
    WarlockPact,
    /// https://www.dnd5eapi.co/api/features/additional-fighting-style
    AdditionalFighterFightingStyle,
    /// https://www.dnd5eapi.co/api/features/fighter-fighting-style
    FighterFightingStyle,
    /// https://www.dnd5eapi.co/api/features/ranger-fighting-style
    RangerFightingStyle,
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
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/multiattack
    Multiattack,
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/superior-hunters-defense
    SuperiorHuntersDefense,
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/favored-enemy-1-type
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/favored-enemy-2-types
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/favored-enemy-3-enemies
    RangerFavoredEnemyType,
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/natural-explorer-1-terrain-type
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/natural-explorer-2-terrain-types
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/natural-explorer-3-terrain-types
    RangerTerrainType,
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/metamagic-1
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/metamagic-2
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/metamagic-3
    Metamagic,
    /// https://dnd5eapi.rpgmaster.ai/api/2014/features/dragon-ancestor
    DragonAncestor,
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

    RangerTerrainTypeArctic,
    RangerTerrainTypeCoast,
    RangerTerrainTypeDesert,
    RangerTerrainTypeForest,
    RangerTerrainTypeGrassland,
    RangerTerrainTypeMountain,
    RangerTerrainTypeSwamp,

    RangerFavoredEnemyTypeAberrations,
    RangerFavoredEnemyTypeBeasts,
    RangerFavoredEnemyTypeCelestials,
    RangerFavoredEnemyTypeConstructs,
    RangerFavoredEnemyTypeDragons,
    RangerFavoredEnemyTypeElementals,
    RangerFavoredEnemyTypeFey,
    RangerFavoredEnemyTypeFiends,
    RangerFavoredEnemyTypeGiants,
    RangerFavoredEnemyTypeMonstrosities,
    RangerFavoredEnemyTypeOozes,
    RangerFavoredEnemyTypePlants,
    RangerFavoredEnemyTypeUndead,
    RangerFavoredEnemyTypeHumanoids,

    FightingStyleDefense,
    FightingStyleDueling,
    FightingStyleGreatWeaponFighting,
    FightingStyleProtection,

    HuntersPreyGiantKiller,
    HuntersPreyHordeBreaker,
    HuntersPreyColossusSlayer,

    DefensiveTacticsSteelWill,
    DefensiveTacticsEscapeTheHorde,
    DefensiveTacticsMultiattackDefense,

    MultiattackVolley,
    MultiattackWhirlwindAttack,

    SuperiorHuntersDefenseEvasion,
    SuperiorHuntersDefenseStandAgainstTheTide,
    SuperiorHuntersDefenseUncannyDodge,

    MetamagicCarefulSpell,
    MetamagicDistantSpell,
    MetamagicEmpoweredSpell,
    MetamagicExtendedSpell,
    MetamagicHeightenedSpell,
    MetamagicQuickenedSpell,
    MetamagicSubtleSpell,
    MetamagicTwinnedSpell,

    #[serde(rename = "dragon-ancestor-black---acid-damage")]
    DragonAncestorBlackAcidDamage,
    #[serde(rename = "dragon-ancestor-blue---lightning-damage")]
    DragonAncestorBlueLightningDamage,
    #[serde(rename = "dragon-ancestor-brass---fire-damage")]
    DragonAncestorBrassFireDamage,
    #[serde(rename = "dragon-ancestor-bronze---lightning-damage")]
    DragonAncestorBronzeLightningDamage,
    #[serde(rename = "dragon-ancestor-copper---acid-damage")]
    DragonAncestorCopperAcidDamage,
    #[serde(rename = "dragon-ancestor-gold---fire-damage")]
    DragonAncestorGoldFireDamage,
    #[serde(rename = "dragon-ancestor-green---poison-damage")]
    DragonAncestorGreenPoisonDamage,
    #[serde(rename = "dragon-ancestor-red---fire-damage")]
    DragonAncestorRedFireDamage,
    #[serde(rename = "dragon-ancestor-silver---cold-damage")]
    DragonAncestorSilverColdDamage,
    #[serde(rename = "dragon-ancestor-white---cold-damage")]
    DragonAncestorWhiteColdDamage,
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
            value: ChoosableCustomLevelFeatureOption,
        }

        let json = json!({
            "value": index
        });

        serde_json::from_value::<Helper>(json)
            .map(|helper| helper.value)
            .ok()
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
                let ability_names = vec![
                    StrengthPlusOne,
                    DexterityPlusOne,
                    ConstitutionPlusOne,
                    IntelligencePlusOne,
                    WisdomPlusOne,
                    CharismaPlusOne,
                ];

                vec![ability_names.clone(), ability_names]
            }
            ChoosableCustomLevelFeature::WarlockPact => {
                vec![vec![PactOfTheChain, PactOfTheBlade, PactOfTheTome]]
            }
            ChoosableCustomLevelFeature::AdditionalFighterFightingStyle
            | ChoosableCustomLevelFeature::FighterFightingStyle => {
                vec![vec![
                    FighterFightingStyleArchery,
                    FighterFightingStyleDefense,
                    FighterFightingStyleDueling,
                    FighterFightingStyleGreatWeaponFighting,
                    FighterFightingStyleProtection,
                    FighterFightingStyleTwoWeaponFighting,
                ]]
            }
            ChoosableCustomLevelFeature::RangerFightingStyle => {
                vec![vec![
                    RangerFightingStyleArchery,
                    RangerFightingStyleDefense,
                    RangerFightingStyleDueling,
                    RangerFightingStyleTwoWeaponFighting,
                ]]
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
                vec![vec![
                    FightingStyleDefense,
                    FightingStyleDueling,
                    FightingStyleGreatWeaponFighting,
                    FightingStyleProtection,
                ]]
            }
            ChoosableCustomLevelFeature::HuntersPrey => {
                vec![vec![
                    HuntersPreyGiantKiller,
                    HuntersPreyHordeBreaker,
                    HuntersPreyColossusSlayer,
                ]]
            }
            ChoosableCustomLevelFeature::DefensiveTactics => {
                vec![vec![
                    DefensiveTacticsSteelWill,
                    DefensiveTacticsEscapeTheHorde,
                    DefensiveTacticsMultiattackDefense,
                ]]
            }
            ChoosableCustomLevelFeature::Multiattack => {
                vec![vec![MultiattackWhirlwindAttack, MultiattackVolley]]
            }
            ChoosableCustomLevelFeature::SuperiorHuntersDefense => {
                vec![vec![
                    SuperiorHuntersDefenseEvasion,
                    SuperiorHuntersDefenseStandAgainstTheTide,
                    SuperiorHuntersDefenseUncannyDodge,
                ]]
            }
            ChoosableCustomLevelFeature::RangerFavoredEnemyType => {
                vec![vec![
                    RangerFavoredEnemyTypeAberrations,
                    RangerFavoredEnemyTypeBeasts,
                    RangerFavoredEnemyTypeCelestials,
                    RangerFavoredEnemyTypeConstructs,
                    RangerFavoredEnemyTypeDragons,
                    RangerFavoredEnemyTypeElementals,
                    RangerFavoredEnemyTypeFey,
                    RangerFavoredEnemyTypeFiends,
                    RangerFavoredEnemyTypeGiants,
                    RangerFavoredEnemyTypeMonstrosities,
                    RangerFavoredEnemyTypeOozes,
                    RangerFavoredEnemyTypePlants,
                    RangerFavoredEnemyTypeUndead,
                    RangerFavoredEnemyTypeHumanoids,
                ]]
            }
            ChoosableCustomLevelFeature::RangerTerrainType => {
                vec![vec![
                    RangerTerrainTypeArctic,
                    RangerTerrainTypeCoast,
                    RangerTerrainTypeDesert,
                    RangerTerrainTypeForest,
                    RangerTerrainTypeGrassland,
                    RangerTerrainTypeMountain,
                    RangerTerrainTypeSwamp,
                ]]
            }
            ChoosableCustomLevelFeature::Metamagic => {
                let all_metamagics = vec![
                    MetamagicCarefulSpell,
                    MetamagicDistantSpell,
                    MetamagicEmpoweredSpell,
                    MetamagicExtendedSpell,
                    MetamagicHeightenedSpell,
                    MetamagicQuickenedSpell,
                    MetamagicSubtleSpell,
                    MetamagicTwinnedSpell,
                ];

                vec![all_metamagics.clone(), all_metamagics]
            }
            ChoosableCustomLevelFeature::DragonAncestor => {
                vec![vec![
                    DragonAncestorBlackAcidDamage,
                    DragonAncestorBlueLightningDamage,
                    DragonAncestorBrassFireDamage,
                    DragonAncestorBronzeLightningDamage,
                    DragonAncestorCopperAcidDamage,
                    DragonAncestorGoldFireDamage,
                    DragonAncestorGreenPoisonDamage,
                    DragonAncestorRedFireDamage,
                    DragonAncestorSilverColdDamage,
                    DragonAncestorWhiteColdDamage,
                ]]
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
    Ignored,
}

impl CustomLevelFeatureType {
    pub fn identify(index: String) -> Option<CustomLevelFeatureType> {
        use ChoosableCustomLevelFeature::*;
        use CustomLevelFeatureType::*;
        use SheetLevelFeatureType::*;
        match index.as_str() {
            // Ignore all subclass choices since we have only one subclass per class
            "bard-college"
            | "divine-domain"
            | "monastic-tradition"
            | "sacred-oath"
            | "ranger-archetype"
            | "sorcerous-origin"
            | "druid-circle"
            | "primal-path"
            | "martial-archetype"
            | "roguish-archetype"
            | "otherworldly-patron" => Some(Ignored),
            "pact-boon" => Some(Choosable(WarlockPact)),
            "additional-fighting-style" => Some(Choosable(AdditionalFighterFightingStyle)),
            "fighter-fighting-style" => Some(Choosable(FighterFightingStyle)),
            "bonus-proficiency" => Some(Passive),
            "additional-magical-secrets" | "bonus-cantrip" => Some(Ignored),
            "channel-divinity-1-rest" | "channel-divinity-2-rest" | "channel-divinity-3-rest" => {
                Some(Ignored)
            }
            //"magical-secrets-1" | "magical-secrets-2" | "magical-secrets-3" => Some(Choosable(ChooseTwoSpellForAnyClass)), TODO: Implement this
            "magical-secrets-1" | "magical-secrets-2" | "magical-secrets-3" => Some(Ignored),
            "mystic-arcanum-6th-level"
            | "mystic-arcanum-7th-level"
            | "mystic-arcanum-8th-level"
            | "mystic-arcanum-9th-level" => Some(Choosable(ChooseOne6thLevelSpellFromWarlockList)),
            "paladin-fighting-style" => Some(Choosable(PaladinFightingStyle)),
            "primal-champion" => Some(Sheet(PrimalChampion)),
            // TODO: Implement https://www.dnd5eapi.co/api/features/diamond-soul
            "diamond-soul" => Some(Passive),
            "arcane-recovery"
            | "arcane-tradition"
            | "archdruid"
            | "aura-improvements"
            | "aura-of-courage"
            | "aura-of-devotion"
            | "aura-of-protection"
            | "blessed-healer"
            | "blindsense"
            | "brutal-critical-1-dice"
            | "brutal-critical-2-dice"
            | "brutal-critical-3-dice"
            | "danger-sense"
            | "dark-ones-blessing"
            | "dark-ones-own-luck"
            | "destroy-undead-cr-1-or-below"
            | "destroy-undead-cr-2-or-below"
            | "destroy-undead-cr-3-or-below"
            | "destroy-undead-cr-4-or-below"
            | "destroy-undead-cr-1-2-or-below"
            | "disciple-of-life"
            | "divine-health"
            | "draconic-resilience"
            | "font-of-magic"
            | "druid-lands-stride"
            | "druid-timeless-body"
            | "druidic"
            | "elusive"
            | "empowered-evocation"
            | "fast-movement"
            | "feral-instinct"
            | "feral-senses"
            | "foe-slayer"
            | "hurl-through-hell"
            | "improved-critical"
            | "improved-divine-smite"
            | "indomitable-1-use"
            | "indomitable-2-uses"
            | "indomitable-3-uses"
            | "indomitable-might"
            | "ki-empowered-strikes"
            | "jack-of-all-trades"
            | "martial-arts"
            | "monk-evasion"
            | "monk-timeless-body"
            | "purity-of-body"
            | "purity-of-spirit"
            | "natures-sanctuary"
            | "natures-ward"
            | "sculpt-spells"
            | "ranger-lands-stride"
            | "relentless-rage"
            | "reliable-talent"
            | "remarkable-athlete"
            | "rogue-evasion"
            | "superior-critical"
            | "superior-inspiration"
            | "supreme-healing"
            | "supreme-sneak"
            | "survivor"
            | "thiefs-reflexes"
            | "thieves-cant"
            | "tongue-of-the-sun-and-moon"
            | "tranquility"
            | "unarmored-movement-1"
            | "unarmored-movement-2"
            | "use-magic-device"
            | "ki"
            | "monk-unarmored-defense"
            | "perfect-self"
            | "slippery-mind"
            | "mindless-rage"
            | "barbarian-unarmored-defense"
            | "divine-intervention-improvement"
            | "persistent-rage"
            | "evocation-savant"
            | "overchannel"
            | "potent-cantrip"
            | "font-of-inspiration"
            | "second-story-work"
            | "primeval-awareness"
            | "beast-spells" => Some(Passive),
            // ignored until implementation?
            "oath-spells" => Some(Ignored),
            "natural-recovery" => Some(Ignored),
            x if x.starts_with("metamagic-") => {
                if x.len() == 11 {
                    Some(Choosable(Metamagic))
                } else {
                    Some(Ignored)
                }
            }
            "hunters-prey" => Some(Choosable(HuntersPrey)),
            x if x.starts_with("hunters-prey-") => Some(Ignored),
            "superior-hunters-defense" => Some(Choosable(SuperiorHuntersDefense)),
            x if x.starts_with("superior-hunters-defenese-") => Some(Ignored),
            //x if x.starts_with("bard-expertise-")|| x.starts_with("rogue-expertise-") => Some(Choosable(MultiplyTwoSkillProficiency)),
            x if x.starts_with("bard-expertise-") || x.starts_with("rogue-expertise-") => {
                Some(Ignored)
            } // TODO: Implement this
            x if x.starts_with("spellcasting-") => Some(Ignored),
            // Ignore all eldritch invocations since they are unlocked using invocation known table
            x if x.starts_with("eldritch-invocation") => Some(Ignored),
            // Ignore all circle-spells until implementation
            x if x.starts_with("circle-spells-") => Some(Ignored),
            // Ignore all circle of the land until implementation
            x if x.starts_with("circle-of-the-land") => Some(Ignored),
            // Ignore all domain spells until implementation
            x if x.starts_with("domain-spells-") => Some(Ignored),
            // sorcery points not yet implemented
            x if x.starts_with("flexible-casting-") => Some(Ignored),
            "dragon-ancestor" => Some(Choosable(DragonAncestor)),
            x if x.starts_with("dragon-ancestor-") => Some(Ignored),
            "defensive-tactics" => Some(Choosable(DefensiveTactics)),
            x if x.starts_with("defensive-tactics-") => Some(Ignored),
            "multiattack" => Some(Choosable(Multiattack)),
            x if x.starts_with("multiattack-") => Some(Ignored),
            "ranger-fighting-style" => Some(Choosable(RangerFightingStyle)),
            x if x.starts_with("favored-enemy-") => Some(Choosable(RangerFavoredEnemyType)),
            x if x.starts_with("natural-explorer-") => Some(Choosable(RangerTerrainType)),
            // Ignore pacts from patc-boon
            x if x.starts_with("pact-of-the-") => Some(Ignored),
            x if x.contains("ability-score-improvement") => {
                Some(Choosable(AbilityScoreImprovement))
            }
            x if x.starts_with("fighting-style-") => Some(Ignored),
            x if x.starts_with("fighter-fighting-style-") => Some(Ignored),
            x if x.starts_with("ranger-fighting-style-") => Some(Ignored),
            _ => None,
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
                KnowledgePrepared {
                    pending_preparation,
                    spells_prepared_index,
                    ..
                }
                | AlreadyKnowPrepared {
                    pending_preparation,
                    spells_prepared_index,
                    ..
                } => {
                    *pending_preparation = true;
                    spells_prepared_index.clear();
                }
                KnowledgeAlreadyPrepared { usable_slots, .. } => {
                    if let Ok(Some(spellcasting_slots)) =
                        get_spellcasting_slots(index.as_str(), self.1.level).await
                    {
                        *usable_slots = spellcasting_slots.into();
                    }
                }
            }
        }
    }

    pub async fn get_spellcasting_ability_index(&self) -> Result<String, ApiError> {
        let op = SpellcastingAbilityQuery::build(SpellcastingAbilityQueryVariables {
            index: Some(self.index().to_string()),
        });

        let ability_index = Client::new()
            .post(GRAPHQL_API_URL.as_str())
            .run_graphql(op)
            .await?
            .data
            .ok_or(ApiError::Schema)?
            .class
            .ok_or(ApiError::Schema)?
            .spellcasting
            .ok_or(ApiError::Schema)?
            .spellcasting_ability
            .index;

        Ok(ability_index)
    }

    pub async fn get_spellcasting_slots(&self) -> Result<Option<LevelSpellcasting>, ApiError> {
        get_spellcasting_slots(self.index(), self.1.level).await
    }

    pub async fn set_level(
        &mut self,
        new_level: u8,
    ) -> Result<Vec<ChoosableCustomLevelFeature>, ApiError> {
        let op = LevelFeaturesQuery::build(LevelFeaturesQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
            level: Some(LevelFilter {
                gt: Some(self.1.level),
                lte: Some(new_level),
                gte: None,
            }),
        });

        let features = Client::new()
            .post(GRAPHQL_API_URL.as_str())
            .run_graphql(op)
            .await?
            .data
            .ok_or(ApiError::Schema)?
            .features
            .ok_or(ApiError::Schema)?;

        let mut pending_features = vec![];

        features
            .iter()
            .filter_map(|feature| CustomLevelFeatureType::identify(feature.index.clone()))
            .for_each(|feature| match feature {
                CustomLevelFeatureType::Passive => {}
                CustomLevelFeatureType::Choosable(feature) => {
                    pending_features.push(feature);
                }
                CustomLevelFeatureType::Sheet(feature) => match feature {
                    SheetLevelFeatureType::PrimalChampion => {
                        self.1.abilities_modifiers.strength.score += 4;
                        self.1.abilities_modifiers.constitution.score += 4;
                    }
                },
                Ignored => {}
            });

        self.1.level = new_level;

        Ok(pending_features)
    }

    pub async fn get_levels_features(
        &self,
        from_level: Option<u8>,
        passive: bool,
    ) -> Result<Vec<String>, ApiError> {
        let op = LevelFeaturesQuery::build(LevelFeaturesQueryVariables {
            class: Some(StringFilter(self.index().to_string())),
            level: Some(LevelFilter {
                gte: Some(from_level.unwrap_or(0)),
                lte: Some(self.1.level),
                gt: None,
            }),
        });

        let features = Client::new()
            .post(GRAPHQL_API_URL.as_str())
            .run_graphql(op)
            .await?
            .data
            .ok_or(ApiError::Schema)?
            .features
            .ok_or(ApiError::Schema)?;

        // First convert features to String objects and filter out non-matching features
        let features: Vec<String> = features
            .into_iter()
            .filter_map(
                |feature| match CustomLevelFeatureType::identify(feature.index.clone()) {
                    None => Some(feature.index),
                    Some(custom_type) => match custom_type {
                        CustomLevelFeatureType::Passive if passive => Some(feature.index),
                        _ => None,
                    },
                },
            )
            .collect();

        // Define all regexes at once
        lazy_static! {
            static ref CR_REGEX: regex::Regex =
                regex::Regex::new(r"(.*)-cr-([0-9]+(?:-[0-9]+)?)-or-below.*").unwrap();
            static ref DICE_REGEX: regex::Regex = regex::Regex::new(r"^(.+)-d(\d+)$").unwrap();
            static ref DIE_DICE_REGEX: regex::Regex =
                regex::Regex::new(r"^(.+)-(\d+)-(die|dice)$").unwrap();
            static ref UNARMORED_MOVEMENT_REGEX: regex::Regex =
                regex::Regex::new(r"^(unarmored-movement)-(\d+)$").unwrap();
        }

        // Track the highest values for each pattern type
        let mut cr_features: HashMap<String, (f32, String)> = HashMap::new();
        let mut dice_features: HashMap<String, u32> = HashMap::new();
        let mut die_dice_features: HashMap<String, u32> = HashMap::new();
        let mut unarmored_movement_features: HashMap<String, (u32, String)> = HashMap::new();

        // First pass to collect all the pattern information
        for feature in &features {
            // Process CR pattern
            if let Some(caps) = CR_REGEX.captures(feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();
                let cr_str = caps.get(2).unwrap().as_str();

                // Parse CR value (handling fractions like "1-2" for 1/2)
                let cr_value = if cr_str.contains('-') {
                    let parts: Vec<&str> = cr_str.split('-').collect();
                    if parts.len() == 2 {
                        parts[0].parse::<f32>().unwrap_or(0.0)
                            / parts[1].parse::<f32>().unwrap_or(1.0)
                    } else {
                        0.0
                    }
                } else {
                    cr_str.parse::<f32>().unwrap_or(0.0)
                };

                // Update if this is higher CR for this prefix
                if let Some((existing_cr, _)) = cr_features.get(&prefix) {
                    if cr_value > *existing_cr {
                        cr_features.insert(prefix, (cr_value, feature.clone()));
                    }
                } else {
                    cr_features.insert(prefix, (cr_value, feature.clone()));
                }
                continue;
            }

            // Process dice-N pattern
            if let Some(caps) = DICE_REGEX.captures(feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();
                let dice_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap_or(0);

                let current_max = dice_features.entry(prefix).or_insert(0);
                if dice_value > *current_max {
                    *current_max = dice_value;
                }
                continue;
            }

            // Process N-die/dice pattern
            if let Some(caps) = DIE_DICE_REGEX.captures(feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();
                let dice_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap_or(0);

                let current_max = die_dice_features.entry(prefix).or_insert(0);
                if dice_value > *current_max {
                    *current_max = dice_value;
                }
            }

            // Process unarmored-movement-N pattern
            if let Some(caps) = UNARMORED_MOVEMENT_REGEX.captures(feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();
                let movement_value = caps.get(2).unwrap().as_str().parse::<u32>().unwrap_or(0);

                // Update if this is a higher value for unarmored movement
                if let Some((existing_value, _)) = unarmored_movement_features.get(&prefix) {
                    if movement_value > *existing_value {
                        unarmored_movement_features
                            .insert(prefix, (movement_value, feature.clone()));
                    }
                } else {
                    unarmored_movement_features.insert(prefix, (movement_value, feature.clone()));
                }
            }
        }

        // Second pass: Filter to keep only the highest value patterns
        let mut filtered_features = Vec::new();
        let mut has_improved_divine_smite = false;

        // First check if improved-divine-smite exists
        for feature in &features {
            if feature == "improved-divine-smite" {
                has_improved_divine_smite = true;
                break;
            }
        }

        for feature in features {
            // Skip divine-smite if improved-divine-smite is present
            if feature == "divine-smite" && has_improved_divine_smite {
                continue;
            }

            // Handle CR pattern
            if let Some(caps) = CR_REGEX.captures(&feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();

                if let Some((_, highest_feature)) = cr_features.get(&prefix) {
                    if &feature == highest_feature {
                        filtered_features.push(feature);
                    }
                }
                continue;
            }

            // Handle dice pattern
            if let Some(caps) = DICE_REGEX.captures(&feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();
                let dice_value = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .expect("Parsing dice value");

                if let Some(&max_dice) = dice_features.get(&prefix) {
                    if dice_value == max_dice {
                        filtered_features.push(feature);
                    }
                }
                continue;
            }

            // Handle die/dice pattern
            if let Some(caps) = DIE_DICE_REGEX.captures(&feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();
                let dice_value = caps
                    .get(2)
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .expect("Parsing die/dice value");

                if let Some(&max_dice) = die_dice_features.get(&prefix) {
                    if dice_value == max_dice {
                        filtered_features.push(feature);
                    }
                }
                continue;
            }

            // Handle unarmored-movement-N pattern
            if let Some(caps) = UNARMORED_MOVEMENT_REGEX.captures(&feature) {
                let prefix = caps.get(1).unwrap().as_str().to_string();

                if let Some((_, highest_feature)) = unarmored_movement_features.get(&prefix) {
                    if &feature == highest_feature {
                        filtered_features.push(feature);
                    }
                }
                continue;
            }

            // Regular feature, keep it
            filtered_features.push(feature);
        }

        let mut features = filtered_features;

        // Add the selected multiattack feature if it exists and we're not requesting passive features
        if !passive {
            if let Some(multiattack) = &self.1.multiattack {
                features.push(multiattack.clone());
            }
            if let Some(hunters_prey) = &self.1.hunters_prey {
                features.push(hunters_prey.clone());
            }
            if let Some(metamagic) = &self.1.sorcerer_metamagic {
                features.append(&mut metamagic.clone());
            }
        }

        Ok(features)
    }

    pub fn apply_option(&mut self, option: ChoosableCustomLevelFeatureOption) {
        use ChoosableCustomLevelFeatureOption::*;

        match option {
            StrengthPlusOne | DexterityPlusOne | ConstitutionPlusOne | IntelligencePlusOne
            | WisdomPlusOne | CharismaPlusOne => self.increase_score(option),
            PactOfTheChain | PactOfTheBlade | PactOfTheTome => {
                println!("Pact of the Chain, Blade or Tome not yet implemented");
            }
            HuntersPreyGiantKiller | HuntersPreyHordeBreaker | HuntersPreyColossusSlayer => {
                self.1
                    .hunters_prey
                    .replace(option.as_index_str().to_string());
            }
            DefensiveTacticsSteelWill
            | DefensiveTacticsEscapeTheHorde
            | DefensiveTacticsMultiattackDefense => {
                self.1
                    .defensive_tactics
                    .replace(option.as_index_str().to_string());
            }
            FighterFightingStyleArchery
            | FighterFightingStyleDefense
            | FighterFightingStyleDueling
            | FighterFightingStyleGreatWeaponFighting
            | FighterFightingStyleProtection
            | FighterFightingStyleTwoWeaponFighting
            | RangerFightingStyleArchery
            | RangerFightingStyleDefense
            | RangerFightingStyleDueling
            | RangerFightingStyleTwoWeaponFighting
            | FightingStyleDefense
            | FightingStyleDueling
            | FightingStyleGreatWeaponFighting
            | FightingStyleProtection => {
                if self.1.fighting_style.is_none() {
                    self.1
                        .fighting_style
                        .replace(option.as_index_str().to_string());
                } else {
                    self.1
                        .additional_fighting_style
                        .replace(option.as_index_str().to_string());
                }
            }
            MultiattackVolley | MultiattackWhirlwindAttack => {
                self.1
                    .multiattack
                    .replace(option.as_index_str().to_string());
            }
            SuperiorHuntersDefenseEvasion
            | SuperiorHuntersDefenseStandAgainstTheTide
            | SuperiorHuntersDefenseUncannyDodge => {
                self.1
                    .superior_hunters_defense
                    .replace(option.as_index_str().to_string());
            }
            RangerTerrainTypeArctic
            | RangerTerrainTypeCoast
            | RangerTerrainTypeDesert
            | RangerTerrainTypeForest
            | RangerTerrainTypeGrassland
            | RangerTerrainTypeMountain
            | RangerTerrainTypeSwamp => {
                self.1
                    .natural_explorer_terrain_type
                    .get_or_insert_with(Vec::new)
                    .push(option.as_index_str().to_string());
            }
            RangerFavoredEnemyTypeAberrations
            | RangerFavoredEnemyTypeBeasts
            | RangerFavoredEnemyTypeCelestials
            | RangerFavoredEnemyTypeConstructs
            | RangerFavoredEnemyTypeDragons
            | RangerFavoredEnemyTypeElementals
            | RangerFavoredEnemyTypeFey
            | RangerFavoredEnemyTypeFiends
            | RangerFavoredEnemyTypeGiants
            | RangerFavoredEnemyTypeMonstrosities
            | RangerFavoredEnemyTypeOozes
            | RangerFavoredEnemyTypePlants
            | RangerFavoredEnemyTypeUndead
            | RangerFavoredEnemyTypeHumanoids => {
                self.1
                    .ranger_favored_enemy_type
                    .get_or_insert_with(Vec::new)
                    .push(option.as_index_str().to_string());
            }
            MetamagicCarefulSpell
            | MetamagicDistantSpell
            | MetamagicEmpoweredSpell
            | MetamagicExtendedSpell
            | MetamagicHeightenedSpell
            | MetamagicQuickenedSpell
            | MetamagicSubtleSpell
            | MetamagicTwinnedSpell => {
                self.1
                    .sorcerer_metamagic
                    .get_or_insert_with(Vec::new)
                    .push(option.as_index_str().to_string());
            }
            DragonAncestorBlackAcidDamage
            | DragonAncestorBlueLightningDamage
            | DragonAncestorBrassFireDamage
            | DragonAncestorBronzeLightningDamage
            | DragonAncestorCopperAcidDamage
            | DragonAncestorGoldFireDamage
            | DragonAncestorGreenPoisonDamage
            | DragonAncestorRedFireDamage
            | DragonAncestorSilverColdDamage
            | DragonAncestorWhiteColdDamage => {
                self.1
                    .sorcerer_dragon_ancestor
                    .replace(option.as_index_str().to_string());
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
}

pub async fn get_spellcasting_slots(
    index: &str,
    level: u8,
) -> Result<Option<LevelSpellcasting>, ApiError> {
    let op = SpellcastingQuery::build(SpellcastingQueryVariables {
        index: Some(format!("{}-{}", index, level)),
    });

    let spellcasting_slots = Client::new()
        .post(GRAPHQL_API_URL.as_str())
        .run_graphql(op)
        .await?
        .data
        .ok_or(ApiError::Schema)?
        .level
        .ok_or(ApiError::Schema)?
        .spellcasting;

    Ok(spellcasting_slots)
}
