# ***I am trying to design a D&D character building app written entirely in Rust. I intend to make it an interactive command line app then later add a GUI. How should I design the architecture of the app?***

Designing this well early will save you a lot of refactoring later, especially since you want:

1. Interactive CLI first
2. GUI later
3. Core logic in Rust

The key principle is: separate the game logic from the interface. Think of your project as three layers.

### 1. Recommended High-Level Architecture

Use a layered / hexagonal architecture:
```
┌──────────────────────────┐
│        Interfaces        │
│  CLI (now)   GUI (later) │
└─────────────┬────────────┘
              │
┌─────────────▼────────────┐
│      Application Layer   │
│ Character builder logic  │
│ workflows / use cases    │
└─────────────┬────────────┘
              │
┌─────────────▼────────────┐
│        Domain Layer      │
│ D&D rules + data models  │
└─────────────┬────────────┘
              │
┌─────────────▼────────────┐
│      Data / Storage      │
│ JSON / YAML / DB etc     │
└──────────────────────────┘
```
Why this works

* CLI and GUI share the same logic
* Game rules remain independent
* Testing becomes very easy

### 2. Suggested Rust Project Structure
```
dnd_builder/
│
├─ Cargo.toml
│
├─ src/
│  ├─ main.rs              # CLI entry
│
│  ├─ app/                 # application logic
│  │   ├─ mod.rs
│  │   ├─ builder.rs
│  │   ├─ commands.rs
│  │
│  ├─ domain/              # game rules
│  │   ├─ mod.rs
│  │   ├─ character.rs
│  │   ├─ race.rs
│  │   ├─ class.rs
│  │   ├─ stats.rs
│  │   └─ feat.rs
│  │
│  ├─ data/                # loading rules data
│  │   ├─ mod.rs
│  │   ├─ loader.rs
│  │   └─ repository.rs
│  │
│  ├─ cli/                 # command line UI
│  │   ├─ mod.rs
│  │   ├─ menu.rs
│  │   └─ prompts.rs
│  │
│  └─ gui/                 # placeholder for later
│      └─ mod.rs
```

Later you can move this into separate crates:


```
workspace/

dnd-core      # domain + rules
dnd-app       # application workflows
dnd-cli       # CLI interface
dnd-gui       # GUI interface
```
### 3. Domain Layer (Pure Game Logic)

This layer contains no CLI code.

Example:

``` Rust
pub struct Character {
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub stats: Stats,
}

pub struct Stats {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}
```

Example method:


``` Rust
impl Character {
    pub fn modifier(score: u8) -> i8 {
        ((score as i8) - 10) / 2
    }
}
```

This layer should contain:

- validation
- rules
- calculations
- derived stats

Example:

```
attack bonus
skill modifiers
spell slots
proficiencies
```
### 4. Application Layer (Character Builder Workflow)

This layer handles the process of building characters.

Example:

```
choose race
choose class
assign stats
choose background
choose spells
```

Example builder pattern:

``` Rust
pub struct CharacterBuilder {
    character: Character,
}

impl CharacterBuilder {

    pub fn new(name: String) -> Self { }

    pub fn set_race(&mut self, race: Race) { }

    pub fn set_class(&mut self, class: Class) { }

    pub fn assign_stats(&mut self, stats: Stats) { }

    pub fn build(self) -> Character { }
}
```

This layer contains:

* validation
* flow control
- game rule orchestration
### 5. CLI Layer (UI Only)

This layer should only deal with user interaction.

Example:
``` Rust
fn choose_race(builder: &mut CharacterBuilder) {
    println!("Choose a race:");

    for race in Race::all() {
        println!("{}", race);
    }

    let input = read_line();
    builder.set_race(parse_race(input));
}
```

Important: No rules here.

The CLI just:
```
display options
get input
call application logic
```
### 6. Data Layer (Game Data)

You probably don't want to hardcode everything.

Store D&D content as JSON/YAML.

Example:
```
data/
  races.json
  classes.json
  spells.json
  feats.json
```
Example JSON:
```
{
  "name": "Elf",
  "stat_bonus": {
    "dexterity": 2
  },
  "speed": 30
}
```
Load with serde.
``` Rust
#[derive(Deserialize)]
pub struct Race {
    name: String,
    dex_bonus: i8,
}
```
### 7. Command System (Optional but Powerful)

For an interactive CLI, you may want a command pattern.

Example:

> race elf
> class wizard
> stats roll
> save

Define commands:
``` Rust
trait Command {
    fn execute(&self, ctx: &mut AppContext);
}
```

Example:
```
SetRaceCommand
SetClassCommand
RollStatsCommand
SaveCharacterCommand
```
This makes the CLI extremely extensible.

8. State Management

Keep application state separate.

Example:
``` Rust
pub struct AppState {
    pub builder: CharacterBuilder,
    pub rules: RulesRepository,
}
```

This allows both CLI and GUI to manipulate the same state.

### 9. Testing Strategy

Rust excels here.

Domain tests
```
tests/
stat_modifiers.rs
spell_slots.rs
multiclass_rules.rs
```

Example:
```Rust
#[test]
fn modifier_18_is_4() {
    assert_eq!(Character::modifier(18), 4);
}
```
Builder tests
```
test_character_builder
test_stat_assignment
test_multiclass
```
### 10. GUI Later (Zero Rewrite)

Because logic is separated, a GUI becomes trivial.

Example with:

- `egui`
- `iced`
- `tauri`

GUI just calls the same builder:
```
button -> set_race()
dropdown -> choose_class()
slider -> assign_stat()
```
No rule duplication.

### 11. Recommended Rust Crates

CLI:

- `clap`
- `dialoguer`
- `inquire`

Serialization:

- `serde`
- `serde_json`
- `serde_yaml`

Data validation:

- `thiserror`

Testing:

- built-in Rust tests

GUI later:

- `egui`
- `iced`

### 12. One Important Tip for D&D Apps

Make rules data-driven.

Don't do:
```Rust
if race == "Elf" {
    dex += 2;
}
```

Do:
```
race.dex_bonus
race.features
race.speed
```
This allows you to add:
```
homebrew
future expansions
different editions
```
without changing code.

### 13. Example Flow
```
CLI
 ↓
Application layer
 ↓
CharacterBuilder
 ↓
Domain rules
 ↓
Data repository
```

✅ If you'd like, I can also show you:

- A full Rust example project layout
- A Rust trait-based system for races/classes
- A data-driven rule engine for D&D
- A very clean CLI architecture used in real Rust tools (like cargo, ripgrep).

# ***how would this change for supporting deep homebrew systems?***

Supporting deep homebrew systems changes the architecture in an important way:

You should shift from hard-coded rules to a data-driven + rule engine architecture.

Instead of your program knowing D&D, it should act as a generic character rules engine that loads rules from content packs.


### 1. Core Principle: Treat Rules as Data

Instead of:
```Rust
if race == Race::Elf {
    character.dex += 2;
}
```
You want something like:
```JSON
{
  "name": "Elf",
  "effects": [
    { "type": "stat_bonus", "stat": "dexterity", "value": 2 }
  ]
}
```

Then the engine interprets those effects.

This allows:
```
official rules
homebrew races
entire custom systems
variant rules
```
without recompiling.

### 2. New Architecture

Your architecture becomes:
```
Interfaces
  CLI / GUI
        │
Application Layer
  Character builder workflow
        │
Rules Engine
  Effect interpreter
  Validation
        │
Content System
  JSON / YAML / TOML packs
        │
Domain Models
  Character state
```
The rules engine becomes the center.

### 3. Content Packs

Everything should live in content packs.

Example structure:
```
content/
  dnd5e/
    races.json
    classes.json
    feats.json
  grimdark_homebrew/
    races.json
    classes.json
    magic.json
```
Each pack defines:
```
entities
features
effects
requirements
```
### 4. Entity-Based System

Define everything as an entity with features.

Example race:
```JSON
{
  "id": "elf",
  "name": "Elf",
  "features": ["elf_dex_bonus", "darkvision"]
}
```
Feature:
```JSON
{
  "id": "elf_dex_bonus",
  "effects": [
    { "type": "modify_stat", "stat": "dexterity", "value": 2 }
  ]
}
```
Your engine loads the features and applies them.

### 5. Effect System (Critical for Homebrew)

Define a generic effect enum.

Example Rust:
```Rust
enum Effect {
    ModifyStat { stat: Stat, value: i32 },
    GrantSkill { skill: Skill },
    AddSpell { spell: String },
    SetSpeed { value: u32 },
}
```
Then apply them dynamically.
```Rust
fn apply_effect(character: &mut Character, effect: &Effect) {
    match effect {
        Effect::ModifyStat { stat, value } => {
            character.modify_stat(*stat, *value);
        }
        Effect::GrantSkill { skill } => {
            character.skills.insert(*skill);
        }
    }
}
```
Now homebrew can add combinations of effects.

### 6. Requirement System

Homebrew systems need conditional rules.

Example:
```
requires:
  class = wizard
  level >= 3
```
Rust model:
```Rust
enum Requirement {
    Class(String),
    Level(u8),
    StatAbove { stat: Stat, value: u8 },
}
```
Engine:
```Rust
fn check_requirement(character: &Character, req: &Requirement) -> bool
```
### 7. Generic "Choice" System

Homebrew often requires nested choices.

Example:
```
Choose one:
 - Fire Magic
 - Ice Magic
 - Storm Magic
```
Represent it as data.
```JSON
{
  "type": "choice",
  "options": [
    "fire_magic",
    "ice_magic",
    "storm_magic"
  ]
}
```
Your CLI/GUI renders this automatically.

### 8. Feature Graph (Very Powerful)

Everything becomes features that grant other features.

Example:
```
Elf
 ├─ Darkvision
 ├─ Keen Senses
 └─ Dex Bonus
```
Graph:
```
Race
 → Features
 → Effects
```
Rust:
```Rust
struct Feature {
    id: String,
    effects: Vec<Effect>,
    grants: Vec<String>,
}
```
This allows:
```
subclasses
feats
prestige classes
homebrew traits
```
### 9. Character as a Mutable State

Instead of rigid fields:
```
Character
```
Use a state container.

Example:
```Rust
pub struct Character {
    stats: HashMap<Stat, i32>,
    tags: HashSet<String>,
    features: Vec<FeatureId>,
}
```
This allows homebrew to introduce:
```
sanity
corruption
mana
luck
karma
```
without code changes.

10. Data Schema Versioning

Homebrew means users will break things.

Version your schemas.

Example:
```JSON
{
  "schema_version": 1,
  "features": [...]
}
```
### 11. Validation System

Add validation passes.
```
load content
validate references
detect circular features
validate requirements
```
Example:
```
feature references that don't exist
duplicate IDs
infinite loops
```
12. CLI Design Change

The CLI shouldn't know about races/classes specifically.

Instead:
```
Select option: race
Select option: class
Select option: subclass
```
Better:
```
Choose category
Choose option
Apply features
```
The UI becomes generic.

### 13. GUI Advantage

Because the system is data-driven, the GUI can automatically generate:
```
dropdowns
checkboxes
feature descriptions
trees
```
from the data.

### 14. Optional: Embedded Scripting

If you want very deep homebrew, add scripting.

Possible options:

Rust embedded scripting:

- `rhai`
- `lua (via mlua)`
- `wasm`

Example script:
```rhai
if character.level >= 5 {
    character.stats["strength"] += 2;
}
```
This enables fully custom rules.

### 15. Recommended Architecture for Your Case

I would structure the core like this:
```
dnd-core
  character/
  stats/
  engine/
  effects/
  requirements/

dnd-content
  loaders
  schemas

dnd-cli
  interactive interface

dnd-gui
  future GUI
```
### 16. Real Systems That Use This Design

Your architecture will resemble:

- Foundry VTT
- Pathfinder 2e character builders
- tabletop simulator rule engines

These all use effect systems + feature graphs.

### 17. Biggest Mistake to Avoid

Do NOT encode rules like:
```
Race
Class
Subclass
Feat
```
Instead use:
```
Entity
Feature
Effect
Requirement
Choice
```
Those abstractions allow any RPG system.

✅ If you'd like, I can also show you a very powerful Rust pattern that works extremely well for this:

an Entity-Component-System (ECS) architecture for RPG rule engines

It’s surprisingly perfect for homebrew-heavy tabletop systems and scales far better than traditional OOP-style designs.

</details>

# ***how would I design the database system for this app?***

For a deep-homebrew RPG character builder, the database should support:

- Content packs (official rules + homebrew)
- Versioning
- graph-like relationships (features → effects → requirements)
- fast loading for rule evaluation
- offline-first usage

The best design is usually a hybrid approach:

Relational DB for structure + JSON for flexible rule data.

For Rust apps this commonly means:

- SQLite (local storage)
- JSON columns for rule definitions
- Serde models for loading

This gives you flexibility without losing queryability.

### 1. High-Level Database Architecture

Your database will store three types of data:
```
CONTENT
  rules, races, classes, features, effects

USER DATA
  characters
  saved builds
  choices

SYSTEM
  content packs
  versions
```
Conceptually:
```
Content Packs
      │
Entities (Race/Class/Feat/etc)
      │
Features
      │
Effects + Requirements
```
And separately:
```
Characters
   │
Selections
   │
Applied Features
```
### 2. Recommended Storage Engine

For this type of application:

Use SQLite.

Advantages:

- zero configuration
- embeddable
- fast
- great Rust ecosystem
- portable save files
- supports JSON

Good Rust crates:

- `sqlx`
- `rusqlite`
- `sea-orm` (if you want an ORM)

For your case, `sqlx` is ideal.

### 3. Content Pack Tables

Content packs let users install rule sets.

Example table:
```SQL
content_packs
-------------
id TEXT PRIMARY KEY
name TEXT
version TEXT
description TEXT
author TEXT
enabled BOOLEAN
```
Examples:
```
dnd5e_core
grimdark_homebrew
my_custom_setting
```
### 4. Entity Table (Core Rule Objects)

Everything should be stored as a generic entity.
```SQL
entities
-------------
id TEXT PRIMARY KEY
pack_id TEXT
type TEXT
name TEXT
description TEXT
data JSON
```
Examples of type:
```
race
class
feat
spell
background
feature
```
Example row:
```
id: elf
type: race
pack_id: dnd5e_core
```
data might contain:
```JSON
{
  "speed": 30,
  "size": "medium",
  "features": ["darkvision", "keen_senses"]
}
```
This avoids rigid schema problems.

### 5. Feature Table

Features are the core rule units.
```SQL
features
-------------
id TEXT PRIMARY KEY
pack_id TEXT
name TEXT
description TEXT
data JSON
```
Example data:
```JSON
{
  "effects": [
    { "type": "modify_stat", "stat": "dexterity", "value": 2 }
  ]
}
```
6. Effect Storage

You can either:

Option A (recommended): store effects as JSON

Example:
```JSON
{
  "type": "modify_stat",
  "stat": "dexterity",
  "value": 2
}
```
Rust enum:
```Rust
enum Effect {
    ModifyStat { stat: Stat, value: i32 },
    GrantSkill { skill: String },
    AddFeature { feature_id: String },
}
```
Deserialize using serde.

This keeps the database flexible for homebrew.

### 7. Requirement Storage

Requirements should also be JSON.

Example:
```JSON
{
  "type": "level_at_least",
  "value": 3
}
```
Or:
```JSON
{
  "type": "class",
  "value": "wizard"
}
```
Rust:
```Rust
enum Requirement {
    LevelAtLeast(u8),
    Class(String),
    StatAbove { stat: Stat, value: i32 },
}
```
### 8. Choice System Table

Choices are important for character builders.

Example:
```SQL
choices
-------------
id TEXT PRIMARY KEY
entity_id TEXT
description TEXT
data JSON
```
Example JSON:
```JSON
{
  "type": "select_feature",
  "options": [
    "fire_magic",
    "ice_magic",
    "storm_magic"
  ]
}
```
### 9. Character Storage

Characters should store decisions, not recalculated stats.
```SQL
characters
-------------
id TEXT PRIMARY KEY
name TEXT
system TEXT
created_at DATETIME
```
Selections:
```SQL
character_selections
-------------
character_id TEXT
entity_id TEXT
type TEXT
```
Examples:
```SQL
character_id | entity_id | type
--------------------------------
char1        | elf       | race
char1        | wizard    | class
char1        | fireball  | spell
```
### 10. Character Feature Table

To speed up recalculation you may store applied features.
```SQL
character_features
-------------
character_id TEXT
feature_id TEXT
source TEXT
```
Example:
```
darkvision
source: race:elf
```
But you can also recompute them dynamically.

### 11. Versioning System

Homebrew often changes.

Add versions:
```SQL
content_versions
-------------
pack_id
version
installed_at
```
Characters reference:
```
system_version
```
This prevents rule breakage.

### 12. Caching Strategy (Important)

Loading rules repeatedly is expensive.

Recommended:
```
Database → Rust structs → In-memory rule graph
```
At startup:

1. Load all rules
2. Build feature graph
3. Cache in memory

Example:
```Rust
struct RulesDatabase {
    entities: HashMap<String, Entity>,
    features: HashMap<String, Feature>,
}
```
Runtime rule evaluation becomes very fast.

### 13. Graph Relationships

Rules are actually a graph.

Example:
```
Race (Elf)
   │
   ├── Feature (Dex Bonus)
   └── Feature (Darkvision)
           │
           └── Effect (Vision 60ft)
```
You should build this graph in memory, not with SQL joins.

14. Content Pack Installation

Homebrew should be installable.

Example workflow:
```
homebrew pack
     ↓
validate JSON
     ↓
insert into DB
     ↓
rebuild rule graph
```
### 15. File-Based Alternative (Worth Considering)

For a local RPG builder, many projects skip SQL entirely and use:
```
rules/
  races.json
  classes.json
  features.json

characters/
  bob_the_wizard.json
```
Advantages:

- easier homebrew
- git friendly
- human readable
- simpler code

Many real tools do this (including Foundry modules).

### 16. Best Hybrid Design

Many successful systems use:
```
Rules → JSON files
Characters → SQLite
```
Because:

- rules are modded often
- characters require fast querying

## 17. Rust Model Example

Entity model:
```Rust
#[derive(Deserialize)]
pub struct Entity {
    pub id: String,
    pub name: String,
    pub entity_type: EntityType,
    pub data: serde_json::Value,
}
```
Feature:
```Rust
#[derive(Deserialize)]
pub struct Feature {
    pub id: String,
    pub effects: Vec<Effect>,
}
```
### 18. Final Recommended Architecture
```
Content Packs
   ↓
JSON Files
   ↓
Loader
   ↓
SQLite cache
   ↓
Rule Graph (in memory)
   ↓
Character builder engine
```
✅ If you'd like, I can also show you something extremely useful for this project:

- a complete Rust schema + loader system
- a feature graph implementation
- a rule engine that supports unlimited homebrew

This pattern is used in several professional TTRPG engines, and it fits Rust extremely well.

# ***how do entities work and why are they being used here?***

Entities are used here to decouple your engine from specific RPG concepts (race, class, feat, spell, etc.). Instead of hard-coding those types into the program, you represent everything as a generic object (“entity”) with data and relationships. This is what allows deep homebrew and even entirely different systems without rewriting the engine.

Let’s break this down clearly.

### 1. What an Entity Is

An entity is simply a uniquely identifiable object in your rules system.

Minimal structure:
```Rust
pub struct Entity {
    pub id: String,
    pub name: String,
    pub kind: EntityKind,
    pub data: serde_json::Value,
}
```
Example entity:
```JSON
{
  "id": "elf",
  "kind": "race",
  "name": "Elf",
  "data": {
    "speed": 30,
    "size": "medium",
    "features": ["darkvision", "keen_senses"]
  }
}
```
The entity itself is just a container for information.

2. Why Entities Are Used

Without entities, you would do something like this:
```
struct Race { ... }
struct Class { ... }
struct Feat { ... }
struct Spell { ... }
```
Then your builder would contain logic like:
```Rust
if race == Race::Elf {
    dex += 2;
}
```
This approach fails for homebrew because:

- new rule types require new code
- mods require recompiling
- systems can't evolve dynamically

Entities solve this by making your engine data-driven.

Instead:
```
Character
  ├── Entity: elf
  ├── Entity: wizard
  └── Entity: fireball
```
Your engine doesn't care what those are—it just processes their features and effects.

### 3. Entities Represent Everything

In a flexible system, these are all entities:
```
Race
Class
Subclass
Feat
Spell
Background
Feature
Item
Ability
Condition
```
Example:
```
Entity: elf
Entity: wizard
Entity: darkvision
Entity: fireball
Entity: stealth_proficiency
```
Your engine just loads them.

### 4. Entities Have Features

Entities usually grant features.

Example:
```
Elf (entity)
   ↓
features:
  - darkvision
  - keen_senses
  - elf_dex_bonus
```
Those features are also entities or feature objects.

Example:
```JSON
{
  "id": "elf_dex_bonus",
  "effects": [
    { "type": "modify_stat", "stat": "dexterity", "value": 2 }
  ]
}
```
The engine applies the effects.

### 5. Entity Relationships

Entities form a graph.

Example:
```
Wizard (entity)
  ├── spellcasting_feature
  ├── arcane_recovery
  └── subclass_choice
         ↓
    Evocation
    Illusion
    Necromancy
```
Graph structure:
```
Entity → Features → Effects
```
### 6. Character as a Collection of Entities

A character simply stores selected entities.

Example:
```
Character
  ├── race: elf
  ├── class: wizard
  ├── background: sage
  └── feat: alert
```
From that, the engine calculates:
```
features
effects
final stats
```
### 7. Why This Is Perfect for Homebrew

Suppose someone adds a custom race.

They just add:
```JSON
{
  "id": "dragonkin",
  "type": "race",
  "features": ["dragon_breath", "scales"]
}
```
No code changes required.

The engine simply loads the entity.

### 8. Entities vs Hardcoded Types

Hardcoded approach:
```
Race struct
Class struct
Feat struct
Spell struct
```
Entity approach:
```
Entity
 ├─ type = race
 ├─ type = class
 ├─ type = feat
 └─ type = spell
```
You reduce your engine complexity dramatically.

9. Entities vs ECS (Important Distinction)

This design is inspired by ECS, but not identical.

ECS in games:
```
Entity
  + Component
  + Component
```
Your system is more like:
```
Entity
  + Features
      + Effects
```
Example:
```
Elf
  └── Feature: Dex Bonus
          └── Effect: +2 Dex
```
### 10. Example Flow in the Engine

User chooses a race.
```
Select entity: elf
```
Engine loads:
```
Entity elf
   ↓
Features
   ↓
Effects
   ↓
Apply to character
```
Rust-style pseudo code:
```Rust
fn apply_entity(character: &mut Character, entity: &Entity) {
    for feature in &entity.features {
        apply_feature(character, feature);
    }
}
```
### 11. Database Representation

Entities table:
```
entities
---------------------------------
id
type
name
description
data
```
Example rows:
```
elf           race
wizard        class
darkvision    feature
fireball      spell
```
Your engine builds the relationships after loading.

### 12. Benefits of the Entity System
 
  1. Modding

Users add content easily.

2. System Agnostic

You can support:
```
D&D
Pathfinder
homebrew RPGs
entirely custom systems
```
3. Reduced Code Complexity

Your engine only understands:
```
entities
features
effects
requirements
```
Everything else is data.

4. Extensibility

Someone can invent new mechanics like:
```
sanity
corruption
mana
luck
```
without touching Rust code.

### 13. Mental Model

Think of entities like nodes in a rule graph.

Example:
```
Elf
 │
 ├─ Darkvision
 │     └─ Effect: Vision 60ft
 │
 └─ Dex Bonus
       └─ Effect: +2 Dex
```
Your engine just walks the graph.

### 14. Key Insight

Your application is not a D&D program.

It becomes a generic tabletop rules engine.

The system only understands:
```
entities
features
effects
requirements
choices
```
Everything else is content.

✅ If you'd like, I can also show you the ideal Rust struct design for entities, features, effects, and requirements that scales extremely well (and avoids common performance problems in rule engines).

# ***I don't want to build something general enough to include different systems, just complex rules for homebrew following the D&D 5e system structure***

If you only want to support D&D 5e (plus heavy homebrew), the architecture should change in an important way:

You should not fully generalize everything into generic entities. That level of abstraction is mainly useful when supporting multiple RPG systems.

Instead, you want a typed domain model for 5e with a data-driven feature/effect system.

This gives you:

✅ strong Rust type safety
✅ simpler code
✅ good homebrew support
✅ easier rule validation

while avoiding an over-engineered engine.

### 1. Core Idea

Your engine should understand the 5e structure, but allow custom content inside it.

The key abstractions become:
```
Character
Race
Subrace
Class
Subclass
Background
Feat
Spell
Feature
```
And the Feature system handles most rule effects.

### 2. Recommended Domain Model

Your core Rust types should look something like:
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
```
Class levels:
```Rust
pub struct ClassLevel {
    pub class: ClassId,
    pub level: u8,
    pub subclass: Option<SubclassId>,
}
```
Ability scores:
```Rust
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}
```
These are hardcoded concepts because they are stable in 5e.

### 3. The Feature System (Where Homebrew Lives)

Almost every ability in 5e can be represented as a feature.

Examples:
```
Darkvision
Fighting Style
Rage
Sneak Attack
Spellcasting
Extra Attack
```
Rust model:
```Rust
pub struct Feature {
    pub id: FeatureId,
    pub name: String,
    pub description: String,

    pub effects: Vec<Effect>,
}
```
Your homebrew complexity lives here.

### 4. Effects (Rule Engine Layer)

Instead of hardcoding rules, use a typed effect system.

Example:
```Rust
pub enum Effect {
    AbilityScoreBonus {
        ability: Ability,
        amount: i32,
    },

    GrantProficiency {
        proficiency: Proficiency,
    },

    GrantFeature {
        feature: FeatureId,
    },

    IncreaseSpeed {
        amount: u32,
    },

    Spellcasting {
        ability: Ability,
        spell_list: SpellListId,
    },
}
```
Example: Elf racial bonus
```Rust
AbilityScoreBonus {
    ability: Ability::Dexterity,
    amount: 2
}
```
Example: Darkvision
```Rust
GrantFeature {
    feature: "darkvision"
}
```
### 5. Why This Works for Homebrew

Homebrew authors can create:
```
new races
new subclasses
new feats
new features
```
They just compose effects.

Example homebrew feature:
```JSON
{
  "name": "Frostborn",
  "effects": [
    { "type": "resistance", "damage": "cold" },
    { "type": "spell", "spell": "armor_of_agathys" }
  ]
}
```
Your engine already knows how to apply those.

### 6. Race Example

Race struct:
```Rust
pub struct Race {
    pub id: RaceId,
    pub name: String,
    pub speed: u32,
    pub size: Size,

    pub ability_bonuses: Vec<AbilityBonus>,
    pub features: Vec<FeatureId>,
}
```
Example JSON:
```JSON
{
  "id": "elf",
  "name": "Elf",
  "speed": 30,
  "size": "medium",
  "ability_bonuses": [
    { "ability": "dexterity", "amount": 2 }
  ],
  "features": ["darkvision", "keen_senses"]
}
```
7. Class Example
```Rust
pub struct Class {
    pub id: ClassId,
    pub name: String,

    pub hit_die: u8,
    pub primary_abilities: Vec<Ability>,

    pub level_features: HashMap<u8, Vec<FeatureId>>,
}
```
Example:
```JSON
{
  "id": "fighter",
  "hit_die": 10,
  "level_features": {
    "1": ["fighting_style", "second_wind"],
    "2": ["action_surge"]
  }
}
```
8. Subclass Structure

Subclass:
```Rust
pub struct Subclass {
    pub id: SubclassId,
    pub class: ClassId,

    pub level_features: HashMap<u8, Vec<FeatureId>>,
}
```
Example:
```
Wizard
 └── Evocation
      ├── Sculpt Spells
      └── Potent Cantrip
```
### 9. Database Design

Since you are targeting 5e specifically, the database should reflect that.

Tables:
```
races
subraces
classes
subclasses
backgrounds
feats
features
spells
characters
```
Example table:
```SQL
races
-----
id
name
speed
size
data_json
```
Feature table:
```SQL
features
--------
id
name
description
effects_json
```
Characters:
```SQL
characters
----------
id
name
race_id
background_id
ability_scores_json
```
Class levels:
```SQL
character_classes
-----------------
character_id
class_id
level
subclass_id
```
### 10. Rules Evaluation

Your engine should derive features dynamically.

Example process:
```
Character
  ↓
Race features
  ↓
Class features by level
  ↓
Subclass features
  ↓
Background features
  ↓
Feat features
```
Combine all features → apply effects.

### 11. Rule Engine Loop

Pseudo Rust:
```Rust
fn compute_features(character: &Character) -> Vec<FeatureId> {

    let mut features = Vec::new();

    features.extend(race.features);

    for class_level in &character.class_levels {
        features.extend(class_features(class_level));
    }

    features.extend(character.feats.features());

    features
}
```
Then apply effects.

### 12. Where Homebrew Fits

Homebrew creators can add:
```
races
subraces
classes
subclasses
feats
features
spells
```
They do not change engine logic.

They just define:
```
features
effects
```
### 13. When You Should NOT Generalize Further

Avoid building a fully generic entity system if:

- you only support 5e
- you want strong Rust types
- you want easier debugging
- you want clearer rules code

The 5e structure is stable enough to encode directly.

### 14. Ideal Project Structure
```
dnd_core/
  character/
  abilities/
  combat/
  features/
  effects/

dnd_data/
  races/
  classes/
  spells/

dnd_engine/
  feature_engine.rs
  character_builder.rs

dnd_cli/
  ui/
```
### 15. The One Abstraction That Matters

The most important design decision is:
```
Feature + Effect system
```
Everything else can stay strongly typed to 5e.

That gives you powerful homebrew without losing simplicity.

✅ If you'd like, I can also show you the most common architectural mistake in D&D character builders that causes massive refactoring later (and how to avoid it early in Rust).

# ***Please Do \[Most Common Architectural Mistake, Not Deriving State From Sources]***

The most common architectural mistake in D&D character builders is this:

Mutating the character state directly when choices are made instead of deriving it from sources.

This sounds harmless, but it becomes a nightmare once you add:

- multiclassing
- respec/editing characters
- conditional features
- homebrew
- rule interactions
- recalculation after changes

The correct model is source-based derivation instead of incremental mutation.

### 1. The Bad Approach (Mutation-Based)

Many builders do this:
```Rust
character.dexterity += 2; // elf bonus
character.proficiencies.push(Skill::Perception);
```
This happens when the user selects things.

Example flow:
```
Choose race → modify stats
Choose class → modify proficiencies
Choose feat → modify abilities
```
Your character struct becomes:
```Rust
pub struct Character {
    pub strength: i32,
    pub dexterity: i32,
    pub proficiencies: Vec<Skill>,
}
```
Why this breaks later

Imagine the user changes race.

You must now:
```
remove old race bonuses
remove race features
apply new race bonuses
recalculate dependent values
```
This becomes very fragile.

You get bugs like:

- bonuses stacking twice
- features not removed
- stats drifting over time

### 2. The Correct Approach (Source-Based)

Instead, your character should store only choices.
```Rust
pub struct Character {
    pub race: RaceId,
    pub classes: Vec<ClassLevel>,
    pub background: BackgroundId,
    pub feats: Vec<FeatId>,

    pub base_abilities: AbilityScores,
}
```
Notice what's missing:
```
no final stats
no derived bonuses
no computed proficiencies
```
Those are calculated.

### 3. Derived Character State

You compute a derived sheet from sources.
```Rust
pub struct CharacterSheet {
    pub ability_scores: AbilityScores,
    pub speed: u32,
    pub proficiencies: HashSet<Proficiency>,
    pub features: Vec<FeatureId>,
}
```
Pipeline:
```
Character choices
      ↓
collect features
      ↓
apply effects
      ↓
derive final sheet
```
This makes recalculation trivial.

### 4. Rule Evaluation Pipeline

A good engine works like this:
```
Character
   ↓
Collect Sources
   race
   class levels
   subclass
   background
   feats
   ↓
Resolve Features
   ↓
Apply Effects
   ↓
Derived Character Sheet
```
Example pipeline in Rust:
```Rust
fn build_character_sheet(character: &Character) -> CharacterSheet {
    let features = collect_features(character);
    apply_effects(character, &features)
}
```
No mutation.

### 5. Why This Is So Important

Because 5e rules stack from many places.

Example:
```
Elf race
+ Fighter class
+ Champion subclass
+ Alert feat
+ magic item
+ spell buff
```
All contribute effects.

Your engine must recompute cleanly.

6. Real Example

Character:
```
Race: Elf
Class: Ranger
Feat: Sharpshooter
```
Feature collection:
```
Darkvision
Keen Senses
Favored Enemy
Natural Explorer
Sharpshooter
```
Effects:
```
+2 Dex
Perception proficiency
+10 ranged damage option
60ft darkvision
```
These are applied every time you rebuild the sheet.

7. Handling Multiclassing

Mutation-based systems break badly here.

Example:
```
Fighter 3 / Rogue 2
```
Features come from:
```
Fighter level features
Rogue level features
Subclass features
```
With a derivation model, this is easy.

### 8. Handling Editing

User changes class:
```
Fighter → Paladin
```
In a mutation system:
```
undo fighter bonuses
remove fighter features
apply paladin bonuses
```
In a derivation system:
```
update class
rebuild sheet
```
Done.

### 9. Derived vs Stored Data

Store only sources.

Good:
```
race
class levels
subclass
background
feats
base ability scores
```
Derived:
```
final ability scores
proficiencies
AC
speed
spell slots
skill modifiers
```
Never store derived values.

10. How Features Fit Into This

Your pipeline becomes:
```
Character
   ↓
collect features
   ↓
features produce effects
   ↓
effects modify derived sheet
```
Rust concept:
```Rust
trait EffectApplier {
    fn apply(&self, sheet: &mut CharacterSheet);
}
```
Each effect modifies the sheet.

### 11. Example Effect
```Rust
pub enum Effect {
    AbilityBonus {
        ability: Ability,
        amount: i32,
    },
}
```
Apply:
```Rust
match effect {
    Effect::AbilityBonus { ability, amount } => {
        sheet.ability_scores.add(*ability, *amount);
    }
}
```
### 12. Order of Operations

Some effects depend on others.

Example:
```
base stat
+ racial bonus
+ feat bonus
→ modifier
→ skill bonus
```
So the pipeline may be:
```
apply ability bonuses
calculate modifiers
apply skill bonuses
```
You may implement rule phases.

### 13. Another Mistake to Avoid

Don't store features directly on the character.

Store sources, derive features.

Bad:
```Rust
character.features.push("darkvision");
```
Good:
```Rust
race.features -> collected
```
### 14. Ideal Character Architecture
```
Character (sources)
   ↓
Feature Resolver
   ↓
Effect Engine
   ↓
Derived Character Sheet
```
This pattern is used in:

- professional character builders
- Foundry VTT systems
- Pathfinder 2e rule engines

### 15. Rust Struct Layout
```Rust
pub struct Character {
    pub race: RaceId,
    pub class_levels: Vec<ClassLevel>,
    pub feats: Vec<FeatId>,
    pub base_abilities: AbilityScores,
}
```
Derived:
```Rust
pub struct CharacterSheet {
    pub abilities: AbilityScores,
    pub proficiencies: HashSet<Proficiency>,
    pub speed: u32,
}
```
### 16. Performance Tip

Compute the sheet only when needed.

Cache it:
```Rust
struct CachedCharacter {
    character: Character,
    sheet: Option<CharacterSheet>,
}
```
Invalidate cache on changes.

### 17. The Key Rule

Your engine should follow this rule:

> Characters store decisions, not results.

Everything else is computed.

✅ If you'd like, I can also show you a very clean Rust rule engine architecture (about ~150 lines) that handles features, effects, multiclassing, and homebrew cleanly. It’s a pattern used in several serious TTRPG tools.

# ***Please Do \[Rust Rule Engine Architecture]***

Below is a clean, minimal Rust rule engine architecture (~150–200 lines conceptually) that supports:

- races
- classes
- subclasses
- feats
- multiclassing
- homebrew features
- derived character sheets

The design is intentionally simple and extensible, which is ideal for a Rust CLI tool that will later gain a GUI.

The architecture has four layers:
```
Character (player choices)
        ↓
Feature Resolver
        ↓
Effect Engine
        ↓
Derived Character Sheet
```
### 1. Core Types

Start with strongly typed enums.
```Rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}
```
Ability scores:
```Rust
#[derive(Debug, Clone)]
pub struct AbilityScores {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}
```
Helper method:
```Rust
impl AbilityScores {
    pub fn add(&mut self, ability: Ability, amount: i32) {
        match ability {
            Ability::Strength => self.strength += amount,
            Ability::Dexterity => self.dexterity += amount,
            Ability::Constitution => self.constitution += amount,
            Ability::Intelligence => self.intelligence += amount,
            Ability::Wisdom => self.wisdom += amount,
            Ability::Charisma => self.charisma += amount,
        }
    }
}
```
### 2. Character (Only Stores Choices)

This struct never stores derived values.
```Rust
pub struct Character {
    pub race: String,
    pub classes: Vec<ClassLevel>,
    pub feats: Vec<String>,
    pub base_abilities: AbilityScores,
}
```
Class levels:
```Rust
pub struct ClassLevel {
    pub class: String,
    pub level: u8,
    pub subclass: Option<String>,
}
```
Example character:
```
Race: elf
Class: fighter 3
Subclass: champion
Feat: alert
```
### 3. Derived Character Sheet

This is what the rule engine computes.
```Rust
use std::collections::HashSet;

pub struct CharacterSheet {
    pub abilities: AbilityScores,
    pub speed: u32,
    pub proficiencies: HashSet<String>,
    pub features: Vec<String>,
}
```
### 4. Features

Features are the core rule objects.

Examples:
```
Darkvision
Second Wind
Sneak Attack
Fighting Style
```
Rust struct:
```Rust
pub struct Feature {
    pub id: String,
    pub effects: Vec<Effect>,
}
```
### 5. Effects (Rule Engine Core)

Effects modify the derived sheet.
```Rust
pub enum Effect {
    AbilityBonus {
        ability: Ability,
        amount: i32,
    },

    SetSpeed {
        speed: u32,
    },

    GrantProficiency {
        proficiency: String,
    },

    GrantFeature {
        feature: String,
    },
}
```
Example effect:
```
Elf → +2 Dex
```
```Rust
Effect::AbilityBonus {
    ability: Ability::Dexterity,
    amount: 2
}
```
### 6. Rule Database

You need a central repository of rules.
```Rust
use std::collections::HashMap;

pub struct Rules {
    pub races: HashMap<String, Race>,
    pub classes: HashMap<String, Class>,
    pub features: HashMap<String, Feature>,
    pub feats: HashMap<String, Feat>,
}
```
Race:
```Rust
pub struct Race {
    pub id: String,
    pub features: Vec<String>,
}
```
Class:
```Rust
pub struct Class {
    pub id: String,
    pub level_features: HashMap<u8, Vec<String>>,
}
```
Feat:
```Rust
pub struct Feat {
    pub id: String,
    pub features: Vec<String>,
}
```
### 7. Feature Resolver

This gathers all features from the character.
```Rust
pub fn collect_features(character: &Character, rules: &Rules) -> Vec<String> {
    let mut features = Vec::new();

    // race features
    if let Some(race) = rules.races.get(&character.race) {
        features.extend(race.features.clone());
    }

    // class features
    for class_level in &character.classes {
        if let Some(class) = rules.classes.get(&class_level.class) {
            for level in 1..=class_level.level {
                if let Some(level_features) = class.level_features.get(&level) {
                    features.extend(level_features.clone());
                }
            }
        }
    }

    // feat features
    for feat_id in &character.feats {
        if let Some(feat) = rules.feats.get(feat_id) {
            features.extend(feat.features.clone());
        }
    }

    features
}
```

This produces a list like:
```
darkvision
keen_senses
second_wind
action_surge
alert_bonus
```
### 8. Effect Engine

Now apply the effects.
```Rust
pub fn apply_effects(
    base: &AbilityScores,
    features: &[String],
    rules: &Rules,
) -> CharacterSheet {

    let mut sheet = CharacterSheet {
        abilities: base.clone(),
        speed: 30,
        proficiencies: HashSet::new(),
        features: features.to_vec(),
    };

    for feature_id in features {
        if let Some(feature) = rules.features.get(feature_id) {
            for effect in &feature.effects {
                apply_effect(effect, &mut sheet);
            }
        }
    }

    sheet
}
```
### 9. Applying Individual Effects
```Rust
fn apply_effect(effect: &Effect, sheet: &mut CharacterSheet) {
    match effect {

        Effect::AbilityBonus { ability, amount } => {
            sheet.abilities.add(*ability, *amount);
        }

        Effect::SetSpeed { speed } => {
            sheet.speed = *speed;
        }

        Effect::GrantProficiency { proficiency } => {
            sheet.proficiencies.insert(proficiency.clone());
        }

        Effect::GrantFeature { feature } => {
            sheet.features.push(feature.clone());
        }
    }
}
```
### 10. Build Character Sheet

This is the top-level engine function.
```Rust
pub fn build_character_sheet(character: &Character, rules: &Rules) -> CharacterSheet {

    let features = collect_features(character, rules);

    apply_effects(&character.base_abilities, &features, rules)
}
```
Done.

Every time something changes, call this again.

### 11. Why This Architecture Works
1. Changing race is trivial

Just update:
```
character.race
```
Then rebuild.

2. Multiclassing works naturally
```
Fighter 3
Rogue 2
```
Feature resolver handles both.

3. Homebrew is easy

Users add:
```
new race
new feature
new effect
```
No engine changes.

4. Editing characters is safe

You never need to undo mutations.

Just rebuild the sheet.

### 12. Example Feature Data

Example JSON feature:
```JSON
{
  "id": "elf_dex_bonus",
  "effects": [
    {
      "type": "AbilityBonus",
      "ability": "Dexterity",
      "amount": 2
    }
  ]
}
```
Loaded using serde.

### 13. Performance

Even large builds evaluate in microseconds because:

feature counts are small

rule evaluation is linear

### 14. When You Might Expand This

Later you may add:
```
requirement system
choice system
spellcasting engine
condition system
```
But the core architecture remains unchanged.

✅ If you'd like, I can also show you the best way to structure the Rust project itself (modules + crates) so the CLI and future GUI both reuse the rule engine cleanly. This is another place many projects accidentally create huge refactoring later.

# ***How does this change with a spellcasting system as well as tracking resource usage such as current HP, spent hit dice, used spell slots, and so on?***

Once you add spellcasting and mutable resources, the architecture should split the character model into two layers:

1. Character Definition (permanent build choices)
2. Character State (current play session state)

Your rule engine still derives the sheet, but resources like HP and spell slots must live in runtime state.

This prevents corruption of the character definition while still supporting gameplay tracking.

### 1. The Three Character Layers

A clean architecture ends up like this:
```
CharacterDefinition
      ↓
Rule Engine
      ↓
CharacterSheet (derived stats)
      ↓
CharacterState (runtime resources)
```
Each layer has a different responsibility.

### 2. CharacterDefinition (Build Choices)

This remains mostly unchanged.
```Rust
pub struct CharacterDefinition {
    pub name: String,

    pub race: String,
    pub classes: Vec<ClassLevel>,
    pub feats: Vec<String>,

    pub base_abilities: AbilityScores,
}
```
This represents:
```
race
classes
subclass
feats
ability scores
background
```
It never stores HP, slots, etc.

3. CharacterSheet (Derived Rules)

The rule engine computes a sheet from the definition.
```Rust
pub struct CharacterSheet {
    pub abilities: AbilityScores,
    pub proficiency_bonus: i32,

    pub max_hp: i32,
    pub hit_dice: Vec<HitDie>,

    pub speed: u32,

    pub spellcasting: Option<Spellcasting>,

    pub features: Vec<String>,
}
```
Example derived data:
```
ability modifiers
proficiency bonus
max HP
hit dice pool
spell slots per level
prepared spell limit
```
This is recomputable at any time.

### 4. CharacterState (Runtime Resources)

This tracks current gameplay state.
```Rust
pub struct CharacterState {
    pub current_hp: i32,

    pub spent_hit_dice: Vec<HitDie>,

    pub spell_slots_used: HashMap<u8, u8>,

    pub conditions: HashSet<String>,
}
```
Examples:
```
current HP
used spell slots
expended hit dice
temporary effects
conditions
```
This layer is mutable.

### 5. Why the Separation Matters

If you store runtime data in the character definition, things break.

Example:
```
level up
max HP changes
spell slots change
```
If you mix state + rules, you must migrate data constantly.

Separating them gives you:
```
definition = stable
sheet = derived
state = mutable
```
### 6. Spellcasting Model

Spellcasting should be derived from the Spellcasting feature.

Example:
```Rust
pub struct Spellcasting {
    pub ability: Ability,

    pub slots: SpellSlotTable,

    pub prepared_spells: usize,

    pub spell_list: String,
}
```
Spell slot table:
```Rust
pub struct SpellSlotTable {
    pub slots: HashMap<u8, u8>,
}
```
Example:
```
level 1 → 4 slots
level 2 → 3 slots
level 3 → 2 slots
```
### 7. Runtime Spell Slot Usage

Spell slots used go in CharacterState.
```Rust
pub struct CharacterState {
    pub spell_slots_used: HashMap<u8, u8>,
}
```
Example:
```
Level 1 slots used: 2
Level 2 slots used: 1
```
Available slots are computed:
```Rust
fn remaining_slots(sheet: &CharacterSheet, state: &CharacterState, level: u8) -> u8 {
    sheet.spellcasting
        .as_ref()
        .unwrap()
        .slots
        .slots[&level] - state.spell_slots_used.get(&level).copied().unwrap_or(0)
}
```
### 8. HP System

HP should work similarly.

Derived max HP:
```Rust
pub struct CharacterSheet {
    pub max_hp: i32,
}
```
Runtime HP:
```Rust
pub struct CharacterState {
    pub current_hp: i32,
}
```
Damage application:
```Rust
fn apply_damage(state: &mut CharacterState, amount: i32) {
    state.current_hp -= amount;
}
```
### 9. Hit Dice Tracking

Derived from classes:
```Rust
pub struct HitDie {
    pub die: u8,
}
```
Example:
```
Fighter 3 / Rogue 2
```
Hit dice:
```
d10, d10, d10, d8, d8
```
Runtime tracking:
```Rust
pub struct CharacterState {
    pub spent_hit_dice: Vec<HitDie>,
}
```
### 10. Long Rest / Short Rest System

State layer handles rests.

Short rest:
```Rust
fn short_rest(state: &mut CharacterState) {
    state.spent_hit_dice.clear();
}
```
Long rest:
```Rust
fn long_rest(sheet: &CharacterSheet, state: &mut CharacterState) {
    state.current_hp = sheet.max_hp;
    state.spell_slots_used.clear();
    state.spent_hit_dice.clear();
}
```
### 11. Spell Lists and Known Spells

Spell definitions live in rules.
```Rust
pub struct Spell {
    pub id: String,
    pub level: u8,
    pub school: String,
}
```

Prepared spells belong to the state.
```Rust
pub struct CharacterState {
    pub prepared_spells: Vec<String>,
}
```

For known-spell casters (Sorcerer, Bard), store them in definition.

### 12. Final Character Container

Your application will typically wrap everything.
```Rust
pub struct Character {
    pub definition: CharacterDefinition,
    pub state: CharacterState,
}
```
When needed:
```Rust
let sheet = build_character_sheet(&character.definition, rules);
```
### 13. Save Format

Persist both parts.

Example JSON:
```JSON
{
  "definition": { ... },
  "state": {
    "current_hp": 21,
    "spell_slots_used": { "1": 2 }
  }
}
```
14. CLI Interaction Example

Example commands:
```
> damage 8
> cast magic_missile
> short_rest
> long_rest
```
These modify CharacterState only.

### 15. Important Design Rule

Your system should follow this rule:

> Rules derive capabilities. State tracks usage.

Capabilities:
```
max HP
spell slots
features
proficiencies
```
Usage:
```
current HP
used slots
expended dice
conditions
```
### 16. Resulting Architecture

Your full system becomes:
```
Rules Database
        ↓
CharacterDefinition
        ↓
Rule Engine
        ↓
CharacterSheet
        ↓
CharacterState
```
This scales well to:
```
spellcasting
conditions
combat tracking
buffs/debuffs
magic items
```
✅ If you'd like, I can also show you a very clean Rust way to implement spell slot progression and multiclass spellcasting (which is one of the trickiest parts of the entire 5e ruleset).

# ***why are fields of structs being made public?***

In Rust, struct fields are made `pub` primarily for ergonomics and module boundaries, but whether they should be public depends on the role of the type in your architecture.

Short answer:
I made them `pub` in the examples because they were data containers, not invariant-enforcing types. But in a real system you usually make far fewer fields public.

Let's break down the tradeoffs.

### 1. Rust Privacy Basics

By default everything in Rust is private to the module.
```Rust
struct CharacterState {
    current_hp: i32
}
```
Only the same module can access `current_hp`.

To allow access from other modules:
```Rust
pub struct CharacterState {
    pub current_hp: i32
}
```
Now both the struct and the field are visible.

### 2. When Public Fields Are Good

Public fields are good for plain data structures.

These are often called "data structs" or DTOs (data transfer objects).

Example:
```Rust
pub struct AbilityScores {
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wis: i32,
    pub cha: i32,
}
```
This works well because:

- no invariants
- no complex logic
- simple serialization

You don't gain much by hiding them.

### 3. When Fields Should NOT Be Public

Fields should be private when invalid states must be prevented.

Example: HP.

Bad:
```Rust
pub struct CharacterState {
    pub current_hp: i32
}
```
Nothing stops:
```Rust
state.current_hp = -500;
```

Better:
```Rust
pub struct CharacterState {
    current_hp: i32
}
```
Expose safe methods:
```Rust
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
```
Now you enforce:

- no negative HP
- no overhealing

### 4. Good Rule of Thumb

A common Rust design rule:
```
Public fields for passive data
Private fields for stateful logic
```
For your system that usually becomes:

Public Fields

Things like:
```
AbilityScores
ClassLevel
Spell
Race
Feature definitions
```
These are rule data, not mutable state.

Private Fields

Things like:
```
current_hp
spell_slots_used
conditions
hit_dice_spent
```
Because gameplay logic modifies them.

### 5. Example "Production" Version

A cleaner version of the earlier struct might look like:
```Rust
pub struct CharacterState {
    current_hp: i32,
    spell_slots_used: HashMap<u8, u8>,
}
```
Public API:
```Rust
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
Now the rest of your program cannot corrupt state accidentally.

### 6. Why Tutorials Often Use `pub`

You'll see pub everywhere in examples because it avoids:

- module explanations
- getters/setters
- visibility discussions

It keeps the example short.

But most mature Rust codebases use much stricter encapsulation.

### 7. For Your D&D Engine

A good split is:

Public (rule data)
```
Spell
Feature
Class
Race
AbilityScores
```
Private (gameplay state)
```
current_hp
hit_dice_spent
spell_slots_used
conditions
```
Expose behavior through methods like:
```
take_damage()
heal()
cast_spell()
short_rest()tions

- getters/setters

- visibility disc
long_rest()
```
✅ If you'd like, I can also show a very Rust-idiomatic architecture for a 5e rules engine (traits + rule composition) that avoids the giant if class == wizard style logic most implementations fall into.