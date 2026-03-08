use serde::{Deserialize, Serialize};

use super::inventory::{Item, ItemRarity, ItemType};



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct Container {
    name: String,
    description: String,
    capacity: String,
    equipped: bool,
    is_magic: bool,
    rarity: ItemRarity,
    weight: Option<u32>,
    weight_capacity: Option<u32>,
    contents: Vec<ItemType>,
}

impl Container {
    pub fn get_total_weight(&self) -> u32 {
        let container_weight = match self.weight {
            Some(w) => w,
            None => 0,
        };
        let content_weight = self.contents.iter()
            .map(|x| x.weight())
            .reduce(|acc, x| acc + x);
        match content_weight {
            Some(w) => container_weight + w,
            None => container_weight,
        }
    }

    pub fn items(&self) -> &Vec<ItemType> {
        &self.contents
    }

    pub fn equipped(&self) -> bool {
        self.equipped
    }

    pub fn add_item(&mut self, item: ItemType) -> () {
        self.contents.push(item);
    }
}

impl Item for Container {
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
        self.get_total_weight()
    }
}