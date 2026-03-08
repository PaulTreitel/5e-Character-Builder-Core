use crate::inventory::{equipment::Equipment, money::Money};
use crate::proficiencies::{SkillProf, ToolLangProf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Background {
    name: String,
    description: String,
    skill_profs: (SkillProf, SkillProf),
    lang_tool_profs: (ToolLangProf, ToolLangProf),
    gear: Vec<Equipment>,
    money: Money,
    personality: Vec<String>, 
    ideals: Vec<String>,
    bonds: Vec<String>,
    flaws: Vec<String>,
    bg_choice: Vec<String>,
}

impl Background {
    pub fn new(
        name: String,
        description: String,
        skills: (SkillProf, SkillProf),
        lang_tool_profs: (ToolLangProf, ToolLangProf),
        gear: Vec<Equipment>,
        money: Money,
        personality: Vec<String>, 
        ideals: Vec<String>,
        bonds: Vec<String>,
        flaws: Vec<String>,
        bg_choice: Vec<String>,
    ) -> Self {
        Background {
            name, 
            description, 
            skill_profs: skills, 
            lang_tool_profs,
            gear, 
            money,
            personality,
            ideals,
            bonds,
            flaws,
            bg_choice,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn desc(&self) -> &str {
        &self.description
    }

    pub fn skills(&self) -> &(SkillProf, SkillProf) {
        &self.skill_profs
    }

    pub fn tools_and_langs(&self) -> &(ToolLangProf, ToolLangProf) {
        &self.lang_tool_profs
    }

    pub fn gear(&self) -> &Vec<Equipment> {
        &self.gear
    }

    pub fn money(&self) -> &Money {
        &self.money
    }

    pub fn personality(&self) -> &Vec<String> {
        &self.personality
    }
    
    pub fn ideals(&self) -> &Vec<String> {
        &self.ideals
    }

    pub fn bonds(&self) -> &Vec<String> {
        &self.bonds
    }

    pub fn flaws(&self) -> &Vec<String> {
        &self.flaws
    }

    pub fn background_choice(&self) -> &Vec<String> {
        &self.bg_choice
    }
}