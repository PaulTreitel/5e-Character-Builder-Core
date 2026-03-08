use serde::{Deserialize, Serialize};
    
use crate::{
    basic::Stat,
    proficiencies::{ArmorProf, LanguageProf, Proficiency, ProficiencyLevel, SkillProf, ToolProf, WeaponProf}
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CharProficiencies {
    skills: Vec<(SkillProf, ProficiencyLevel)>,
    saves: Vec<Stat>,
    armor: Vec<ArmorProf>,
    weapons: Vec<WeaponProf>,
    languages: Vec<LanguageProf>,
    tools: Vec<(ToolProf, ProficiencyLevel)>,
    initiative: ProficiencyLevel,
}

pub enum CharProfError {
    RemoveNonExistentProf,
    NoChangeDueToProfMismatch,
}

impl CharProficiencies {
    // TODO do we need a new fucntion or just construct from default?

    pub fn get_proficiency(&self, p: &Proficiency) -> ProficiencyLevel {
        match p {
            Proficiency::Tool(t, _) => {
                CharProficiencies::get_full_range_prof(&self.tools, &t)
            },
            Proficiency::Lang(l) => {
                CharProficiencies::get_binary_prof(&self.languages, &l)
            },
            Proficiency::Weapon(w) => {
                CharProficiencies::get_binary_prof(&self.weapons, &w)
            },
            Proficiency::Armor(a) => {
                CharProficiencies::get_binary_prof(&self.armor, &a)
            },
            Proficiency::Save(s) => {
                CharProficiencies::get_binary_prof(&self.saves, &s)
            },
            Proficiency::Skill(s, _) => {
                CharProficiencies::get_full_range_prof(&self.skills, &s)
            },
            Proficiency::Initiative(_) => self.initiative.clone(),
        }
    }

    fn get_binary_prof<T: PartialEq>(v: &Vec<T>, search: &T) -> ProficiencyLevel {
        if v.contains(search) {
            ProficiencyLevel::Full
        } else {
            ProficiencyLevel::None
        }
    }

    fn get_full_range_prof<T: Ord>(
        v: &Vec<(T, ProficiencyLevel)>, 
        search: &T
    ) -> ProficiencyLevel {
        let tmp = v.binary_search_by(|x| x.0.cmp(search));
        match tmp {
            Ok(idx) => v.get(idx).unwrap().1.clone(),
            Err(_) => ProficiencyLevel::None,
        }
    }

    pub fn add_proficiency(&mut self, p: Proficiency) -> () {
        match p {
            Proficiency::Tool(t, prof) => {
                CharProficiencies::add_full_range_prof(&mut self.tools, t, prof);
            },
            Proficiency::Lang(l) => {
                CharProficiencies::add_binary_prof(&mut self.languages, l);
            },
            Proficiency::Weapon(w) => {
                CharProficiencies::add_binary_prof(&mut self.weapons, w);
            },
            Proficiency::Armor(a) => {
                CharProficiencies::add_binary_prof(&mut self.armor, a);
            },
            Proficiency::Save(s) => {
                CharProficiencies::add_binary_prof(&mut self.saves, s);
            },
            Proficiency::Skill(s, prof) => {
                CharProficiencies::add_full_range_prof(&mut self.skills, s, prof);
            },
            Proficiency::Initiative(prof) => {
                if self.initiative <= prof {
                    self.initiative = prof;
                }
            },
        }
    }

    fn add_binary_prof<T: Ord>(v: &mut Vec<T>, new: T) -> () {
        match v.binary_search(&new) {
            Ok(_) => (),
            Err(idx) => v.insert(idx, new),
        }
    }

    fn add_full_range_prof<T: Ord>(
        v: &mut Vec<(T, ProficiencyLevel)>,
        new: T,
        prof: ProficiencyLevel,
    ) -> () {
        match v.binary_search_by(|x| x.0.cmp(&new)) {
            Ok(idx) => {
                if v.get(idx).unwrap().1 < prof {
                    *v.get_mut(idx).unwrap() = (new, prof);
                }
            },
            Err(idx) => v.insert(idx, (new, prof)),
        }
    }

    pub fn remove_proficiency(&mut self, p: Proficiency) -> Result<(), CharProfError> {
        match p {
            Proficiency::Tool(t, prof) => {
                CharProficiencies::remove_full_range_prof(&mut self.tools, &t, &prof)
            },
            Proficiency::Lang(l) => {
                CharProficiencies::remove_binary_prof(&mut self.languages, &l)
            },
            Proficiency::Weapon(w) => {
                CharProficiencies::remove_binary_prof(&mut self.weapons, &w)
            },
            Proficiency::Armor(a) => {
                CharProficiencies::remove_binary_prof(&mut self.armor, &a)
            },
            Proficiency::Save(s) => {
                CharProficiencies::remove_binary_prof(&mut self.saves, &s)
            },
            Proficiency::Skill(s, prof) => {
                CharProficiencies::remove_full_range_prof(&mut self.skills, &s, &prof)
            },
            Proficiency::Initiative(prof) => {
                if self.initiative == prof {
                    self.initiative = ProficiencyLevel::None;
                    Ok(())
                } else {
                    Err(CharProfError::NoChangeDueToProfMismatch)
                }
            },
        }
    }

    fn remove_binary_prof<T: Ord>(v: &mut Vec<T>, rem: &T) -> Result<(), CharProfError> {
        match v.binary_search(&rem) {
            Ok(idx) => {
                v.remove(idx);
                Ok(())
            },
            Err(_) => Err(CharProfError::RemoveNonExistentProf),
        }
    }

    fn remove_full_range_prof<T: Ord>(
        v: &mut Vec<(T, ProficiencyLevel)>,
        rem: &T, 
        prof: &ProficiencyLevel
    ) -> Result<(), CharProfError> {
        match v.binary_search_by(|x|x.0.cmp(&rem)) {
            Ok(idx) => {
                if v.get(idx).unwrap().1 == *prof {
                    v.remove(idx);
                    Ok(())
                } else {
                    Err(CharProfError::NoChangeDueToProfMismatch)
                }
            },
            Err(_) => Err(CharProfError::RemoveNonExistentProf),
        }
    }
}