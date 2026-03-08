use serde::{Deserialize, Serialize};

use crate::basic::Stat;


#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Speed {
    Walk,
    Swim,
    Climb,
    Fly,
    Burrow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Speeds {
    walk: u32,
    swim: u32,
    climb: u32,
    fly: u32,
    burrow: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
    LawfulGood,
    LawfulNeutral,
    LawfulEvil,
    NeutralGood,
    TrueNeutral,
    NeutralEvil,
    ChaoticGood,
    ChaoticNeutral,
    ChaoticEvil,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Sense {
    Blindsight(u32),
    Darkvision(u32),
    Tremorsense(u32),
    Truesight(u32),
    Custom{ name: String, distance: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CreatureType {
    Abberation,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    #[default]
    Humanoid,
    Monstrosity,
    Ooze,
    Undead,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum CreatureSize {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Huge,
    Gargantuan,
}

impl AbilityScores {
    pub fn get_stat(&self, s: Stat) -> u8 {
        match s {
            Stat::Strength => self.strength,
            Stat::Dexterity => self.dexterity,
            Stat::Constitution => self.constitution,
            Stat::Intelligence => self.intelligence,
            Stat::Wisdom => self.wisdom,
            Stat::Charisma => self.charisma,
        }
    }

    pub fn get_mod(&self, s: Stat) -> i8 {
        (self.get_stat(s) as i8 / 2) - 5
    }

    pub fn change_stat(&mut self, s: Stat, amt: i8, max: u8) -> () {
        match s {
            Stat::Strength => {
                self.strength = max.min((self.strength as i8 + amt) as u8).max(1)
            },
            Stat::Dexterity => {
                self.dexterity = max.min((self.dexterity as i8 + amt) as u8).max(1)
            },
            Stat::Constitution => {
                self.constitution = max.min((self.constitution as i8 + amt) as u8).max(1)
            },
            Stat::Intelligence => {
                self.intelligence = max.min((self.intelligence as i8 + amt) as u8).max(1)
            },
            Stat::Wisdom => {
                self.wisdom = max.min((self.wisdom as i8 + amt) as u8).max(1)
            },
            Stat::Charisma => {
                self.charisma = max.min((self.charisma as i8 + amt) as u8).max(1)
            },
        }
    }

    pub fn set_stat(&mut self, s: Stat, val: u8) -> () {
        match s {
            Stat::Strength => self.strength = val,
            Stat::Dexterity => self.dexterity = val,
            Stat::Constitution => self.constitution = val,
            Stat::Intelligence => self.intelligence = val,
            Stat::Wisdom => self.wisdom = val,
            Stat::Charisma => self.charisma = val,
        }
    }
}

impl Speeds {
    pub fn get_speed(&self, s: Speed) -> u32 {
        match s {
            Speed::Walk => self.walk,
            Speed::Swim => self.swim,
            Speed::Climb => self.climb,
            Speed::Fly => self.fly,
            Speed::Burrow => self.burrow,
        }
    }

    pub fn has_speed(&self, s: Speed) -> bool {
        self.get_speed(s) > 0
    }

    pub fn set_speed(&mut self, s: Speed, amt: u32) -> () {
        match s {
            Speed::Walk => self.walk = amt,
            Speed::Swim => self.swim = amt,
            Speed::Climb => self.climb = amt,
            Speed::Fly => self.fly = amt,
            Speed::Burrow => self.burrow = amt,
        }
    }
}