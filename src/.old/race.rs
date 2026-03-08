use serde::{Serialize, Deserialize};

use crate::{
    basic::Stat, 
    character::char_attributes::{CreatureSize, CreatureType, Speeds}, 
    proficiencies::LanguageProf
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Race {
    name: String,
    description: String,
    asi_type: RaceASI,
    // an internal racial choice, like a dragonborn's draconic heritage
    choice: Vec<String>,
    creature_size_choices: Vec<CreatureSize>,
    creature_type: CreatureType,
    speeds: Speeds,
    lamguages: Vec<LanguageProf>,
    abilities: Vec<RacialAbility>,
    subraces: Vec<SubRace>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SubRace {
    name: String,
    description: String,
    // TODO Should racial effects use the FeatEffect enum or its own specific effect?
    // abilities: Vec<FeatEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
struct RacialAbility {
    name: String,
    description: String,
    // TODO Should racial effects use the FeatEffect enum or its own specific effect?
    // effect: Option<FeatEffect>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RaceASI {
    PlusTwo(Option<Stat>),
    PlusOne(Option<Stat>),
    PlusTwoPlusOne(Option<Stat>, Option<Stat>),
    TriplePlusOne(Option<Stat>, Option<Stat>, Option<Stat>),
    PlusOneToAll,
    DoublePlusTwo(Option<Stat>, Option<Stat>),
}

impl Default for RaceASI {
    fn default() -> Self {
        RaceASI::PlusTwoPlusOne(None, None)
    }
}