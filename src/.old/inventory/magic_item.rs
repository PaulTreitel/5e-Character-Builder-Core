use serde::{Deserialize, Serialize};

use super::{inventory::{Equippable, Item, ItemRarity}, money::Money};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct MagicItem {
    name: String,
    description: String,
    rarity: ItemRarity,
    equipped: Equippable,
    weight: Option<u32>,
    cost: Option<Money>,
    req_attunement: bool,
    consumable: bool,
    // TODO what else to represent?
}

impl Item for MagicItem {
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
        true
    }
    
    fn weight(&self) -> u32 {
        match self.weight {
            Some(w) => w,
            None => 0,
        }
    }
}