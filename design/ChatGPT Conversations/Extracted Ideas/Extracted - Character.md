# Links

### Part 1
1. [[Part 1 Raw#3. Domain Layer (Pure Game Logic)|Character struct]] (and builder in 4)
2. [[Part 1 Raw#9. Character as a Mutable State|Another struct]]
3. [[Part 1 Raw#8. Choice System Table|Choice SQL]] (and character SQL in 9, 10)
4. [[Part 1 Raw#2. Recommended Domain Model|Another struct plus class levels]]
5. [[Part 1 Raw#2. The Correct Approach (Source-Based)|Struct + Sheet]]
6. [[Part 1 Raw#15. Rust Struct Layout|Again]] and [[Part 1 Raw#2. Character (Only Stores Choices)|again]]
7. [[Part 1 Raw#1. The Three Character Layers|Definition, Sheet, and State]]

### Part 2
1.  [[Part 2 Raw#3. Core Domain Types (Strongly Typed 5e)|Definition, Level, Sheet, and State]]
2. ...[[Part 2 Raw#7. Character Module|and again]]

### Part 3
1. [[Part 3 Raw#2. Game Model Layer (Runtime State)|State JSON]]

### Part 4
1. [[Part 4 Raw#2. Character Model|Another struct]]

### Part 5
1. [[Part 5 Raw#6. Character Sheet Cache|Cached character sheet]] (through 12)
2. [[Part 5 Raw#3. Immutable Character Snapshots|Character snapshots]]
3. [[Part 5 Raw#5. Character State|wildshape/polymorph/etc state]]
4. [[Part 5 Raw#18. Character Runtime Data|Runtime data (minimal)]]

### Part 6
1. [[Part 6 Raw#1. The Conceptual Model|wildshape part 2]]
2. [[Part 6 Raw#19. Character Model|Data Types (full architecture)]]

### Part 7
1. [[Part 7 Raw#2. Core Character Table|Character SQL]]

# Common Data Structure Elements

### struct Character / CharacterDefinition
- name, race, class, background, etc
	- class may be a `Vec` for multiclassing
- stats
	- `Stats` struct
	- `HashMap<Stat, i32>` or similar (`Stat` being an enum)
	- an `AbilityScores` struct or similar for the base stats
- features
	- `Vec<FeatureId>`
	- sometimes a separate `Vec<FeatId>` for recording feats specifically
- class levels
	- specific `ClassLevel` struct, store a `Vec`
- proficiencies
	- `HashSet` or a `HashMap` from the proficiency (e.g., heavy armor, Celestial, disguise kit) to its type (None, Half, Full, Expertise)
- addition: specific field for known wildshape forms
- alternate: store a state, rule graph, and sheet cache

### struct CharacterSheet
- store of compute values
	- final ability scores
	- speeds
	- proficiency `HashSet` as above
	- features `Vec` as above
	- max HP, proficiency bonus, optional spellcasting
	- stats `HashMap as above`
- broadly a way to represent derived information, the idea being that `Character` stores the choices, etc and `CharacterSheet` acts as the intermediary storing things derived from it

### struct ClassLevel
- class ID, level in that class, optional subclass ID

### struct CachedCharacter
- `Character` and `Option<CharacterSheet>`
- `HashMap` of stats to their "breakdowns" (here `Stat` is used in the broad sense including all derived values)

### struct CharacterState
- current HP, spent resources (hit dice, spell slots, etc), active conditions
- active form (e.g., wildshape)

### struct CharacterBuilder
- builder pattern for `Character`

# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub stats: Stats,
}

pub struct CharacterBuilder {
    character: Character,
}
```

#### #2
```Rust
pub struct Character {
    stats: HashMap<Stat, i32>,
    tags: HashSet<String>,
    features: Vec<FeatureId>,
}
```

#### #3
```SQL
choices
-------------
id TEXT PRIMARY KEY
entity_id TEXT
description TEXT
data JSON

characters
-------------
id TEXT PRIMARY KEY
name TEXT
system TEXT
created_at DATETIME

character_selections
-------------
character_id TEXT
entity_id TEXT
type TEXT

character_features
-------------
character_id TEXT
feature_id TEXT
source TEXT
```

#### #4
```Rust
pub struct Character {
    pub name: String,

    pub race: RaceId,
    pub class_levels: Vec<ClassLevel>,
    pub background: BackgroundId,

    pub abilities: AbilityScores,
    pub feats: Vec<FeatId>,

    pub features: Vec<FeatureId>,
}

pub struct ClassLevel {
    pub class: ClassId,
    pub level: u8,
    pub subclass: Option<SubclassId>,
}
```

#### #5
```Rust
pub struct Character {
    pub race: RaceId,
    pub classes: Vec<ClassLevel>,
    pub background: BackgroundId,
    pub feats: Vec<FeatId>,

    pub base_abilities: AbilityScores,
}

pub struct CharacterSheet {
    pub ability_scores: AbilityScores,
    pub speed: u32,
    pub proficiencies: HashSet<Proficiency>,
    pub features: Vec<FeatureId>,
}
```

#### #6
```Rust
pub struct Character {
    pub race: RaceId,
    pub class_levels: Vec<ClassLevel>,
    pub feats: Vec<FeatId>,
    pub base_abilities: AbilityScores,
}

pub struct CharacterSheet {
    pub abilities: AbilityScores,
    pub proficiencies: HashSet<Proficiency>,
    pub speed: u32,
}

struct CachedCharacter {
    character: Character,
    sheet: Option<CharacterSheet>,
}

// Again

pub struct Character {
    pub race: String,
    pub classes: Vec<ClassLevel>,
    pub feats: Vec<String>,
    pub base_abilities: AbilityScores,
}

pub struct ClassLevel {
    pub class: String,
    pub level: u8,
    pub subclass: Option<String>,
}

pub struct CharacterSheet {
    pub abilities: AbilityScores,
    pub speed: u32,
    pub proficiencies: HashSet<String>,
    pub features: Vec<String>,
}
```

#### #7
```Rust
pub struct CharacterDefinition {
    pub name: String,

    pub race: String,
    pub classes: Vec<ClassLevel>,
    pub feats: Vec<String>,

    pub base_abilities: AbilityScores,
}

pub struct CharacterSheet {
    pub abilities: AbilityScores,
    pub proficiency_bonus: i32,

    pub max_hp: i32,
    pub hit_dice: Vec<HitDie>,

    pub speed: u32,

    pub spellcasting: Option<Spellcasting>,

    pub features: Vec<String>,
}

pub struct CharacterState {
    pub current_hp: i32,

    pub spent_hit_dice: Vec<HitDie>,

    pub spell_slots_used: HashMap<u8, u8>,

    pub conditions: HashSet<String>,
}
```

### Part 2
#### #1
```Rust
pub struct CharacterDefinition {  
    pub race: RaceId,  
    pub classes: Vec<ClassLevel>,  
    pub feats: Vec<FeatId>,  
    pub base_abilities: AbilityScores,  
}

pub struct ClassLevel {  
    pub class: ClassId,  
    pub level: u8,  
    pub subclass: Option<SubclassId>,  
}

pub struct CharacterSheet {  
    pub abilities: AbilityScores,  
    pub proficiency_bonus: i32,  
  
    pub max_hp: i32,  
    pub speed: u32,  
  
    pub proficiencies: HashSet<Proficiency>,  
    pub features: Vec<FeatureId>,  
  
    pub spellcasting: Option<Spellcasting>,  
}

pub struct CharacterState {  
    current_hp: i32,  
    spell_slots_used: HashMap<u8, u8>,  
    spent_hit_dice: Vec<HitDie>,  
}
```

#### #2
```Rust
pub struct Character {  
    pub abilities: AbilityScores,  
    pub levels: Vec<ClassLevel>,  
    pub feats: Vec<FeatId>,  
    pub inventory: Inventory,  
}
```

### Part 3
```JSON
{  
  "character_id": "c123",  
  "class": "wizard",  
  "level": 5,  
  "hp": 28,  
  "inventory": [  
    { "def": "longsword_plus_1" },  
    { "def": "potion_healing", "quantity": 3 }  
  ],  
  "effects": [  
    { "effect": "mage_armor", "duration": 8 }  
  ]  
}
```

### Part 4

#### #1
```Rust
pub struct Character {  
    pub id: CharacterId,  
  
    pub race: RaceId,  
    pub classes: Vec<ClassLevel>,  
  
    pub ability_scores: AbilityScores,  
  
    pub feats: Vec<FeatId>,  
  
    pub inventory: Inventory,  
  
    pub conditions: Vec<ConditionInstance>,  
  
    pub active_spells: Vec<ActiveSpell>,  
}

pub struct ClassLevel {  
    pub class: ClassId,  
    pub level: u8,  
}
```

### Part 5

#### #1
```Rust
pub struct CharacterSheetCache {  
    pub stats: HashMap<Stat, StatBreakdown>,  
}

pub struct Character {  
    pub state: CharacterState,  
  
    pub rule_graph: CachedRuleGraph,  
    pub sheet_cache: CharacterSheetCache,  
}

pub struct CharacterSheet {  
    pub stats: HashMap<Stat, i32>,  
    pub breakdowns: HashMap<Stat, StatBreakdown>,  
}
```

#### #2
```Rust
pub struct CharacterSnapshot {  
    pub state: CharacterState,  
    pub compiled_rules: Vec<CompiledRule>,  
    pub stat_cache: HashMap<Stat, i32>,  
}
```

#### #3
```Rust
pub struct CharacterState {  
    pub active_form: Option<FormId>,  
}
```

#### #4
```
Character {  
    level  
    inventory  
    features  
    active_spells  
    conditions  
    resources  
}
```

### Part 6

#### #1
```Rust
struct Character {  
    known_wildshape_forms: Vec<CreatureId>,  
}
```

#### #2
```
Character  
CharacterId  
CharacterIdentity  
CharacterLevel  
CharacterClassLevels  
CharacterSpecies  
CharacterBackground  
CharacterFeats  
CharacterFeatures  
CharacterChoices  
CharacterInventory  
CharacterResources  
CharacterConditions  
CharacterSpellbook  
CharacterWildshapeForms
```

### Part 7

#### #1
```SQL
CREATE TABLE characters (  
    id INTEGER PRIMARY KEY,  
    name TEXT NOT NULL,  
    ruleset TEXT NOT NULL,  
    level INTEGER NOT NULL,  
    species_id INTEGER,  
    background_id INTEGER  
);

CREATE TABLE character_classes (  
    character_id INTEGER,  
    class_id INTEGER,  
    level INTEGER,  
    PRIMARY KEY(character_id, class_id)  
);

CREATE TABLE character_choices (  
    id INTEGER PRIMARY KEY,  
    character_id INTEGER,  
    choice_id INTEGER,  
    selection_type TEXT  
);

CREATE TABLE character_resources (  
    character_id INTEGER,  
    resource_id INTEGER,  
    current_value INTEGER  
);

CREATE TABLE character_conditions (  
    character_id INTEGER,  
    condition_id INTEGER,  
    source_rule INTEGER,  
    remaining_duration INTEGER  
);

CREATE TABLE character_meta (  
    character_id INTEGER,  
    key TEXT,  
    value TEXT  
);
```

# (Pseudo)Code

### Part 1
#### #1
```Rust
impl Character {
    pub fn modifier(score: u8) -> i8 {
        ((score as i8) - 10) / 2
    }
}

impl CharacterBuilder {

    pub fn new(name: String) -> Self { }

    pub fn set_race(&mut self, race: Race) { }

    pub fn set_class(&mut self, class: Class) { }

    pub fn assign_stats(&mut self, stats: Stats) { }

    pub fn build(self) -> Character { }
}
```

#### #5
```Rust
fn build_character_sheet(character: &Character) -> CharacterSheet {
    let features = collect_features(character);
    apply_effects(character, &features)
}
```

#### #6
```Rust
pub fn build_character_sheet(character: &Character, rules: &Rules) -> CharacterSheet {

    let features = collect_features(character, rules);

    apply_effects(&character.base_abilities, &features, rules)
}
```

#### #7
```Rust
fn apply_damage(state: &mut CharacterState, amount: i32) {
    state.current_hp -= amount;
}

fn short_rest(state: &mut CharacterState) {
    state.spent_hit_dice.clear();
}

fn long_rest(sheet: &CharacterSheet, state: &mut CharacterState) {
    state.current_hp = sheet.max_hp;
    state.spell_slots_used.clear();
    state.spent_hit_dice.clear();
}

impl CharacterState {
    pub fn apply_damage(&mut self, amount: i32) {
        self.current_hp -= amount;
    }

    pub fn heal(&mut self, amount: i32, max_hp: i32) {
        self.current_hp = (self.current_hp + amount).min(max_hp);
    }

    pub fn hp(&self) -> i32 {
        self.current_hp
    }
}

impl CharacterState {

    pub fn hp(&self) -> i32 {
        self.current_hp
    }

    pub fn spend_spell_slot(&mut self, level: u8) {
        *self.spell_slots_used.entry(level).or_insert(0) += 1;
    }

    pub fn reset_spell_slots(&mut self) {
        self.spell_slots_used.clear();
    }
}
```

### Part 2

#### #1
```Rust
impl CharacterState {  
  
    pub fn apply_damage(&mut self, dmg: i32) {  
        self.current_hp -= dmg;  
    }  
  
    pub fn spend_spell_slot(&mut self, level: u8) {  
        *self.spell_slots_used.entry(level).or_insert(0) += 1;  
    }  
  
}

pub fn build_character_sheet(  
    character: &CharacterDefinition,  
    rules: &Rules,  
) -> CharacterSheet {  
  
    let features = collect_features(character, rules);  
  
    apply_effects(&character.base_abilities, &features, rules)  
}
```

### Part 5

#### #1
```Rust
fn update_sheet(  
    cache: &mut CharacterSheetCache,  
    graph: &CachedRuleGraph,  
    dirty: &DirtyStats,  
) {  
    for stat in &dirty.stats {  
        cache.stats.insert(  
            *stat,  
            graph.values[stat]  
        );  
    }  
}
```

#### #2
```Rust
pub fn rebuild_snapshot(character: &CharacterState) -> CharacterSnapshot
```