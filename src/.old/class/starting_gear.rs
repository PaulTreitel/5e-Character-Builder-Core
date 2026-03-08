use serde::{Deserialize, Serialize};

use crate::{basic::DieSize, inventory::container::Container, proficiencies::WeaponProf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct StartingGear {
    all_choices: Vec<StartingGearChoice>,
    guaranteed: Container,
    gold_option_dice: u8,
    gold_option_die: DieSize,
    gold_option_mult: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum StartingGearChoice {
    Armor(Vec<StartingArmorOption>),
    Pack(Vec<StartingPack>),
    WeaponAndMore(Vec<StartingWeaponOption>)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
struct StartingPack {
    name: String,
    contents: Container,
}

// TODO Is there a good way to require the armor/weapon names to match actual
// weapon/armor items? This might be the job of the homebrewing UI or a layer in 
// between. Whatever it is, it will need access to the items in all collections.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
struct StartingArmorOption {
    armor_name: String,
    proficiency_locked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum StartingWeaponOption {
    Weapon{ weapon: WeaponProf, count: u8, proficiency_locked: bool },
    RangedWeaponAndAmmunition{ 
        weapon: WeaponProf, 
        ammunition_name: String, 
        proficiency_locked: bool
    },
    WeaponAndShield{ weapon: WeaponProf, proficiency_locked: bool},
    Armor{ armor_name: String, proficiency_locked: bool },
    WeaponAndArmor{ 
        armor_name: String, 
        weapon: WeaponProf, 
        ammunition_name: Option<String>, 
        proficiency_level: bool
    },
}

impl Default for StartingGearChoice {
    fn default() -> Self {
        StartingGearChoice::Pack(Vec::new())
    }
}

impl Default for StartingWeaponOption {
    fn default() -> Self {
        StartingWeaponOption::Weapon { 
            weapon: WeaponProf::Specific { name: "longsword".to_string() }, 
            count: 1, 
            proficiency_locked: false 
        }
    }
}