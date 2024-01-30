use super::shared::schema;
use cynic::http::ReqwestExt;
use reqwest::Client;
use crate::api::shared::ApiError;
use cynic::QueryBuilder;
use serde::{Serialize};
use crate::classes::Class;

#[derive(cynic::QueryVariables, Debug)]
struct SpellcastingAbilityQueryVariables {
    pub index: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(graphql_type = "Query", variables = "SpellcastingAbilityQueryVariables")]
struct SpellcastingAbilityQuery {
    #[arguments(index: $index)]
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
    #[arguments(index: $index)]
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
    #[arguments(level: $level, class: $class)]
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
    // Ask the user to spend 2 points in any ability score
    AbilityScoreImprovement,
    // https://www.dnd5eapi.co/api/features/pact-boon
    WarlockPact,
    // Ignore this feature, since we have only one subclass per class
    SubclassChoice,
    // https://www.dnd5eapi.co/api/features/additional-fighting-style
    AdditionalFighterFightingStyle,
    // https://www.dnd5eapi.co/api/features/bonus-proficiencies
    BonusBardProficiency,
    // https://www.dnd5eapi.co/api/features/beast-spells
    // This feature will not be implemented for now
    // TODO: Implement
    BeastSpells,
    // This is for features already handled by other parts of the code and not needed to be managed as "features"
    Ignored
}

impl CustomLevelFeature {
    pub fn identify(index: String) -> Option<CustomLevelFeature> {
        match index.as_str() {
            x if x.contains("ability-score-improvement") => Some(CustomLevelFeature::AbilityScoreImprovement),
            "bard-college" | "divine-domain" | "monastic-tradition" | "sacred-oath" | "ranger-archetype" | "sorcerous-origin" => Some(CustomLevelFeature::SubclassChoice),
            "pact-boon" => Some(CustomLevelFeature::WarlockPact),
            "additional-fighting-style" => Some(CustomLevelFeature::AdditionalFighterFightingStyle),
            "beast-spells" => Some(CustomLevelFeature::BeastSpells),
            "bonus-proficiencies" => Some(CustomLevelFeature::BonusBardProficiency),
            "additional-magical-secrets" | "bonus-cantrip" => Some(CustomLevelFeature::Ignored),
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
            level: Some(IntFilter(format!("{{ gte: {}, lte: {} }}", self.1.level, new_level)))
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
            level: Some(IntFilter(format!("{{ gte: {}, lte: {} }}", from_level.unwrap_or(0), self.1.level)))
        });

        let features = Client::new()
            .post("https://www.dnd5eapi.co/graphql")
            .run_graphql(op).await?
            .data.ok_or(ApiError::Schema)?
            .features.ok_or(ApiError::Schema)?;

        // Remove all identifiable features
        let features = features.into_iter().filter(|feature| {
            CustomLevelFeature::identify(feature.index.clone()).is_none()
        }).map(|feature| feature.index).collect();

        Ok(features)
    }
}
