use crate::{background::Background, class::Class, inventory::Inventory, race::Race};

use super::{
    char_attributes::{AbilityScores, Alignment, CreatureSize, CreatureType, Sense, Speeds}, 
    char_background::CharBackground, 
    char_class::CharClass, 
    char_description::CharDescription, 
    char_proficiencies::CharProficiencies, 
    defenses::Defenses, 
    health::CharHealth, 
    Character
};

impl Character {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_player(mut self, player: &str) -> Self {
        self.player = player.to_string();
        self
    }

    pub fn with_scores(mut self, scores: AbilityScores) -> Self {
        self.scores = scores.to_owned();
        self.base_scores = scores;
        self
    }

    pub fn with_race(mut self, race: &Race) -> Self {
        self.race = race.to_owned();
        self
    }

    pub fn with_background(mut self, bg: &Background) -> Self {
        self.background = CharBackground::from_background(bg);
        self
    }

    pub fn with_xp(mut self, xp: u32) -> Self {
        self.experience = xp;
        self
    }

    pub fn with_class(mut self, c: &Class, lvl: u8) -> Self {
        self.main_class = CharClass::new(c.to_owned(), lvl);
        self
    }

    pub fn with_multiclass(mut self, mc: Vec<CharClass>) -> Self {
        self.multiclasses = mc;
        self
    }

    pub fn with_speeds(mut self, s: Speeds) -> Self {
        self.speeds = s;
        self
    }

    pub fn with_senses(mut self, s: Vec<Sense>) -> Self {
        self.senses = s;
        self
    }
    
    pub fn with_size(mut self, size: CreatureSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_creature_type(mut self, creature_type: CreatureType) -> Self {
        self.creature_type = creature_type;
        self
    }

    pub fn with_alignment(mut self, a: Option<Alignment>) -> Self {
        self.alignment = a;
        self
    }

    pub fn with_proficiencies(mut self, p: CharProficiencies) -> Self {
        self.proficiencies = p.to_owned();
        self.base_proficiencies = p;
        self
    }

    pub fn with_inventory(mut self, i: Inventory) -> Self {
        self.inventory = i;
        self
    }

    pub fn with_inspiration(mut self, i: bool) -> Self {
        self.inspiration = i;
        self
    }

    pub fn with_health(mut self, h: CharHealth) -> Self {
        self.health = h;
        self
    }

    pub fn with_defenses(mut self, d: Defenses) -> Self {
        self.defenses = d;
        self
    }

    pub fn with_description(mut self, d: CharDescription) -> Self {
        self.description = d;
        self
    }

    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = notes;
        self
    }
}