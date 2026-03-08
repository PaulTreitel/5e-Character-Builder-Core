
use serde::{Serialize, Deserialize};

use crate::{
    basic::{Condition, DamageResistImmune, Stat}, 
    character::char_attributes::{AbilityScores, Alignment, CreatureSize, CreatureType, Sense, Speeds}, 
    proficiencies::{LanguageProf, SkillProf}
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonsterBase {
    name: String,
    size: CreatureSize,
    creature_type: CreatureType,
    alignment: Option<Alignment>,
    armor_class: u8,
    health: MonsterHealthStats,
    speeds: Speeds,
    scores: AbilityScores,
    senses: Vec<Sense>,
    languages: Vec<LanguageProf>,
    challenge_rating: f32,
    prof_bonus: u8,
    saving_throw_profs: Vec<Stat>,
    skill_profs: Vec<SkillProf>,
    dmg_resistances: Vec<DamageResistImmune>,
    dmg_vulnerabilities: Vec<DamageResistImmune>,
    dmg_immunities: Vec<DamageResistImmune>,
    condition_immunities: Vec<Condition>,
    actions: Vec<String>,
    bonus_actions: Vec<String>,
    reactions: Vec<String>,
    legendary_actions: Vec<String>,
    lair_actions: Vec<String>,
    mythic_actions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct MonsterHealthStats {
    hit_die_size: u8,
    hit_die_count: u8,
    hit_dice_mod: i32,
    avg_hit_points: u32,
}

impl Eq for MonsterBase {
    
}