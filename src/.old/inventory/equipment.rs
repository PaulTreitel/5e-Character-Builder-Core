use serde::{Deserialize, Serialize};

use super::{inventory::{Item, ItemRarity}, money, container};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Equipment {
    name: String,
    description: String,
    weight: Option<u32>,
    cost: Option<money::Money>,
    consumable: bool,
    // TODO what else to represent?
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EquipmentPack {
    name: String,
    description: String,
    cost: money::Money,
    container: container::Container,
}

impl Item for Equipment {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> &ItemRarity {
        &ItemRarity::Common
    }

    fn is_magic(&self) -> bool {
        false
    }
    
    fn weight(&self) -> u32 {
        match self.weight {
            Some(w) => w,
            None => 0,
        }
    }
}

impl Item for EquipmentPack {
    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn rarity(&self) -> &ItemRarity {
        &ItemRarity::Common
    }

    fn is_magic(&self) -> bool {
        false
    }
    
    fn weight(&self) -> u32 {
        self.container.get_total_weight()
    }
}