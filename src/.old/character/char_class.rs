use serde::{Deserialize, Serialize};
use crate::class::Class;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct CharClass {
    class: Class,
    level: u8,
}

impl CharClass {
    pub fn new(class: Class, level: u8) -> Self {
        CharClass { class, level }
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn change_level(&mut self, amt: u8) -> () {
        self.level += amt;
    }
}