pub mod equipment;
pub mod magic_item;
pub mod armor;
pub mod money;
pub mod weapon;
pub mod container;

use container::Container;
use inventory::ItemType;
use money::Money;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Inventory {
    money: Money,
    main_inventory: Vec<ItemType>,
    containers: Vec<Container>,
}

mod inventory {

    use std::default;

    use serde::{Deserialize, Serialize};

    use super::{
        armor::Armor, 
        container::Container, 
        equipment::{Equipment, EquipmentPack}, 
        magic_item::MagicItem, 
        weapon::Weapon
    };

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
    pub enum ItemRarity {
        #[default]
        Common,
        Uncommon,
        Rare,
        VeryRare,
        Legendary,
        Artifact,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum Equippable {
        Unequippable,
        Unequipped,
        Equipped,
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
    pub enum ItemType {
        BasicGear(Equipment),
        Pack(EquipmentPack),
        Armor(Armor),
        Weapon(Weapon),
        MagicItem(MagicItem),
        Container(Container),
    }

    pub trait Item {

        fn name(&self) -> &str;

        fn description(&self) -> &str;

        fn rarity(&self) -> &ItemRarity;

        fn is_magic(&self) -> bool;

        fn weight(&self) -> u32;
    }

    impl Item for ItemType {
        fn name(&self) -> &str {
            match self {
                ItemType::BasicGear(i) => i.name(),
                ItemType::Pack(i) => i.name(),
                ItemType::Armor(i) => i.name(),
                ItemType::Weapon(i) => i.name(),
                ItemType::MagicItem(i) => i.name(),
                ItemType::Container(i) => i.name(),
            }
        }
    
        fn description(&self) -> &str {
            match self {
                ItemType::BasicGear(i) => i.description(),
                ItemType::Pack(i) => i.description(),
                ItemType::Armor(i) => i.description(),
                ItemType::Weapon(i) => i.description(),
                ItemType::MagicItem(i) => i.description(),
                ItemType::Container(i) => i.description(),
            }
        }
    
        fn rarity(&self) -> &ItemRarity {
            match self {
                ItemType::BasicGear(i) => i.rarity(),
                ItemType::Pack(i) => i.rarity(),
                ItemType::Armor(i) => i.rarity(),
                ItemType::Weapon(i) => i.rarity(),
                ItemType::MagicItem(i) => i.rarity(),
                ItemType::Container(i) => i.rarity(),
            }
        }
    
        fn is_magic(&self) -> bool {
            match self {
                ItemType::BasicGear(i) => i.is_magic(),
                ItemType::Pack(i) => i.is_magic(),
                ItemType::Armor(i) => i.is_magic(),
                ItemType::Weapon(i) => i.is_magic(),
                ItemType::MagicItem(i) => i.is_magic(),
                ItemType::Container(i) => i.is_magic(),
            }
        }
    
        fn weight(&self) -> u32 {
            match self {
                ItemType::BasicGear(i) => i.weight(),
                ItemType::Pack(i) => i.weight(),
                ItemType::Armor(i) => i.weight(),
                ItemType::Weapon(i) => i.weight(),
                ItemType::MagicItem(i) => i.weight(),
                ItemType::Container(i) => i.weight(),
            }
        }
    }
}