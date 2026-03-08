pub mod subclass;
pub mod starting_gear;

use serde::{Deserialize, Serialize};
use starting_gear::StartingGear;
use subclass::Subclass;

use crate::{basic::{DieSize, Stat}, proficiencies::{ArmorProf, SkillProf, ToolProf, WeaponProf}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Class {
    hit_die: DieSize,
    armor_prof: Vec<ArmorProf>,
    weapon_prof: Vec<WeaponProf>,
    tool_prof: Vec<ToolProf>,
    saving_throw_prof: Vec<Stat>,
    skill_prof: Vec<SkillProf>,
    skill_choices: u8,
    multiclass_reqs: MultiClassRequirements,
    starting_gear: StartingGear,
    starting_gold: ClassStartingGold,
    subclasses: Vec<Subclass>,
    features: Vec<ClassFeature>,
    // TODO Add something to represent spellcasting.
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
struct MultiClassRequirements {
    stats: Vec<(Stat, u8)>,
    // `logical_or` represents whether the stats requirements are X or Y vs X and Y.
    logical_or: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
struct ClassStartingGold {
    die: DieSize,
    die_ct: u8,
    multiplier: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ClassFeature {
    name: String,
    description: String,
    level: u8,
    // TODO feature effects
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Classes {
    Artificer,
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
    Custom(String),
}

mod class {
    
}