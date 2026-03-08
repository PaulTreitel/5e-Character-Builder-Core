use crate::basic::{ActivationTime, AreaOfEffect, AttackType, EffectDuration, EffectRange, Stat};
use serde::{self, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spell {
    name: String,
    description: String,
    level: u8,
    school: SpellSchool,
    cast_time: ActivationTime,
    range: EffectRange,
    area: Option<AreaOfEffect>,
    components: SpellComponents,
    duration: EffectDuration,
    concentration: bool,
    ritual: bool,
    save: Option<Stat>,
    attack: Option<AttackType>,
    spell_lists: Vec<String>,
    // TODO: How do we store (a) spell effects that act on the character sheet
    // and (b) spell upcasting benefits (which should also be displayed on
    // spells in upcast slots)?
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpellComponents {
    verbal: bool,
    somatic: bool,
    material: bool,
    material_desc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpellSchool {
    Abjuration,
    Conjuration,
    Divination,
    Enchantment,
    Evocation,
    Illusion,
    Necromancy,
    Transmutation
}

impl SpellComponents {
    pub fn new(v: bool, s: bool, m: bool, m_desc: Option<String>) -> Self {
        SpellComponents {
            verbal: v, 
            somatic: s, 
            material: m, 
            material_desc: m_desc
        }
    }
}