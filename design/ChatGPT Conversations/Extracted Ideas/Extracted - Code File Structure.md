# Links

### Part 1
- [[Part 1 Raw#2. Suggested Rust Project Structure|Suggested Rust Project Structure]]
- [[Part 1 Raw#15. Recommended Architecture for Your Case|Another Structure]]
- [[Part 1 Raw#14. Ideal Project Structure|"Ideal Project Structure"]]

### Part 2
- [[Part 2 Raw#2. Rust Workspace Layout|Rust Workspace Layout]]
- [[Part 2 Raw#11. Suggested Module Layout|Suggested Module Layout]]
- [[Part 2 Raw#1. Top-Level Crate Layout|Top-Level Crate Layout]]

### Part 4
- [[Part 4 Raw#1. Rust Project File Structure|Project File Structure]]

### Part 5
- [[Part 5 Raw#1. The Core Architecture We Built|"Core Architecture"]]
- [[Part 5 Raw#20. Final Architecture|"Final Architecture"]]

### Part 6
- [[Part 6 Raw#2. Code File Structure|Code File Structure]]

### Part 7
- [[Part 7 Raw#Top-Level Project Structure|Project Crate Structure]]

# Commonalities

### Crate Structure
- `engine/`
	- typically stuff like `rule_graph.rs` and evaluation
	- may contain the `RuleEffect` type stuff as a subdirectory
- `character/`
	- character modeling like ability scores, class levels, class features
- `rules/`
	- sometimes this contains stuff like `races.rs`, `saving_throws.rs`, and `feats.rs`
	- other times it contains `rule_effect.rs`, `rule_phase.rs`, `modifier.rs`, and `condition.rs`
- `inventory/`
	- inventory specifically is probably separated out because a substantial amount of conversation was spent on it
	- sometimes goes under the `character/` crate
- `data/ or loader/ or `storage/`
	- loads from YAML or DB and turns it into internal data structures
	- can also handle module level stuff (e.g., TCoE depends on PHB)
	- contains `races.rs` and `feats.rs` if `rules/` doesn't
	- deals with persistence layer (both loading and writing)
- `cli/`
- `gui/`

### Engine
- rule graph
	- `rule_graph.rs`
	- `node.rs`
	- `fragment.rs`
	- `context.rs`
	- `evaluator.rs`
	- `stacking.rs`
- rule effects
	- `rule_effect.rs`
	- `modifier_engine.rs`
	- `rule_condition.rs`
	- `rule_phases.rs`
- stats?
	- `stat_id.rs`

### Character
- general character sheet modeling plus tracking active effects and so on
	- `character.rs`
	- `ability_scores.rs`
	- `class_levels.rs`
	- `conditions.rs`
	- etc
- can contain inventory or item information, tho item probably belongs under `rules/`
- track used resources, prepared spells, and other character building choices like 2024 wildshape picks

### Rules
- storing loaded rules data
	- `class.rs`, `feats.rs`, and so on
- resolving actual game rules directly
	- `skills.rs`, `saving_throws.rs`, `armor_class.rs`, and so on
- rules engine
	- `rule_graph.rs`, `rule_node.rs`, etc
	- includes `rule_effect.rs`
- subengine for rules used by the engine crate
	- `rule.rs`, `rule_effect.rs`, `rule_filter.rs`,  `choice_set.rs`, `rule_conditions.rs`, `rule_phase.rs`, and so on

### Inventory
- `inventory.rs`, `item_instance.rs`, `item_location.rs`, and a `containers.rs` or similar
- should probably be a submodule of `character/`

### Data or Loader or Storage
- `loader.rs` and `schemas.rs`
	- `compiler.rs` for interpreting YAML expressions or whatever
	- `pack_loader.rs` for loading full datapacks
- `storage/` includes `sqlite.rs` and `yaml.rs`
- sometimes (especially the `data/` variant) actually stores the loaded rules data structures (e.g., `races.rs`) leaving rules to do stuff like `saving_throws.rs` and `armor_class.rs`
- **Database Crate (Part 7)** 
	- `schema/` with stuff like `characters.sql`
	- `models/` with stuff like `character_row.rs`
	- `queries/` with stuff like `load_character.rs`
- **Loader Crate (Part 7)**
	- `modules/` for `module_graph.rs` and such
	- `yaml/` for `raw_spell.rs` and such
	- `converters/` for stuff like `spell_loader.rs`
	- `pipeline/` for `load_modules.rs`, `build_registry.rs`, etc

### CLI
- `commands/` 
	- `create_character.rs`, `view_character.rs`, and similar
- `display/`
	- `character_sheet.rs`
- alternatively a `menu.rs` and `prompts.rs`

### GUI
- `state.rs`
- `views/`
	- seems redundant with files in the `character/` crate but here perhaps storing the UI information needed

# Structures (Reverse Chronological)

## Part 7

### #1

#### Crate Structure
```
dnd-engine/  
│  
├─ Cargo.toml  
├─ README.md  
│  
├─ crates/  
│  
│  ├─ engine/  
│  ├─ rules/  
│  ├─ content/  
│  ├─ loader/  
│  ├─ database/  
│  ├─ character/  
│  ├─ cli/  
│  └─ gui/  
│  
├─ modules/  
│  
│  ├─ phb/  
│  ├─ xgte/  
│  ├─ tcoe/  
│  └─ homebrew/  
│  
└─ examples/
```

#### Engine Crate
```
crates/engine/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ rules/  
│  ├─ rule.rs  
│  ├─ rule_effect.rs  
│  ├─ rule_condition.rs  
│  ├─ rule_phase.rs  
│  
├─ evaluation/  
│  ├─ rule_graph.rs  
│  ├─ resolver.rs  
│  ├─ stacking.rs  
│  
├─ stats/  
│  ├─ stat_id.rs  
│  ├─ stat_value.rs  
│  
└─ context/  
   ├─ character_context.rs  
   ├─ rule_context.rs
```

#### Content Crate
```
crates/content/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ ids/  
│  ├─ content_id.rs  
│  ├─ content_key.rs  
│  ├─ module_id.rs  
│  
├─ registry/  
│  ├─ content_registry.rs  
│  
├─ types/  
│  ├─ spell.rs  
│  ├─ feat.rs  
│  ├─ item.rs  
│  ├─ class.rs  
│  ├─ subclass.rs  
│  ├─ species.rs  
│  ├─ background.rs  
│  ├─ creature.rs  
│  
└─ rules/  
   ├─ rule_definition.rs
```

#### Rules Crate
```
crates/rules/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ rules/  
│  ├─ rule.rs  
│  ├─ rule_effect.rs  
│  ├─ rule_condition.rs  
│  
├─ effects/  
│  ├─ modifier.rs  
│  ├─ grant_feature.rs  
│  ├─ override.rs  
│  
├─ choices/  
│  ├─ choice_set.rs  
│  ├─ choice_value.rs
```

#### Loader Crate
```
crates/loader/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ modules/  
│  ├─ manifest.rs  
│  ├─ module_graph.rs  
│  ├─ module_registry.rs  
│  
├─ yaml/  
│  ├─ raw_spell.rs  
│  ├─ raw_feat.rs  
│  ├─ raw_item.rs  
│  
├─ converters/  
│  ├─ spell_loader.rs  
│  ├─ feat_loader.rs  
│  ├─ item_loader.rs  
│  
└─ pipeline/  
   ├─ load_modules.rs  
   ├─ resolve_dependencies.rs  
   ├─ build_registry.rs
```

#### Character Crate
```
crates/character/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ character/  
│  ├─ character.rs  
│  ├─ class_levels.rs  
│  
├─ choices/  
│  ├─ character_choices.rs  
│  
├─ resources/  
│  ├─ resource_pool.rs  
│  
├─ inventory/  
│  ├─ inventory.rs  
│  ├─ item_instance.rs  
│  ├─ item_location.rs  
│  
└─ conditions/  
   ├─ active_condition.rs
```

#### Database Crate
```
crates/database/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ schema/  
│  ├─ characters.sql  
│  ├─ inventory.sql  
│  ├─ resources.sql  
│  
├─ models/  
│  ├─ character_row.rs  
│  ├─ inventory_row.rs  
│  
└─ queries/  
   ├─ load_character.rs  
   ├─ save_character.rs
```

#### CLI Crate
```
crates/cli/  
│  
├─ src/  
│  
├─ main.rs  
│  
├─ commands/  
│  ├─ create_character.rs  
│  ├─ view_character.rs  
│  ├─ list_spells.rs  
│  
└─ display/  
   ├─ character_sheet.rs
```

#### GUI Crate
```
crates/gui/  
│  
├─ src/  
│  
├─ main.rs  
│  
├─ app/  
│  ├─ state.rs  
│  
├─ views/  
│  ├─ character_sheet.rs  
│  ├─ inventory.rs  
│  ├─ spellbook.rs  
│  
└─ widgets/  
   ├─ choice_selector.rs
```

#### Module Content Directory
```
modules/  
│  
├─ phb/  
│  ├─ module.yaml  
│  
│  ├─ spells/  
│  │  ├─ fireball.yaml  
│  │  └─ shield.yaml  
│  
│  ├─ classes/  
│  │  └─ fighter.yaml  
│  
│  ├─ feats/  
│  │  └─ sharpshooter.yaml  
│  
│  └─ items/  
│     └─ longsword.yaml  
│  
├─ xgte/  
│  
└─ homebrew/
```

## Part 6

### File Structure
```
src/  
│  
├── main.rs  
│  
├── app/  
│   ├── cli.rs  
│   └── gui.rs  
│  
├── engine/  
│   ├── rule_engine.rs  
│   ├── rule_index.rs  
│   ├── rule_phases.rs  
│   ├── modifier_engine.rs  
│   ├── condition_engine.rs  
│   ├── spellcasting_engine.rs  
│   └── combat_engine.rs  
│  
├── character/  
│   ├── character.rs  
│   ├── character_builder.rs  
│   ├── character_choices.rs  
│   ├── resources.rs  
│   ├── inventory.rs  
│   ├── spellbook.rs  
│   ├── wildshape.rs  
│   └── cache.rs  
│  
├── rules/  
│   ├── ruleset.rs  
│   ├── rule.rs  
│   ├── rule_filter.rs  
│   ├── rule_effect.rs  
│   ├── rule_choice.rs  
│   ├── modifiers.rs  
│   ├── conditions.rs  
│   └── rule_phase.rs  
│  
├── data/  
│   ├── loader.rs  
│   ├── compiler.rs  
│   ├── index.rs  
│   └── pack_loader.rs  
│  
├── model/  
│   ├── ids.rs  
│   ├── stats.rs  
│   ├── actions.rs  
│   ├── damage.rs  
│   └── resources.rs  
│  
├── content/  
│   ├── classes.rs  
│   ├── subclasses.rs  
│   ├── species.rs  
│   ├── backgrounds.rs  
│   ├── feats.rs  
│   ├── spells.rs  
│   ├── items.rs  
│   ├── creatures.rs  
│   └── weapon_masteries.rs  
│  
└── storage/  
    ├── sqlite.rs  
    ├── yaml.rs  
    └── character_store.rs
```

See the relevant section for various data types within this architecture.

## Part 5

### #1
```
rules/  
    classes  
    species  
    spells  
    feats  
    items  
    forms  
  
engine/  
    rule_engine  
    rule_graph  
    stacking_resolver  
    proficiency_resolver  
  
character/  
    character  
    inventory  
    resources  
    choices  
  
sheet/  
    derived_stats  
    cached_character_sheet  
  
interfaces/  
    cli  
    gui
```

### #2
```
rules_data/  
   classes.yaml  
   subclasses.yaml  
   species.yaml  
   feats.yaml  
   spells.yaml  
   items.yaml  
   features.yaml  
   conditions.yaml  
   forms.yaml  
  
engine/  
   rule_engine  
   rule_graph  
   stacking_resolver  
  
character/  
   character  
   inventory  
   choices  
  
sheet/  
   derived_stats  
   cached_sheet  
  
interfaces/  
   cli  
   gui  
   web
```

## Part 4

### #1
```
src/  
│  
├─ main.rs  
├─ lib.rs  
│  
├─ engine/  
│  ├─ mod.rs  
│  ├─ engine.rs  
│  ├─ rule_graph.rs  
│  ├─ rule_effect.rs  
│  ├─ stacking.rs  
│  
├─ character/  
│  ├─ mod.rs  
│  ├─ character.rs  
│  ├─ ability_scores.rs  
│  ├─ conditions.rs  
│  
├─ inventory/  
│  ├─ mod.rs  
│  ├─ inventory.rs  
│  ├─ item_instance.rs  
│  ├─ item_location.rs  
│  
├─ rules/  
│  ├─ mod.rs  
│  ├─ database.rs  
│  ├─ race.rs  
│  ├─ class.rs  
│  ├─ feat.rs  
│  ├─ spell.rs  
│  ├─ item.rs  
│  
├─ stats/  
│  ├─ mod.rs  
│  ├─ stat.rs  
│  ├─ derived.rs  
│  
├─ systems/  
│  ├─ mod.rs  
│  ├─ rule_collector.rs  
│  ├─ spell_system.rs  
│  ├─ inventory_system.rs  
│  
└─ storage/  
   ├─ mod.rs  
   ├─ loader.rs  
   ├─ persistence.rs
```

## Part 2

### #1
```
workspace/  
  
dnd_core/  
    character/  
    abilities/  
    features/  
    effects/  
    spells/  
    engine/  
  
dnd_data/  
    loader.rs  
    schemas.rs  
  
dnd_rules/  
    races/  
    classes/  
    feats/  
    spells/  
  
dnd_cli/  
    main.rs
```

### #2
```
domain/  
   character.rs  
   ability.rs  
  
   inventory/  
       mod.rs  
       inventory.rs  
       item_instance.rs  
       item_location.rs  
  
   items/  
       mod.rs  
       item_definition.rs  
       weapon.rs  
       armor.rs  
       magic_item.rs  
       container.rs  
  
rules/  
   engine.rs  
   graph.rs  
   node.rs  
   rule_effect.rs  
  
data/  
   item_loader.rs
```

### #3
```
dnd_rules/  
│  
├─ engine/        # generic rule graph engine  
│  
├─ rules/         # D&D 5e rule definitions  
│  
├─ character/     # character domain model  
│  
├─ inventory/     # items, equipment, containers  
│  
├─ actions/       # contextual rule evaluation  
│  
├─ data/          # game data loaders  
│  
└─ types/         # shared enums and IDs
```

#### engine/
```
engine/  
├─ graph.rs  
├─ node.rs  
├─ value.rs  
├─ stack.rs  
├─ fragment.rs  
├─ evaluator.rs  
└─ context.rs
```

#### rules/
```
rules/  
├─ abilities.rs  
├─ combat.rs  
├─ armor_class.rs  
├─ skills.rs  
├─ saving_throws.rs  
└─ spellcasting.rs
```

#### character/
```
character/  
├─ character.rs  
├─ ability_scores.rs  
├─ class_levels.rs  
├─ features.rs  
└─ conditions.rs
```

#### inventory/
```
inventory/  
├─ inventory.rs  
├─ item_instance.rs  
├─ equipment.rs  
└─ containers.rs
```

#### actions/
```
actions/  
├─ attack.rs  
├─ damage.rs  
├─ skill_check.rs  
├─ saving_throw.rs  
└─ spell_cast.rs
```

#### data/
```
data/  
├─ races.rs  
├─ classes.rs  
├─ feats.rs  
├─ spells.rs  
└─ items.rs
```

#### types/
```
N/A
```

## Part 1

### #1
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

### #2
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

### #3
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