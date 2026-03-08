use serde::{Deserialize, Serialize};

use crate::{basic::{DamageType, DieSize}, proficiencies::WeaponProf};

use super::inventory::{Item, ItemRarity};



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Weapon {
    name: String,
    description: String,
    weight: Option<u32>,
    rarity: ItemRarity,
    is_magic: bool,
    req_attunement: bool,
    base_dmg_die: DieSize,
    base_dmg_type: DamageType,
    extra_dmg: Vec<(DieSize, DamageType)>,
    category: WeaponProf,
    silvered: bool,
    adamantine: bool,
    properties: Vec<WeaponProperty>,
    equipped: bool,
    // TODO anything else to represent?
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponProperty {
    Ammunition,
    Finesse,
    Heavy,
    Light,
    Loading,
    Range{base: u32, long: u32},
    Reach,
    Special,
    Thrown{base: u32, long: u32},
    TwoHanded,
    Versatile{one_hand: DieSize, two_hand: DieSize},
}

impl Weapon {

}

impl Item for Weapon {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> &ItemRarity {
        &self.rarity
    }

    fn is_magic(&self) -> bool {
        self.is_magic
    }
    
    fn weight(&self) -> u32 {
        match self.weight {
            Some(w) => w,
            None => 0,
        }
    }
}