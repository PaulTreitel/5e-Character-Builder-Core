use serde::{Serialize, Deserialize};

use crate::basic::{Condition, DamageResistImmune};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Defenses {
    armor_class: u8,
    resistances: Vec<DamageResistImmune>,
    dmg_immunities: Vec<DamageResistImmune>,
    vulnerabilities: Vec<DamageResistImmune>,
    condition_immunities: Vec<Condition>,
}

impl Defenses {
    pub fn resistances(&self) -> &Vec<DamageResistImmune> {
        &self.resistances
    }

    pub fn vulnerabilities(&self) -> &Vec<DamageResistImmune> {
        &self.vulnerabilities
    }

    pub fn dmg_immunities(&self) -> &Vec<DamageResistImmune> {
        &self.dmg_immunities
    }

    pub fn condition_immunities(&self) -> &Vec<Condition> {
        &self.condition_immunities
    }

    pub fn ac(&self) -> u8 {
        self.armor_class
    }

    pub fn add_resistance(&mut self, r: DamageResistImmune) -> () {
        self.resistances.push(r);
    }

    pub fn add_vulnerability(&mut self, r: DamageResistImmune) -> () {
        self.vulnerabilities.push(r);
    }

    pub fn add_dmg_immunity(&mut self, r: DamageResistImmune) -> () {
        self.dmg_immunities.push(r);
    }

    pub fn add_condition_immunity(&mut self, r: Condition) -> () {
        self.condition_immunities.push(r);
    }

    pub fn set_ac(&mut self, ac: u8) -> () {
        self.armor_class = ac;
    }

    pub fn reset_resistances(&mut self) -> () {
        self.resistances = Vec::new();
    }

    pub fn reset_vulnerabilities(&mut self) -> () {
        self.vulnerabilities = Vec::new();
    }

    pub fn reset_dmg_immunities(&mut self) -> () {
        self.dmg_immunities = Vec::new();
    }

    pub fn reset_condition_immunities(&mut self) -> () {
        self.condition_immunities = Vec::new();
    }    
}