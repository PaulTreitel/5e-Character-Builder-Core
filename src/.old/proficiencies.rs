use serde::{Deserialize, Serialize};

use crate::basic::{Skill, Stat};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LanguageProf {
    Choice,
    Lang{name: String},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum SkillProf {
    #[default]
    Choice,
    Skill(Skill),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ToolProf {
    InstrumentChoice,
    Instrument{name: String},
    GamingChoice,
    Game{name: String},
    ArtisanChoice,
    Artisan{name: String},
    DuisguiseKit,
    ForgeryKit,
    HerbalismKit,
    NavigatorsTools,
    PoisonersKit,
    ThievesTools,
    LandVehicle,
    WaterVehicle,
    Custom{name: String},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ArmorProf {
    Light,
    Medium,
    Heavy,
    Shields,
    Specific{name: String},
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponProf {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
    Firearm,
    Specific{name: String},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolLangProf {
    Lang(LanguageProf),
    Tool(ToolProf),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Proficiency {
    Tool(ToolProf, ProficiencyLevel),
    Lang(LanguageProf),
    Weapon(WeaponProf),
    Armor(ArmorProf),
    Save(Stat),
    Skill(SkillProf, ProficiencyLevel),
    Initiative(ProficiencyLevel),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum ProficiencyLevel {
    #[default]
    None,
    Half,
    Full,
    Expertise,
}

impl Default for ToolLangProf {
    fn default() -> Self {
        // Language is chosen arbitrarily vs tool
        ToolLangProf::Lang(LanguageProf::Choice)
    }
}