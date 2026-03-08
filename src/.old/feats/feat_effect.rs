use serde::{Deserialize, Serialize};

use crate::{
    basic::{ActivationTime, Condition, DamageResistImmune, Skill, Stat}, 
    character::char_attributes::{CreatureSize, Sense, Speed}, 
    proficiencies::Proficiency
};

use super::Feat;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatEffect {
    IncreaseStat{ options: Vec<Stat>, by: u8, max: u8 },
    MaxHealthIncreaseFlat(u8),
    MaxHealthIncreaseLevel{ scalar: u8 },
    GrantProficiency(Proficiency),
    SkillAdv{ s: Skill, context: Option<String> },
    SkillDadv{ s: Skill, context: Option<String> },
    SaveAdv{ s: Stat, context: Option<String>},
    SaveDadv{ s: Stat, context: Option<String>},
    SaveAdvAgainst{ context: String },
    SaveDadvAgainst{ context: String },
    Sense(Sense),
    Speed(Speed, u32),
    SetSize(CreatureSize),
    InitiativeBonus(u8),
    Resistance(DamageResistImmune),
    ResistanceAgainst{ context: String },
    DamageImmunity(DamageResistImmune),
    ConditionImmunity(Condition),
    // TODO grant an action that has its own activated effect on the character sheet.
    // This will require a developed system of actions and their activations/effects.
    GrantAction{ time: ActivationTime, action_desc: String },
    Choice(Vec<String>),
    GrantFeat(Option<Feat>),
    // TODO represent more things
}

/*
 * TODO: non-exhaustive list of PHB feats that have elements that could/should be represented
 * 
 * Dual Wielder: grants a +1 AC bonus only while wielding separate melee weapons in each hand.
 * 
 * Inspiring Leader: 10m activation to grant temp HP that scales on level + CHA mod.
 * 
 * Lucky: grants limited 3/long rest resource that can be spent
 * 
 * Magic Initiate; enough said.
 * 
 * Martial Adept: grants class feature options.
 * 
 * Medium Armor Master: changes the basic values of medium armor (dex cap and Stealth dadv).
 * 
 * Mounted Combatant: grants advantage on attacks in specific cases only while mounted
 * 
 * Observant: bonus to passive skill
 * 
 * Ritual Caster: enough said.
 * 
 * Spell Sniper: doubles some spells' ranges. Same stuff as Magic Initiate and Ritual Caster.
 */