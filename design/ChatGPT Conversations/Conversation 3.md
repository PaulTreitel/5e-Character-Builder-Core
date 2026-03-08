Below is a **Rust-idiomatic architecture for a D&D 5e rules engine** that builds on the ideas in your previous discussion but restructures them to better match common Rust design patterns:

- **strong typing for 5e concepts**
    
- **data-driven rules**
    
- **derived character sheets**
    
- **separation of immutable rules vs mutable gameplay state**
    
- **trait-based rule composition instead of giant match/if trees**
    

The key Rust idea is that **rules become composable behaviors** rather than hardcoded logic tied to classes or races.

---

# 1. High-Level Architecture

A clean Rust 5e engine usually ends up with **five layers**:

Content (JSON/YAML)  
      ↓  
Rule Database  
      ↓  
Feature Resolver  
      ↓  
Effect Engine  
      ↓  
Derived Character Sheet  
      ↓  
Runtime Character State

So the runtime flow becomes:

CharacterDefinition  
      ↓  
collect features  
      ↓  
apply effects  
      ↓  
CharacterSheet  
      ↓  
CharacterState (HP, slots, etc)

---

# 2. Rust Workspace Layout

A Rust workspace works extremely well for this.

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

### Responsibility

**dnd_core**

- pure rules engine
    
- no IO
    
- no CLI
    

**dnd_data**

- serde loaders
    
- validation
    

**dnd_rules**

- JSON rule files
    

**dnd_cli**

- interactive interface
    

Later:

dnd_gui/

---

# 3. Core Domain Types (Strongly Typed 5e)

Unlike a generic RPG engine, you **should encode 5e structure directly**.

pub enum Ability {  
    Strength,  
    Dexterity,  
    Constitution,  
    Intelligence,  
    Wisdom,  
    Charisma,  
}

Ability scores:

pub struct AbilityScores {  
    pub str: i32,  
    pub dex: i32,  
    pub con: i32,  
    pub int: i32,  
    pub wis: i32,  
    pub cha: i32,  
}

Character build choices:

pub struct CharacterDefinition {  
    pub race: RaceId,  
    pub classes: Vec<ClassLevel>,  
    pub feats: Vec<FeatId>,  
    pub base_abilities: AbilityScores,  
}

Class level:

pub struct ClassLevel {  
    pub class: ClassId,  
    pub level: u8,  
    pub subclass: Option<SubclassId>,  
}

Important:

**This struct contains only choices.**

---

# 4. Derived Character Sheet

The rule engine produces this.

pub struct CharacterSheet {  
    pub abilities: AbilityScores,  
    pub proficiency_bonus: i32,  
  
    pub max_hp: i32,  
    pub speed: u32,  
  
    pub proficiencies: HashSet<Proficiency>,  
    pub features: Vec<FeatureId>,  
  
    pub spellcasting: Option<Spellcasting>,  
}

Everything here is **derived**.

Never saved.

---

# 5. Runtime State (Mutable Gameplay Data)

This tracks **usage during play**.

pub struct CharacterState {  
    current_hp: i32,  
    spell_slots_used: HashMap<u8, u8>,  
    spent_hit_dice: Vec<HitDie>,  
}

Example methods:

impl CharacterState {  
  
    pub fn apply_damage(&mut self, dmg: i32) {  
        self.current_hp -= dmg;  
    }  
  
    pub fn spend_spell_slot(&mut self, level: u8) {  
        *self.spell_slots_used.entry(level).or_insert(0) += 1;  
    }  
  
}

---

# 6. Features (The Core Rule Unit)

Almost every rule in 5e can be expressed as a **feature**.

Examples:

- Darkvision
    
- Sneak Attack
    
- Rage
    
- Fighting Style
    
- Spellcasting
    

pub struct Feature {  
    pub id: FeatureId,  
    pub effects: Vec<Effect>,  
}

---

# 7. Effect System (Rule Engine Core)

Effects modify the **derived sheet**.

pub enum Effect {  
  
    AbilityBonus {  
        ability: Ability,  
        amount: i32  
    },  
  
    SetSpeed {  
        speed: u32  
    },  
  
    GrantProficiency {  
        proficiency: Proficiency  
    },  
  
    GrantFeature {  
        feature: FeatureId  
    },  
  
    EnableSpellcasting {  
        ability: Ability,  
        spell_list: SpellListId  
    }  
}

These are **data-driven**.

Homebrew authors add effects via JSON.

---

# 8. Rule Database

All rules load into a repository.

pub struct Rules {  
  
    pub races: HashMap<RaceId, Race>,  
    pub classes: HashMap<ClassId, Class>,  
    pub feats: HashMap<FeatId, Feat>,  
  
    pub features: HashMap<FeatureId, Feature>,  
    pub spells: HashMap<SpellId, Spell>,  
}

Example race:

pub struct Race {  
    pub id: RaceId,  
    pub speed: u32,  
    pub features: Vec<FeatureId>,  
}

Example class:

pub struct Class {  
    pub id: ClassId,  
    pub hit_die: u8,  
    pub level_features: HashMap<u8, Vec<FeatureId>>,  
}

---

# 9. Feature Resolution

Collect all features from sources.

pub fn collect_features(  
    character: &CharacterDefinition,  
    rules: &Rules,  
) -> Vec<FeatureId> {  
  
    let mut features = Vec::new();  
  
    let race = &rules.races[&character.race];  
    features.extend(race.features.clone());  
  
    for class_level in &character.classes {  
  
        let class = &rules.classes[&class_level.class];  
  
        for lvl in 1..=class_level.level {  
            if let Some(fs) = class.level_features.get(&lvl) {  
                features.extend(fs.clone());  
            }  
        }  
    }  
  
    for feat in &character.feats {  
        features.extend(rules.feats[feat].features.clone());  
    }  
  
    features  
}

---

# 10. Effect Engine

Apply effects to produce the sheet.

pub fn apply_effects(  
    base: &AbilityScores,  
    features: &[FeatureId],  
    rules: &Rules  
) -> CharacterSheet {  
  
    let mut sheet = CharacterSheet {  
        abilities: base.clone(),  
        proficiency_bonus: 2,  
        max_hp: 0,  
        speed: 30,  
        proficiencies: HashSet::new(),  
        features: features.to_vec(),  
        spellcasting: None  
    };  
  
    for feature_id in features {  
  
        let feature = &rules.features[feature_id];  
  
        for effect in &feature.effects {  
            apply_effect(effect, &mut sheet);  
        }  
    }  
  
    sheet  
}

---

# 11. Applying Effects

fn apply_effect(effect: &Effect, sheet: &mut CharacterSheet) {  
  
    match effect {  
  
        Effect::AbilityBonus { ability, amount } => {  
            sheet.abilities.add(*ability, *amount);  
        }  
  
        Effect::SetSpeed { speed } => {  
            sheet.speed = *speed;  
        }  
  
        Effect::GrantProficiency { proficiency } => {  
            sheet.proficiencies.insert(*proficiency);  
        }  
  
        Effect::EnableSpellcasting { ability, spell_list } => {  
  
            sheet.spellcasting = Some(Spellcasting {  
                ability: *ability,  
                spell_list: *spell_list,  
                slots: SpellSlotTable::default(),  
            });  
        }  
  
        _ => {}  
    }  
}

---

# 12. Spellcasting Model

pub struct Spellcasting {  
    pub ability: Ability,  
    pub spell_list: SpellListId,  
    pub slots: SpellSlotTable,  
}

Slot table:

pub struct SpellSlotTable {  
    pub slots: HashMap<u8, u8>  
}

Example:

1 → 4  
2 → 3  
3 → 2

---

# 13. Multiclass Spellcasting (Important)

This should be implemented as a **separate system**, not inside effects.

1. Determine caster level  
2. Look up multiclass slot table  
3. Assign slot progression

Example:

Wizard 3 + Cleric 2  
→ caster level 5  
→ use level 5 slot table

This lives in:

engine/spellcasting.rs

---

# 14. Rule Phases (Very Rust-Friendly)

A very clean pattern is **phased rule evaluation**.

Phase 1: ability bonuses  
Phase 2: derived modifiers  
Phase 3: proficiencies  
Phase 4: HP  
Phase 5: spellcasting

Example:

pub enum RulePhase {  
    Abilities,  
    Proficiencies,  
    DerivedStats,  
    Spellcasting,  
}

Effects specify phase:

pub struct Feature {  
    pub effects: Vec<(RulePhase, Effect)>  
}

Engine processes phases in order.

This avoids rule order bugs.

---

# 15. Final Engine Entry Point

pub fn build_character_sheet(  
    character: &CharacterDefinition,  
    rules: &Rules,  
) -> CharacterSheet {  
  
    let features = collect_features(character, rules);  
  
    apply_effects(&character.base_abilities, &features, rules)  
}

---

# 16. Why This Architecture Is Rust-Idiomatic

This design uses several patterns common in Rust systems:

### 1️⃣ Immutable source data

CharacterDefinition

### 2️⃣ Derived state

CharacterSheet

### 3️⃣ Explicit mutation layer

CharacterState

### 4️⃣ Trait-free data driven rules

Effects stored in data.

### 5️⃣ No hidden side effects

Everything flows through:

collect_features → apply_effects

---

# 17. Why This Scales to Heavy Homebrew

Homebrew creators can add:

- races
    
- subclasses
    
- feats
    
- features
    
- spells
    

They only need to define:

features  
effects

No Rust changes required.

---

# 18. The One Pattern That Makes the Engine Maintainable

The entire system revolves around:

Feature → Effects → Sheet

Not:

Race → Class → Feat → Spell → Ability → ...

That single design decision keeps the engine **small, composable, and Rust-idiomatic**.

---

✅ If you'd like, I can also show the **next step most engines evolve into**:

**a dependency-graph rule engine (feature DAG)**

This solves tricky rules like:

- Extra Attack stacking
    
- Fighting style choices
    
- conditional features
    
- multiclass feature overrides
    

…and it maps _extremely well_ to Rust’s ownership model.

Explain why "Trait-free data driven rules" is valuable here, then explain how a dependency graph rule engine works and how to use it.

I'll break this into two parts because they're closely related but solve **different architectural problems**:

1. **Why trait-free data-driven rules are valuable**
    
2. **How a dependency-graph rule engine works and how to implement one**
    

The first reduces **code complexity and improves modding/homebrew**.  
The second solves **rule ordering and interactions**.

---

# 1. Why “Trait-Free Data-Driven Rules” Are Valuable

Rust developers often instinctively reach for **traits and polymorphism** when designing rule systems.

Example instinct:

trait Rule {  
    fn apply(&self, sheet: &mut CharacterSheet);  
}

Then create many implementations:

AbilityBonusRule  
SpeedRule  
SpellcastingRule  
SneakAttackRule  
DarkvisionRule

This looks elegant but becomes problematic for a **large RPG ruleset**.

---

## The Trait-Based Rule System Problem

With traits, every new rule requires **new Rust code**.

Example:

homebrew feature:  
"Frostblood"  
  
effects:  
- cold resistance  
- +1 constitution

Trait system requires writing:

ColdResistanceRule  
AbilityBonusRule

…and recompiling.

That creates problems:

### Problem 1 — Homebrew requires code

You can't add new content through JSON.

Every rule requires Rust implementation.

Bad for modding.

---

### Problem 2 — Too many rule types

5e has hundreds of rule behaviors:

grant proficiency  
grant feature  
modify stat  
add damage  
replace attack action  
conditional advantage  
extra attack  
spellcasting  
resistances  
immunities  
etc

A trait system leads to **dozens or hundreds of types**.

---

### Problem 3 — Hard to serialize

Traits are difficult to represent in JSON.

Example problem:

{  
  "rule": ????  
}

Rust cannot deserialize arbitrary trait objects easily.

---

### Problem 4 — Dynamic composition becomes difficult

Example feature:

Dragonborn breath weapon

Rules:

damage = 2d6  
save DC = 8 + CON + proficiency

You want rules like:

{ "effect": "grant_action", "action": "breath_weapon" }  
{ "effect": "damage_resistance", "type": "fire" }

Traits force more code.

---

# Why Data-Driven Rules Are Better

Instead of traits, define **a finite effect language**.

Example:

pub enum Effect {  
    AbilityBonus { ability: Ability, amount: i32 },  
    GrantProficiency { proficiency: Proficiency },  
    SetSpeed { speed: u32 },  
    GrantFeature { feature: FeatureId },  
}

Rules become **data**.

Example JSON:

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

The engine interprets the effect.

---

## Benefits

### 1️⃣ Homebrew without recompiling

Users can write:

features.json

and the engine loads them.

---

### 2️⃣ Smaller engine

Instead of:

150 rule types

You implement:

20 effect types

Most rules become **combinations of effects**.

---

### 3️⃣ Easy serialization

Serde handles enums perfectly.

---

### 4️⃣ Predictable evaluation

Effects are plain data.

No hidden logic.

---

### 5️⃣ Easier debugging

You can print rule execution:

Applying effect: AbilityBonus(Dex +2)  
Applying effect: GrantProficiency(Perception)

---

## When Traits ARE Useful

Traits are still valuable for **engine subsystems**, not individual rules.

Example:

trait SpellcastingSystem  
trait CombatSystem  
trait ConditionSystem

But **rules themselves remain data**.

---

# 2. Why a Dependency Graph Rule Engine Is Needed

Once your engine grows, you hit a new problem:

**rule ordering and dependencies**

Example chain:

base ability score  
↓  
racial bonus  
↓  
ASI  
↓  
ability modifier  
↓  
skill bonus

If applied in the wrong order:

skill bonus calculated before ability modifier

You get incorrect results.

---

## Another Example: Extra Attack

Rules:

Fighter lvl 5 → Extra Attack  
Fighter lvl 11 → Extra Attack (2)

These must **override each other**.

---

## Another Example: AC

Base AC = 10 + Dex  
  
Mage Armor:  
AC = 13 + Dex  
  
Shield spell:  
+5 AC

These interact in complicated ways.

---

# The Dependency Graph Solution

Instead of processing rules in a simple loop, build a **directed graph of rule dependencies**.

Each rule node declares:

what it depends on  
what it produces

---

## Concept

Rules form a graph:

BaseAbility  
      ↓  
AbilityBonuses  
      ↓  
AbilityModifier  
      ↓  
SkillBonus

The engine performs a **topological sort** to determine correct execution order.

---

# Graph Node Model

Each rule node describes:

inputs  
outputs  
operation

Rust structure:

pub struct RuleNode {  
    pub id: RuleId,  
    pub depends_on: Vec<RuleId>,  
    pub apply: fn(&mut CharacterSheet),  
}

Example nodes:

ability_bonus  
ability_modifier  
skill_bonus  
proficiency_bonus

---

# Graph Example

For skill calculation:

AbilityScore  
   ↓  
AbilityModifier  
   ↓  
SkillBonus

Graph:

AbilityScore → AbilityModifier → SkillBonus

---

# Engine Execution

Steps:

1 load rules  
2 build dependency graph  
3 topological sort  
4 execute nodes in order

---

# Example Rust Implementation

Node:

pub struct RuleNode {  
    pub id: &'static str,  
    pub deps: Vec<&'static str>,  
    pub run: fn(&mut CharacterSheet),  
}

Graph execution:

fn run_rules(nodes: Vec<RuleNode>, sheet: &mut CharacterSheet) {  
  
    let order = topo_sort(nodes);  
  
    for node in order {  
        (node.run)(sheet);  
    }  
}

---

# Topological Sort

This resolves dependency order.

Example order:

BaseAbility  
AbilityBonus  
AbilityModifier  
SkillBonus

Many Rust crates implement this:

petgraph  
daggy

---

# Using It in a 5e Engine

Instead of effect phases, you use **rule dependencies**.

Example rules:

AbilityBase  
AbilityBonus  
AbilityModifier  
ProficiencyBonus  
SkillBonus  
SpellSaveDC

Graph:

AbilityBase  
   ↓  
AbilityBonus  
   ↓  
AbilityModifier  
   ↓  
SkillBonus  
   ↓  
SpellSaveDC

---

# How Features Fit Into This

Features **inject effects into nodes**.

Example:

Elf Dex Bonus

Adds effect to node:

AbilityBonus

Example:

Jack of All Trades

Adds effect to node:

SkillBonus

---

# Result

Your engine becomes:

Rule graph (static)  
       ↓  
Features add effects  
       ↓  
Nodes execute  
       ↓  
CharacterSheet

---

# Why This Is Extremely Powerful

It solves complex rule interactions automatically.

Example:

ASI modifies ability  
↓  
modifier recalculated  
↓  
skill recalculated  
↓  
spell DC recalculated

No manual ordering required.

---

# Rust Crate Recommendation

Best library for this:

petgraph

It provides:

Graph  
toposort  
dependency management

---

# Example with petgraph

Simplified:

use petgraph::graph::DiGraph;  
  
let mut graph = DiGraph::<RuleNode, ()>::new();  
  
let ability = graph.add_node(ability_node);  
let modifier = graph.add_node(modifier_node);  
  
graph.add_edge(ability, modifier, ());

Then:

petgraph::algo::toposort(&graph, None)

---

# Final Architecture with Dependency Graph

CharacterDefinition  
        ↓  
Feature Resolver  
        ↓  
Effect Collection  
        ↓  
Dependency Graph Engine  
        ↓  
CharacterSheet  
        ↓  
CharacterState

---

# One Important Insight

Most engines evolve through **three stages**:

### Stage 1 (simple builders)

loop over features  
apply effects

### Stage 2 (phased rules)

ability phase  
skill phase  
spell phase

### Stage 3 (dependency graph)

rules define dependencies  
engine resolves automatically

Stage 3 is the most robust for **large RPG rulesets**.

---

✅ If you'd like, I can also show something extremely useful for a Rust implementation:

**how to model the rule graph as a typed ECS-like system**

This is how some modern RPG engines and simulation systems achieve **very high performance while remaining fully data-driven**.

please do.

An **ECS-like rule graph** is one of the cleanest ways to implement a large RPG rules engine in Rust. It combines three useful ideas:

1. **Data-driven rules** (effects from features)
    
2. **Dependency-aware computation** (rule graph)
    
3. **Entity-component-system style data layout**
    

You’re not building a game engine ECS like Bevy, but you **borrow the architecture pattern** because it works extremely well for rule evaluation.

The key insight:

> A character sheet is really just a set of **components**, and rules are **systems that transform them**.

---

# 1. Why an ECS-Like Design Works Well for 5e

A D&D character sheet is basically a **bundle of independent values**:

Strength  
Dexterity  
HP  
AC  
Skill bonuses  
Spell slots  
Speed

These values have **dependencies**:

Ability Score  
     ↓  
Ability Modifier  
     ↓  
Skill Bonus  
     ↓  
Passive Perception

ECS architecture models this perfectly.

Instead of storing everything in one struct, the sheet becomes a **component store**.

---

# 2. Conceptual Model

The rule engine becomes:

CharacterDefinition  
        ↓  
Feature Resolver  
        ↓  
Effects  
        ↓  
ECS Rule Graph  
        ↓  
Character Components

Where:

components = data  
systems = rules

---

# 3. Component Model

Each derived stat becomes a component.

Example:

struct AbilityScores;  
struct AbilityModifiers;  
struct SkillBonuses;  
struct ArmorClass;  
struct HitPoints;  
struct Spellcasting;

But components contain data.

Example:

pub struct AbilityScores {  
    pub str: i32,  
    pub dex: i32,  
    pub con: i32,  
    pub int: i32,  
    pub wis: i32,  
    pub cha: i32,  
}

Modifier component:

pub struct AbilityModifiers {  
    pub str: i32,  
    pub dex: i32,  
    pub con: i32,  
    pub int: i32,  
    pub wis: i32,  
    pub cha: i32,  
}

Skill bonuses:

pub struct SkillBonuses {  
    pub bonuses: HashMap<Skill, i32>,  
}

---

# 4. Component Storage

Instead of storing these in a giant struct, the rule engine stores them in a **type map**.

Example:

use std::any::{Any, TypeId};  
use std::collections::HashMap;  
  
pub struct ComponentStore {  
    components: HashMap<TypeId, Box<dyn Any>>,  
}

Insert:

store.insert(AbilityScores { ... });

Retrieve:

let scores = store.get::<AbilityScores>();

This gives you **dynamic component storage** similar to ECS frameworks.

---

# 5. Rule Systems

Rules become **systems that operate on components**.

Example system:

AbilityModifierSystem

Rust implementation:

pub trait RuleSystem {  
    fn run(&self, store: &mut ComponentStore);  
}

Example system:

pub struct AbilityModifierSystem;  
  
impl RuleSystem for AbilityModifierSystem {  
    fn run(&self, store: &mut ComponentStore) {  
  
        let scores = store.get::<AbilityScores>().unwrap();  
  
        let mut modifiers = AbilityModifiers {  
            str: (scores.str - 10) / 2,  
            dex: (scores.dex - 10) / 2,  
            con: (scores.con - 10) / 2,  
            int: (scores.int - 10) / 2,  
            wis: (scores.wis - 10) / 2,  
            cha: (scores.cha - 10) / 2,  
        };  
  
        store.insert(modifiers);  
    }  
}

---

# 6. Dependency Graph

Each system declares which components it:

reads  
writes

Example:

AbilityModifierSystem  
reads: AbilityScores  
writes: AbilityModifiers

Skill system:

SkillBonusSystem  
reads: AbilityModifiers  
reads: ProficiencyBonus  
writes: SkillBonuses

Graph:

AbilityScores  
     ↓  
AbilityModifiers  
     ↓  
SkillBonuses

The engine uses this to compute execution order.

---

# 7. Rule Node Definition

A system declares dependencies like this:

pub struct SystemInfo {  
    pub name: &'static str,  
    pub reads: Vec<TypeId>,  
    pub writes: Vec<TypeId>,  
}

Example:

SystemInfo {  
    name: "ability_modifiers",  
    reads: vec![TypeId::of::<AbilityScores>()],  
    writes: vec![TypeId::of::<AbilityModifiers>()],  
}

The rule engine builds the dependency graph automatically.

---

# 8. Feature Effects in ECS

Features modify components.

Example feature:

Elf Dexterity Bonus

Effect:

AbilityScores.dex += 2

Effect representation:

pub enum Effect {  
  
    ModifyAbility {  
        ability: Ability,  
        amount: i32,  
    },  
  
    GrantProficiency {  
        proficiency: Proficiency,  
    },  
  
}

Effect application system:

FeatureEffectSystem

Example:

for effect in effects {  
    match effect {  
  
        Effect::ModifyAbility { ability, amount } => {  
            let scores = store.get_mut::<AbilityScores>().unwrap();  
            scores.add(*ability, *amount);  
        }  
  
        _ => {}  
    }  
}

---

# 9. Example Execution Graph

Full rule graph might look like:

BaseAbilityScores  
        ↓  
FeatureEffects  
        ↓  
AbilityModifiers  
        ↓  
ProficiencyBonus  
        ↓  
SkillBonuses  
        ↓  
ArmorClass  
        ↓  
SpellSaveDC

The graph ensures correct ordering.

---

# 10. Execution Engine

Engine runs systems in dependency order.

pub struct RuleEngine {  
    systems: Vec<Box<dyn RuleSystem>>,  
}

Execution:

pub fn run(&self, store: &mut ComponentStore) {  
    for system in &self.systems {  
        system.run(store);  
    }  
}

Systems are pre-sorted by dependency graph.

---

# 11. Spellcasting in ECS

Spellcasting becomes components:

Spellcasting  
SpellSlots  
PreparedSpells

Example:

pub struct SpellSlots {  
    pub slots: HashMap<u8, u8>,  
}

Multiclass system:

SpellcastingProgressionSystem

It reads:

ClassLevels

and writes:

SpellSlots

---

# 12. Runtime State (Not ECS)

Gameplay state should **not** live in the ECS rule engine.

Instead keep:

pub struct CharacterState {  
    current_hp: i32,  
    spell_slots_used: HashMap<u8, u8>,  
}

The ECS engine produces **maximum capabilities**, not runtime usage.

---

# 13. Why This Architecture Is Powerful

It solves several hard problems simultaneously.

### Rule ordering

Handled by dependency graph.

---

### Feature stacking

Multiple features modify same component.

Example:

Dex +2 (race)  
Dex +1 (feat)  
Dex +2 (ASI)

All applied before modifiers calculated.

---

### New rule systems are easy

Add a new system:

InitiativeSystem

Dependencies determine execution order automatically.

---

### Clean modular design

Systems are small and independent.

---

# 14. Performance

This architecture is extremely fast.

Typical character evaluation:

~50–200 systems

Runtime:

microseconds

Because:

no recursion  
no dynamic rule evaluation  
linear system execution

---

# 15. How This Differs From Full Game ECS

Game ECS (like Bevy):

thousands of entities  
real-time updates

Your rule ECS:

one entity (the character)  
dozens of components  
batch evaluation

So you don't need a full ECS framework.

A **lightweight ECS pattern** is perfect.

---

# 16. Final Architecture

Your complete system becomes:

Rules (JSON)  
      ↓  
Rule Database  
      ↓  
CharacterDefinition  
      ↓  
Feature Resolver  
      ↓  
Effect List  
      ↓  
ECS Rule Engine  
      ↓  
Character Components  
      ↓  
CharacterSheet  
      ↓  
CharacterState

---

✅ If you'd like, I can also show something **extremely important for a real 5e engine**:

**how to model stacking rules and overrides correctly** (for things like AC formulas, Extra Attack progression, and feature replacement).

This is one of the **hardest parts of implementing 5e correctly**, and most rule engines get it wrong on the first attempt.

returning to the original text file I gave you, how would the design change for adding an inventory system. This must account for money, adventuring gear (including container items like backpacks), weapons, armor, and magic items which can have many features, effects, and bonuses.

Adding an **inventory system** to the architecture described in your file becomes interesting because it interacts heavily with the **rules engine** (bonuses, attunement, proficiency effects, encumbrance, etc.). If we keep the original principles from the conversation:

- **Trait-free data-driven rules**
    
- **Dependency graph rule engine**
    
- **Clean domain/application separation**
    

then the inventory system should **not introduce behavior into item types**. Instead, items become **data that emit rule nodes into the dependency graph**.

I'll break this down into:

1. What changes in the architecture
    
2. Inventory domain model (Rust-idiomatic)
    
3. Containers (backpacks, bags, etc.)
    
4. Magic items and rule effects
    
5. How inventory plugs into the dependency graph rule engine
    
6. Example rule evaluation flow
    
7. Recommended module structure
    

---

# 1. Architectural Impact

Your original system likely had something like:

Character  
 ├─ race  
 ├─ class  
 ├─ abilities  
 ├─ feats  
 └─ rules engine

With inventory, we extend this:

Character  
 ├─ race  
 ├─ class  
 ├─ abilities  
 ├─ feats  
 ├─ inventory  
 └─ rules engine

But **inventory itself does not compute rules**.

Instead:

Inventory -> emits rule sources -> dependency graph

Example:

Longsword +1  
    -> AttackBonus +1  
    -> MagicalWeapon flag

Belt of Giant Strength  
    -> Strength = 21

Plate Armor  
    -> Base AC = 18  
    -> Disadvantage on Stealth

The **rules engine aggregates everything**.

Race, class, feats, AND items all behave the same way:

RuleSource -> produces rule nodes -> graph

This keeps the system **fully data-driven**.

---

# 2. Inventory Domain Model

Inventory must support:

- currency
    
- stackable items
    
- containers
    
- equipment
    
- attunement
    
- item features
    

Rust-idiomatic structure:

pub struct Inventory {  
    pub coins: Coins,  
    pub items: Vec<ItemInstance>,  
}

Coins:

pub struct Coins {  
    pub cp: u32,  
    pub sp: u32,  
    pub ep: u32,  
    pub gp: u32,  
    pub pp: u32,  
}

Item instance (important):

pub struct ItemInstance {  
    pub id: ItemId,  
    pub quantity: u32,  
    pub equipped: bool,  
    pub attuned: bool,  
    pub location: ItemLocation,  
}

Where the item definition lives in **data**:

pub struct ItemDefinition {  
    pub id: ItemId,  
    pub name: String,  
    pub kind: ItemKind,  
    pub weight: f32,  
    pub properties: Vec<ItemProperty>,  
    pub rules: Vec<RuleEffect>,  
}

Important distinction:

ItemDefinition -> static data  
ItemInstance -> character-owned object

Example:

ItemDefinition: Longsword  
ItemInstance: Longsword #3 owned by character

---

# 3. Item Types (Still Data Driven)

Instead of traits like:

trait Weapon  
trait Armor  
trait MagicItem

Use **data enums**.

pub enum ItemKind {  
    Weapon(WeaponData),  
    Armor(ArmorData),  
    Gear(GearData),  
    Container(ContainerData),  
    MagicItem(MagicItemData),  
}

Example weapon data:

pub struct WeaponData {  
    pub damage: Damage,  
    pub properties: Vec<WeaponProperty>,  
}

Armor:

pub struct ArmorData {  
    pub base_ac: u8,  
    pub dex_bonus: DexBonusRule,  
    pub stealth_disadvantage: bool,  
}

Container:

pub struct ContainerData {  
    pub capacity_weight: f32,  
}

Magic items:

pub struct MagicItemData {  
    pub requires_attunement: bool,  
}

Again:

**No behavior. Just data.**

---

# 4. Containers (Backpacks etc.)

Inventory systems often fail here. The correct model is **a tree**.

Instead of flat inventory:

Inventory  
 ├─ Backpack  
 │   ├─ Rope  
 │   └─ Torch  
 └─ Sword

Represent location:

pub enum ItemLocation {  
    Carried,  
    Equipped,  
    InContainer(ItemId),  
}

This allows nesting.

Example:

Torch -> InContainer(backpack_id)

Then you can compute weight recursively.

---

# 5. Magic Items and Rule Effects

Magic items should produce **rule effects**.

Example item data:

id: belt_giant_strength  
name: Belt of Hill Giant Strength  
kind: magic  
attunement: true  
  
rules:  
  - type: set_ability_score  
    ability: strength  
    value: 21

Another example:

id: longsword_plus_1  
name: Longsword +1  
  
rules:  
  - type: bonus  
    target: attack_roll  
    value: 1  
  - type: bonus  
    target: damage_roll  
    value: 1

These rules become **nodes in the dependency graph**.

---

# 6. Integrating Inventory With the Rule Graph

Your rule engine likely works like this:

Nodes = values  
Edges = dependencies

Example:

Strength Modifier  
      ↑  
Strength Score  
      ↑  
Belt of Giant Strength

Inventory contributes nodes only if items are:

equipped  
or  
attuned

Pseudo:

impl Inventory {  
    pub fn rule_sources(&self, items: &ItemDatabase) -> Vec<RuleSource> {  
        self.items  
            .iter()  
            .filter(|item| item.equipped || item.attuned)  
            .map(|instance| items.get(instance.id))  
            .flat_map(|def| def.rules.clone())  
            .collect()  
    }  
}

These rules are added to the graph.

---

# 7. Dependency Graph Example

Suppose character equips:

Plate Armor  
Shield  
Ring of Protection

Graph becomes:

Armor Base AC = 18  
Shield Bonus = +2  
Ring Bonus = +1  
Dex Modifier

Dependencies:

AC  
 ├─ ArmorBase  
 ├─ ShieldBonus  
 ├─ DexModifier  
 └─ ItemBonuses

Rule engine computes:

AC = 18 + 2 + 1

---

# 8. Encumbrance Rules

Inventory also affects:

carry_weight

Graph:

TotalWeight  
    ↑  
ItemWeights

Encumbered?  
    ↑  
TotalWeight  
    ↑  
Strength

Inventory contributes:

Σ(item.weight * quantity)

---

# 9. Attunement Rules

Magic items often require attunement.

Inventory logic:

max_attuned = 3

Graph:

AttunedItems  
     ↑  
Inventory

If >3 → invalid state.

The application layer enforces this.

---

# 10. Example Evaluation Flow

Player equips a **Belt of Giant Strength**.

Steps:

1. Inventory marks item `equipped + attuned`
    
2. Inventory emits rule source
    
3. Rule engine inserts node
    

Graph:

StrengthScore  
   ↑  
BaseStrength  
   ↑  
RacialBonus  
  
StrengthOverride  
   ↑  
BeltOfGiantStrength

Resolution rule:

override > additive

Final:

Strength = 21  
Modifier = +5

---

# 11. Suggested Module Layout

Your project could evolve to:

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

---

# 12. Why This Design Scales Well

Because **everything becomes rule sources**:

|Source|Adds Rules|
|---|---|
|Race|ability bonuses|
|Class|proficiencies|
|Feats|features|
|Items|bonuses/effects|
|Spells|temporary effects|

So the rules engine only cares about:

Vec<RuleSource>

Not where they came from.

---

# 13. The Biggest Advantage

This design makes **adding new items trivial**.

You don't write Rust code for items.

You add data:

items/boots_of_elvenkind.yaml

And the engine already understands it.

That is **essential** if you ever support:

- homebrew
    
- new books
    
- modding
    

---

✅ If you'd like, I can also show the **Rust implementation of the dependency graph rule engine itself**, which is the hardest (and most interesting) part of building a 5e rules system.

please do

Below is a **Rust-idiomatic implementation strategy for a dependency-graph rule engine** suitable for a **D&D 5e rules system**. The goal is to keep it:

- **data-driven**
    
- **trait-light**
    
- **deterministic**
    
- **incrementally recomputable**
    
- **fast enough for UI updates**
    

This architecture works very well for systems where many things contribute to a final value:

AC  
Attack bonus  
Spell save DC  
Skill bonuses  
Encumbrance  
Passive perception

All of those are **derived values**.

---

# 1. Core Concept

Instead of computing values procedurally:

fn compute_ac(character: &Character) -> u32 {  
    ...  
}

You build a **dependency graph**.

Example:

AC  
├─ ArmorBase  
├─ ShieldBonus  
├─ DexModifier  
└─ MiscBonuses

Each node declares what it **depends on**.

The engine:

1. builds the graph
    
2. topologically sorts it
    
3. evaluates nodes in order
    

---

# 2. Core Types

We begin with a **value type**.

#[derive(Clone, Debug)]  
pub enum Value {  
    Int(i32),  
    Bool(bool),  
}

(You can extend this later with dice, lists, etc.)

---

### NodeId

Every rule target gets a stable key.

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]  
pub struct NodeId(pub &'static str);

Examples:

"strength_score"  
"strength_mod"  
"armor_class"  
"proficiency_bonus"

---

# 3. Rule Nodes

Each node describes:

- what it depends on
    
- how to compute itself
    

pub struct Node {  
    pub id: NodeId,  
    pub deps: Vec<NodeId>,  
    pub compute: fn(&Context) -> Value,  
}

This avoids traits entirely.

Just **function pointers**.

Rust-friendly and simple.

---

# 4. Evaluation Context

The compute functions read already-computed values.

use std::collections::HashMap;  
  
pub struct Context {  
    values: HashMap<NodeId, Value>,  
}  
  
impl Context {  
    pub fn get_int(&self, id: NodeId) -> i32 {  
        match self.values.get(&id).unwrap() {  
            Value::Int(v) => *v,  
            _ => panic!("expected int"),  
        }  
    }  
  
    pub fn set(&mut self, id: NodeId, value: Value) {  
        self.values.insert(id, value);  
    }  
}

---

# 5. Rule Graph

The engine stores all nodes.

pub struct RuleGraph {  
    nodes: HashMap<NodeId, Node>,  
}

Adding nodes:

impl RuleGraph {  
    pub fn add_node(&mut self, node: Node) {  
        self.nodes.insert(node.id, node);  
    }  
}

Nodes come from:

race  
class  
inventory  
feats  
spells  
conditions

Everything contributes nodes.

---

# 6. Topological Sorting

To evaluate correctly we must sort dependencies.

Simple Kahn algorithm:

pub fn topo_sort(nodes: &HashMap<NodeId, Node>) -> Vec<NodeId> {  
    use std::collections::{HashMap, VecDeque};  
  
    let mut indegree = HashMap::new();  
    let mut edges: HashMap<NodeId, Vec<NodeId>> = HashMap::new();  
  
    for node in nodes.values() {  
        indegree.entry(node.id).or_insert(0);  
  
        for dep in &node.deps {  
            edges.entry(*dep).or_default().push(node.id);  
            *indegree.entry(node.id).or_insert(0) += 1;  
        }  
    }  
  
    let mut queue = VecDeque::new();  
  
    for (id, deg) in &indegree {  
        if *deg == 0 {  
            queue.push_back(*id);  
        }  
    }  
  
    let mut order = Vec::new();  
  
    while let Some(id) = queue.pop_front() {  
        order.push(id);  
  
        if let Some(children) = edges.get(&id) {  
            for child in children {  
                let deg = indegree.get_mut(child).unwrap();  
                *deg -= 1;  
  
                if *deg == 0 {  
                    queue.push_back(*child);  
                }  
            }  
        }  
    }  
  
    order  
}

Now we have a **safe evaluation order**.

---

# 7. Graph Evaluation

impl RuleGraph {  
    pub fn evaluate(&self) -> Context {  
        let order = topo_sort(&self.nodes);  
        let mut ctx = Context {  
            values: HashMap::new(),  
        };  
  
        for id in order {  
            let node = &self.nodes[&id];  
            let value = (node.compute)(&ctx);  
            ctx.set(id, value);  
        }  
  
        ctx  
    }  
}

That's the entire **rule engine loop**.

---

# 8. Example: Ability Score Modifier

Define IDs:

const STR_SCORE: NodeId = NodeId("strength_score");  
const STR_MOD: NodeId = NodeId("strength_mod");

Nodes:

fn strength_mod(ctx: &Context) -> Value {  
    let score = ctx.get_int(STR_SCORE);  
    Value::Int((score - 10) / 2)  
}

Add to graph:

graph.add_node(Node {  
    id: STR_MOD,  
    deps: vec![STR_SCORE],  
    compute: strength_mod,  
});

---

# 9. Example: Armor Class

Dependencies:

armor_base  
dex_mod  
shield_bonus

Compute:

fn armor_class(ctx: &Context) -> Value {  
    let base = ctx.get_int(NodeId("armor_base"));  
    let dex = ctx.get_int(NodeId("dex_mod"));  
    let shield = ctx.get_int(NodeId("shield_bonus"));  
  
    Value::Int(base + dex + shield)  
}

Node:

graph.add_node(Node {  
    id: NodeId("armor_class"),  
    deps: vec![  
        NodeId("armor_base"),  
        NodeId("dex_mod"),  
        NodeId("shield_bonus"),  
    ],  
    compute: armor_class,  
});

---

# 10. How Items Inject Rules

Inventory contributes nodes.

Example: **Shield**

graph.add_node(Node {  
    id: NodeId("shield_bonus"),  
    deps: vec![],  
    compute: |_| Value::Int(2),  
});

Example: **Ring of Protection**

Instead of replacing nodes, we add **bonus nodes**.

ac_bonus_ring

Then AC depends on **sum of bonuses**.

Better design:

ac_base  
ac_bonus_total  
ac

---

# 11. Bonus Aggregation Pattern

This pattern scales best.

attack_bonus_total  
 ├─ proficiency_bonus  
 ├─ ability_mod  
 ├─ item_bonus  
 └─ misc_bonus

Item:

Longsword +1

Adds node:

longsword_bonus -> 1

Aggregator:

fn sum_attack_bonus(ctx: &Context) -> Value {  
    let prof = ctx.get_int(NodeId("proficiency_bonus"));  
    let ability = ctx.get_int(NodeId("str_mod"));  
    let item = ctx.get_int(NodeId("item_bonus"));  
  
    Value::Int(prof + ability + item)  
}

---

# 12. Inventory Integration

Inventory iterates equipped items:

for item in inventory.equipped_items() {  
    for rule in &item.rules {  
        graph.add_node(rule.to_node());  
    }  
}

Items become **graph fragments**.

---

# 13. Incremental Recalculation (Huge Improvement)

Full recomputation works, but the best engines do **partial updates**.

Track:

dirty nodes

Example:

equip armor

Dirty nodes:

armor_base  
armor_class

Not:

spell_save_dc  
initiative

This makes the system **UI-fast**.

Libraries like **Salsa** use similar ideas.

---

# 14. Why This Architecture Works So Well for 5e

Because 5e rules are **dependency heavy**:

Attack Roll  
 ├─ Ability Modifier  
 │   └─ Ability Score  
 ├─ Proficiency Bonus  
 │   └─ Character Level  
 └─ Magic Item Bonus

Dependency graphs match this perfectly.

---

# 15. Typical Node Count

A full character usually has:

150–400 nodes

Which is trivial for Rust.

Evaluation typically takes:

<1 ms

---

# 16. Final Architecture Overview

Game Data  
  ├─ races  
  ├─ classes  
  ├─ feats  
  ├─ items  
  
Character  
  ├─ ability scores  
  ├─ class levels  
  ├─ inventory  
  
Inventory  
  └─ equipped items  
  
All sources emit  
      ↓  
  
Rule Nodes  
      ↓  
  
RuleGraph  
      ↓  
  
Topological Sort  
      ↓  
  
Evaluation  
      ↓  
  
Computed Character Sheet

---

✅ If you'd like, I can also show the **next major evolution used by professional character builders**:

**Typed rule graphs + stacking rules (the hardest part of implementing 5e correctly).**

That covers things like:

- advantage/disadvantage
    
- bonus stacking rules
    
- “set vs add vs override”
    
- armor formula conflicts
    
- multiple AC calculations (Mage Armor vs Plate)
    

Those are where most hobby engines break.

Returning to the inventory, for containers, how does representing `ItemLocation` create a tree structure? Wouldn't it make more sense to store containers as `Vec<Vec<ItemInstance>>` or for names/ids as `HashMap<String, Vec<ItemInstance>>`?

Great question. This is a classic modeling decision in inventory systems. The short answer is:

**`ItemLocation::InContainer(ItemId)` produces a tree implicitly via references rather than explicitly via nesting.**

This pattern is usually preferred in game engines because it avoids ownership and mutation problems that appear with nested `Vec<Vec<_>>`.

Let's break it down carefully.

---

# 1. The Reference-Based Tree (ItemLocation)

Earlier we suggested:

pub enum ItemLocation {  
    Carried,  
    Equipped,  
    InContainer(ItemId),  
}

and the inventory:

pub struct Inventory {  
    pub items: Vec<ItemInstance>,  
}

Example data:

Inventory  
 ├─ Backpack (id=1)     location: Carried  
 ├─ Rope     (id=2)     location: InContainer(1)  
 ├─ Torch    (id=3)     location: InContainer(1)  
 └─ Sword    (id=4)     location: Equipped

Even though this is **stored flat**, it represents a tree:

Carried  
 ├─ Backpack  
 │   ├─ Rope  
 │   └─ Torch  
 └─ Sword

To get children:

fn children_of(container: ItemId, inv: &Inventory) -> Vec<&ItemInstance> {  
    inv.items  
        .iter()  
        .filter(|i| matches!(i.location, ItemLocation::InContainer(id) if id == container))  
        .collect()  
}

This reconstructs the tree dynamically.

---

# 2. Why Game Engines Prefer the Reference Model

Because nested containers cause **ownership and mutation problems**.

Imagine this structure:

Vec<Vec<ItemInstance>>

or

struct Container {  
    items: Vec<ItemInstance>  
}

Then moving an item requires:

1. removing from one vector
    
2. inserting into another
    
3. handling borrow rules
    
4. handling recursive containers
    

In Rust this gets messy fast.

With the **reference model**:

item.location = ItemLocation::InContainer(backpack_id);

That's a single assignment.

No moving memory around.

---

# 3. Containers Can Nest Arbitrarily

Example:

Backpack  
 └─ Pouch  
     └─ Gem

Flat storage:

1 Backpack   Carried  
2 Pouch      InContainer(1)  
3 Gem        InContainer(2)

Tree emerges from following references.

---

# 4. Weight Calculation Example

Recursive traversal:

fn container_weight(id: ItemId, inv: &Inventory, db: &ItemDb) -> f32 {  
    let mut total = 0.0;  
  
    for item in inv.items.iter().filter(|i| i.location == ItemLocation::InContainer(id)) {  
        let def = db.get(item.id);  
        total += def.weight * item.quantity as f32;  
  
        if def.is_container() {  
            total += container_weight(item.id, inv, db);  
        }  
    }  
  
    total  
}

---

# 5. Why `Vec<Vec<ItemInstance>>` Is Problematic

Suppose:

Vec<Vec<ItemInstance>>

You immediately lose:

- container identity
    
- nesting
    
- moving items between containers
    

Example problem:

Backpack -> Pouch -> Gem

`Vec<Vec<_>>` cannot represent **multi-level nesting** cleanly.

You would need:

struct Container {  
    items: Vec<Item>,  
}

But now items must be either:

enum Item {  
    Item(ItemInstance),  
    Container(Container)  
}

This becomes a **recursive ownership structure**, which is much harder to manipulate.

---

# 6. Why `HashMap<String, Vec<ItemInstance>>` Is Also Weak

Example:

HashMap<String, Vec<ItemInstance>>

Problems:

### 1. Names aren't stable identifiers

Two backpacks?

"backpack"  
"backpack"

Collision.

---

### 2. Renaming breaks references

If container name changes:

"Adventurer's Backpack"

Everything breaks.

---

### 3. No nesting

You can't represent:

Backpack  
 └─ Pouch

without more maps.

---

# 7. The Typical Game Architecture

Most engines use:

Vec<Entity>

with parent references.

Example:

struct ItemInstance {  
    id: ItemId,  
    parent: Option<ItemId>,  
}

This is essentially the same model used by **ECS engines** like:

- Bevy
    
- Unity
    
- Unreal
    

Where hierarchy is:

Parent -> Child

not nested vectors.

---

# 8. Efficient Lookup Improvement

If scanning `Vec<ItemInstance>` worries you, add an index:

HashMap<ItemId, Vec<ItemId>>

Example:

container_children  
  
1 -> [2,3]  
2 -> [4]

Structure:

pub struct Inventory {  
    items: HashMap<ItemId, ItemInstance>,  
    children: HashMap<ItemId, Vec<ItemId>>,  
}

Now lookups are **O(1)**.

---

# 9. Best Rust-Idiomatic Inventory Model

What most robust systems end up with:

pub struct Inventory {  
    items: HashMap<ItemId, ItemInstance>,  
}  
  
pub struct ItemInstance {  
    pub id: ItemId,  
    pub quantity: u32,  
    pub location: ItemLocation,  
}

Location:

pub enum ItemLocation {  
    Carried,  
    Equipped,  
    InContainer(ItemId),  
}

Advantages:

✔ flat storage  
✔ easy mutation  
✔ arbitrary nesting  
✔ stable references  
✔ efficient traversal

---

# 10. Visual Summary

### Nested storage

Backpack  
 ├─ Rope  
 └─ Torch

Representation:

Container {  
   items: Vec<Item>  
}

Problems:

- borrow complexity
    
- hard moves
    
- recursive ownership
    

---

### Reference storage (recommended)

Backpack (1)  
Rope (2) -> InContainer(1)  
Torch (3) -> InContainer(1)

Flat storage:

Vec<ItemInstance>

Tree reconstructed via references.

---

# 11. Why This Matters for the Rule Engine

Flat storage also integrates better with your **rule graph**.

Inventory can easily iterate:

inventory.items.iter()

and emit rule sources.

With nested containers you'd need **deep recursion** just to enumerate items.

---

✅ If you'd like, I can also show the **inventory architecture used by large RPG systems (Baldur's Gate / Skyrim style)** that solves three very hard problems simultaneously:

- stackable items
    
- containers
    
- equipment slots
    
- unique vs stackable objects
    

in a **single consistent Rust data model**.

Show me Typed rule graphs + stacking rules as mentioned earlier

Implementing **typed rule graphs + stacking rules** is where most tabletop rule engines either become elegant or collapse under edge cases. D&D 5e especially needs this because many effects interact via **different stacking semantics**:

- additive bonuses (`+1`)
    
- overrides (`Strength becomes 21`)
    
- mutually exclusive formulas (AC from armor vs Mage Armor vs Unarmored Defense)
    
- advantage/disadvantage
    
- minimums / caps
    
- conditional effects
    

A **typed rule graph** solves this by giving each node:

1. a **value type**
    
2. a **stacking rule**
    
3. a **set of contributors**
    

Instead of each rule computing the value directly, rules **contribute to a typed aggregator node**.

---

# 1. Core Idea

Instead of this simple node:

AC = armor + dex + shield + bonuses

we use a **typed aggregation node**:

ArmorClass  
   aggregation: Max  
   contributors:  
      ArmorFormula  
      MageArmorFormula  
      UnarmoredDefense

While bonuses stack elsewhere:

AC_Bonus  
   aggregation: Sum

Then:

AC = AC_Formula + AC_Bonus

This separation is crucial.

---

# 2. Typed Value System

Define a type-safe value system.

#[derive(Clone, Debug)]  
pub enum Value {  
    Int(i32),  
    Bool(bool),  
}

But typed nodes track the expected type:

#[derive(Clone, Copy)]  
pub enum ValueType {  
    Int,  
    Bool,  
}

Each node declares its type.

---

# 3. Stacking Rules

Define stacking strategies.

pub enum StackRule {  
    Sum,  
    Max,  
    Min,  
    Override,  
    BooleanOr,  
    BooleanAnd,  
}

Examples in 5e:

|Rule|Example|
|---|---|
|Sum|attack bonuses|
|Max|AC formulas|
|Override|Belt of Giant Strength|
|BooleanOr|advantage|
|BooleanAnd|conditions|

---

# 4. Node Structure

Instead of compute functions, aggregation nodes collect **contributions**.

pub struct Node {  
    pub id: NodeId,  
    pub value_type: ValueType,  
    pub stack: StackRule,  
    pub deps: Vec<NodeId>,  
}

But contributions are separate.

---

# 5. Rule Contributions

Rules emit **contributions**.

pub struct Contribution {  
    pub target: NodeId,  
    pub source: NodeId,  
}

Example item rule:

Ring of Protection  
   -> contributes +1 to ac_bonus

Example:

Contribution {  
    target: NodeId("ac_bonus"),  
    source: NodeId("ring_protection_bonus"),  
}

---

# 6. Constant Nodes

Many rule sources are constants.

pub struct ConstantNode {  
    pub id: NodeId,  
    pub value: Value,  
}

Example:

ConstantNode {  
    id: NodeId("ring_protection_bonus"),  
    value: Value::Int(1),  
}

---

# 7. Aggregation Engine

Evaluation collects values from contributors.

Pseudo code:

fn evaluate_node(node: &Node, ctx: &Context) -> Value {  
    let values = ctx.values_of(node.deps);  
  
    match node.stack {  
        StackRule::Sum => sum(values),  
        StackRule::Max => max(values),  
        StackRule::Override => last(values),  
        StackRule::BooleanOr => any(values),  
        StackRule::BooleanAnd => all(values),  
        _ => unreachable!(),  
    }  
}

---

# 8. Example: Attack Bonus

Nodes:

attack_bonus_total (Sum)  
 ├─ proficiency_bonus  
 ├─ ability_mod  
 ├─ weapon_bonus  
 └─ misc_bonus

Graph nodes:

Node {  
    id: NodeId("attack_bonus_total"),  
    value_type: ValueType::Int,  
    stack: StackRule::Sum,  
    deps: vec![  
        NodeId("proficiency_bonus"),  
        NodeId("ability_mod"),  
        NodeId("weapon_bonus"),  
        NodeId("misc_bonus"),  
    ],  
}

---

# 9. Example: Strength Override (Belt of Giant Strength)

Strength has two nodes:

strength_base (Sum)  
strength_override (Max)

Final score:

strength_score = max(strength_base, strength_override)

Nodes:

Node {  
    id: NodeId("strength_score"),  
    value_type: ValueType::Int,  
    stack: StackRule::Max,  
    deps: vec![  
        NodeId("strength_base"),  
        NodeId("strength_override")  
    ],  
}

Item rule:

Belt of Giant Strength  
  -> strength_override = 21

---

# 10. Example: Armor Class Formulas

5e has multiple AC formulas:

10 + Dex  
Armor AC  
Mage Armor  
Unarmored Defense

These **do not stack**.

So we separate:

ac_formula (Max)

Contributors:

armor_formula  
mage_armor_formula  
unarmored_formula

Example:

Node {  
    id: NodeId("ac_formula"),  
    stack: StackRule::Max,  
    deps: vec![  
        NodeId("armor_formula"),  
        NodeId("mage_armor_formula"),  
        NodeId("unarmored_formula")  
    ],  
}

Then bonuses:

ac_bonus (Sum)

Final AC:

ac = ac_formula + ac_bonus

---

# 11. Advantage / Disadvantage

Represent advantage as booleans.

Nodes:

attack_advantage (BooleanOr)  
attack_disadvantage (BooleanOr)

Rules contribute:

faerie_fire -> attack_advantage = true  
invisible_target -> attack_disadvantage = true

Resolution node:

attack_roll_mode

Compute:

fn resolve_roll(ctx: &Context) -> RollMode {  
    let adv = ctx.get_bool("attack_advantage");  
    let dis = ctx.get_bool("attack_disadvantage");  
  
    match (adv, dis) {  
        (true, false) => RollMode::Advantage,  
        (false, true) => RollMode::Disadvantage,  
        _ => RollMode::Normal,  
    }  
}

This exactly matches 5e rules.

---

# 12. Inventory Example

Inventory emits rules.

Example item:

Ring of Protection

Rules:

ac_bonus += 1  
saving_throw_bonus += 1

Nodes added:

ConstantNode {  
    id: NodeId("ring_protection_ac"),  
    value: Value::Int(1),  
}

Contribution:

Contribution {  
    target: NodeId("ac_bonus"),  
    source: NodeId("ring_protection_ac"),  
}

---

# 13. Typed Graph Layout

Final graph resembles:

Strength Base (Sum)  
 ├─ base score  
 └─ racial bonus  
  
Strength Override (Max)  
 └─ belt of giant strength  
  
Strength Score (Max)  
 ├─ Strength Base  
 └─ Strength Override  
  
Strength Modifier  
 └─ Strength Score

---

# 14. Why Typed Graphs Matter

Without typing:

add 21 + strength score

Bug.

Typed nodes enforce semantics.

Example:

StrengthScore -> Int  
Advantage -> Bool  
RollMode -> Enum

This prevents rule conflicts.

---

# 15. Why Stacking Rules Are Essential

5e stacking rules differ widely.

Examples:

|Mechanic|Stack Rule|
|---|---|
|Attack bonus|Sum|
|AC formulas|Max|
|Ability overrides|Max|
|Advantage|Boolean OR|
|Disadvantage|Boolean OR|
|Conditions|Boolean OR|
|Spell DC|Derived|

Trying to treat them uniformly leads to broken rules.

---

# 16. Real Engine Size

Typical character graph:

200–600 nodes

Evaluation:

< 1ms

in Rust.

---

# 17. Final Architecture

Game Data  
   races  
   classes  
   feats  
   items  
   spells  
  
Character State  
   ability scores  
   inventory  
   conditions  
  
Rule Sources  
   ↓  
Rule Contributions  
   ↓  
Typed Rule Graph  
   ↓  
Stacking Aggregators  
   ↓  
Computed Character Sheet

---

✅ If you'd like, I can also show the **most powerful rule-engine pattern used by large RPG builders**:

**Rule graph fragments**

This lets every feature (race, class, feat, item, spell) contribute **self-contained mini-graphs**, which the engine merges automatically. It dramatically simplifies implementing the entire 5e ruleset.

please do

A **rule graph fragment system** is the architecture that lets every feature in the game—race, class feature, feat, spell, item, condition—**contribute its own mini-graph** to the rule engine. The engine then merges those fragments into the global dependency graph.

This is the design that keeps a large ruleset like 5e maintainable.

Without fragments you end up with a giant central rules file. With fragments, **every feature carries its own rules**.

---

# 1. Core Idea

Instead of writing rules in a central place:

if character.has_feat("sharpshooter") {  
    attack_bonus -= 5;  
}

Each feature contributes a **graph fragment**.

Example fragment for **Sharpshooter**:

Node: sharpshooter_penalty  
Value: -5  
  
Contribution:  
attack_bonus_total += sharpshooter_penalty

The engine just merges fragments.

---

# 2. Fragment Structure

A fragment contains:

- nodes
    
- contributions
    
- dependencies
    

Rust structure:

pub struct RuleFragment {  
    pub nodes: Vec<Node>,  
    pub constants: Vec<ConstantNode>,  
    pub contributions: Vec<Contribution>,  
}

Everything needed for that feature lives here.

---

# 3. Example: Item Fragment

**Ring of Protection**

Fragment:

Constant Node:  
ring_protection_bonus = 1  
  
Contribution:  
ac_bonus += ring_protection_bonus  
saving_throw_bonus += ring_protection_bonus

Rust:

RuleFragment {  
    constants: vec![  
        ConstantNode {  
            id: NodeId("ring_protection_bonus"),  
            value: Value::Int(1),  
        }  
    ],  
  
    contributions: vec![  
        Contribution {  
            target: NodeId("ac_bonus"),  
            source: NodeId("ring_protection_bonus"),  
        },  
        Contribution {  
            target: NodeId("saving_throw_bonus"),  
            source: NodeId("ring_protection_bonus"),  
        }  
    ],  
  
    nodes: vec![],  
}

---

# 4. Example: Class Feature Fragment

**Barbarian – Unarmored Defense**

Rule:

AC = 10 + Dex + Con

Fragment:

Nodes:

unarmored_formula = 10 + dex_mod + con_mod

Contribution:

ac_formula <- unarmored_formula

Rust:

RuleFragment {  
    nodes: vec![  
        Node {  
            id: NodeId("unarmored_formula"),  
            value_type: ValueType::Int,  
            stack: StackRule::Override,  
            deps: vec![  
                NodeId("dex_mod"),  
                NodeId("con_mod"),  
            ],  
        }  
    ],  
  
    contributions: vec![  
        Contribution {  
            target: NodeId("ac_formula"),  
            source: NodeId("unarmored_formula"),  
        }  
    ],  
  
    constants: vec![],  
}

---

# 5. Example: Spell Fragment

**Mage Armor**

Rule:

AC = 13 + Dex

Fragment:

mage_armor_formula = 13 + dex_mod

Contribution:

ac_formula <- mage_armor_formula

Graph handles the conflict automatically because:

ac_formula stack rule = Max

---

# 6. Fragment Composition

The engine gathers fragments from all sources.

Character  
 ├─ race  
 ├─ class  
 ├─ feats  
 ├─ items  
 ├─ spells  
 └─ conditions

Each returns fragments:

trait RuleSource {  
    fn fragments(&self) -> Vec<RuleFragment>;  
}

Inventory example:

impl Inventory {  
    pub fn fragments(&self, db: &ItemDb) -> Vec<RuleFragment> {  
        self.items  
            .iter()  
            .filter(|i| i.equipped)  
            .flat_map(|item| db.get(item.id).fragments())  
            .collect()  
    }  
}

---

# 7. Merging Fragments

The engine merges fragments into a global graph.

pub struct RuleEngine {  
    pub graph: RuleGraph,  
}

Merge algorithm:

fn merge_fragment(graph: &mut RuleGraph, frag: RuleFragment) {  
    for node in frag.nodes {  
        graph.add_node(node);  
    }  
  
    for constant in frag.constants {  
        graph.add_constant(constant);  
    }  
  
    for contribution in frag.contributions {  
        graph.add_contribution(contribution);  
    }  
}

---

# 8. Full Build Pipeline

Character  
    ↓  
Collect Rule Sources  
    ↓  
Produce RuleFragments  
    ↓  
Merge Fragments  
    ↓  
Build RuleGraph  
    ↓  
Topological Sort  
    ↓  
Evaluate  
    ↓  
Character Sheet

---

# 9. Why Fragments Scale So Well

Because every rule is **localized**.

Example implementation files:

rules/  
   races/  
      elf.rs  
   classes/  
      barbarian.rs  
   feats/  
      sharpshooter.rs  
   items/  
      ring_of_protection.rs

Each file just returns fragments.

---

# 10. This Enables Data-Driven Rules

Fragments can be loaded from data.

Example YAML:

id: ring_of_protection  
  
constants:  
  ring_protection_bonus: 1  
  
contributions:  
  - target: ac_bonus  
    source: ring_protection_bonus  
  - target: saving_throw_bonus  
    source: ring_protection_bonus

Loader converts it into a `RuleFragment`.

No Rust code needed.

---

# 11. Fragments Enable Conditional Rules

Example:

**Rage**

Rule:

advantage on strength checks

Fragment includes condition:

ConditionalFragment {  
    condition: Condition::Raging,  
    fragment: RuleFragment { ... }  
}

Engine activates fragments based on state.

---

# 12. Handling Equipment Slots

Fragments can depend on conditions:

if shield_equipped  
   shield_bonus = 2

Inventory system activates fragment when item is equipped.

---

# 13. Example Final Graph

For a Barbarian wearing:

- shield
    
- ring of protection
    
- Mage Armor spell
    

Graph looks like:

dex_mod  
con_mod  
  
armor_formula  
mage_armor_formula  
unarmored_formula  
  
ac_formula (Max)  
 ├─ armor_formula  
 ├─ mage_armor_formula  
 └─ unarmored_formula  
  
ac_bonus (Sum)  
 ├─ shield_bonus  
 └─ ring_bonus  
  
AC  
 ├─ ac_formula  
 └─ ac_bonus

Evaluation produces the correct AC automatically.

---

# 14. Why Professional Builders Use This

Because the number of rule sources explodes:

Approximate counts in 5e:

races: ~40  
subclasses: ~120  
feats: ~80  
spells: ~500  
items: ~1000+

Fragments let each feature **own its rules**.

---

# 15. Additional Improvements Used in Real Engines

Large systems add:

### Graph caching

Avoid rebuilding entire graph.

### Incremental recompute

Only update affected nodes.

### Rule priority

For override conflicts.

### Contextual fragments

Rules that apply only to specific rolls.

---

# 16. Final Architecture

Game Data  
  races  
  classes  
  feats  
  spells  
  items  
  
Each provides  
   ↓  
  
RuleFragments  
  
Fragments merge into  
   ↓  
  
Typed Rule Graph  
   ↓  
Stacking Aggregators  
   ↓  
Evaluation  
   ↓  
Character Sheet

---

✅ If you'd like, I can also show **the hardest part of implementing 5e rules correctly**:

**Contextual rule graphs** — how the engine computes things like

- _attack with this weapon_
    
- _spell save DC for this spell_
    
- _damage with sneak attack_
    

without rebuilding the whole graph every time.

please do

**Contextual rule graphs** solve a key problem in RPG rule engines: many calculations depend on **the specific action being performed**, not just the character’s static state.

Examples in 5e:

- Attack with **this weapon**
    
- Cast **this spell**
    
- Roll **this skill**
    
- Damage from **this attack**
    
- Saving throw against **this effect**
    

If the rule engine only has one global graph, these become messy. Contextual graphs let the engine evaluate rules **with parameters** without rebuilding everything.

---

# 1. The Core Problem

Consider **attack bonus**.

Naively you might define:

attack_bonus = proficiency + ability_mod + weapon_bonus

But ability depends on the **weapon used**.

Examples:

|Weapon|Ability|
|---|---|
|Longsword|STR|
|Rapier|DEX|
|Dagger|STR or DEX|
|Longbow|DEX|

So the graph cannot contain one fixed node:

ability_mod

It must depend on **the attack context**.

---

# 2. Introducing Rule Context

A **RuleContext** contains parameters describing the action.

Example:

pub struct RuleContext<'a> {  
    pub weapon: Option<&'a Weapon>,  
    pub spell: Option<&'a Spell>,  
    pub skill: Option<Skill>,  
    pub target: Option<&'a Creature>,  
}

Examples:

AttackContext  
   weapon = Longsword

SpellContext  
   spell = Fireball

SkillContext  
   skill = Stealth

The rule engine evaluates the graph **using this context**.

---

# 3. Contextual Nodes

Nodes can access the context.

Example:

pub struct Node {  
    pub id: NodeId,  
    pub deps: Vec<NodeId>,  
    pub compute: fn(&Context, &RuleContext) -> Value,  
}

Now a node can use the weapon.

---

# 4. Example: Ability Modifier for Weapon

Node:

attack_ability_mod

Compute:

fn attack_ability_mod(ctx: &Context, rule: &RuleContext) -> Value {  
    let weapon = rule.weapon.unwrap();  
  
    match weapon.ability {  
        WeaponAbility::Strength => {  
            Value::Int(ctx.get_int(NodeId("str_mod")))  
        }  
  
        WeaponAbility::Dexterity => {  
            Value::Int(ctx.get_int(NodeId("dex_mod")))  
        }  
  
        WeaponAbility::Finesse => {  
            let str_mod = ctx.get_int(NodeId("str_mod"));  
            let dex_mod = ctx.get_int(NodeId("dex_mod"));  
            Value::Int(str_mod.max(dex_mod))  
        }  
    }  
}

This works for any weapon.

---

# 5. Contextual Contributions

Fragments may only apply in certain contexts.

Example:

**Archery fighting style**

+2 attack bonus with ranged weapons

Fragment includes condition:

if context.weapon.is_ranged()

Rust representation:

pub struct ConditionalContribution {  
    pub target: NodeId,  
    pub source: NodeId,  
    pub condition: fn(&RuleContext) -> bool,  
}

Evaluation only includes contributions where condition is true.

---

# 6. Example: Archery Fighting Style

Fragment:

archery_bonus = 2

Contribution:

attack_bonus_total += archery_bonus

Condition:

weapon.is_ranged()

Rust:

ConditionalContribution {  
    target: NodeId("attack_bonus_total"),  
    source: NodeId("archery_bonus"),  
    condition: |ctx| ctx.weapon.map(|w| w.is_ranged()).unwrap_or(false),  
}

---

# 7. Contextual Advantage

Example:

Reckless Attack

Rule:

advantage on melee attacks using strength

Condition:

weapon.is_melee() && ability == strength

Contribution:

attack_advantage = true

---

# 8. Contextual Damage Graph

Damage depends heavily on context.

Example: **Sneak Attack**

Conditions:

weapon is finesse or ranged  
target within 5 ft of ally  
once per turn

Fragment:

sneak_attack_damage = 3d6

Contribution:

damage_bonus += sneak_attack_damage

Conditional:

ctx.weapon.is_finesse_or_ranged()

---

# 9. Graph Layering

The engine typically maintains two layers:

### Base Character Graph

Computed once.

strength_mod  
dex_mod  
proficiency_bonus  
ac

### Context Graph

Computed per action.

attack_bonus  
damage_roll  
spell_save_dc

Diagram:

Character Graph  
      ↓  
Context Graph  
      ↓  
Result

---

# 10. Performance Strategy

Important optimization:

**Do not rebuild the graph per action.**

Instead:

Base graph → cached  
Context nodes → evaluated with parameters

Typical cost:

<100 microseconds

---

# 11. Context Types in 5e

Most systems end up with contexts like:

AttackContext  
DamageContext  
SpellCastContext  
SkillCheckContext  
SavingThrowContext

Example:

pub enum RuleContext<'a> {  
    Attack(AttackContext<'a>),  
    Damage(DamageContext<'a>),  
    Spell(SpellContext<'a>),  
    Skill(SkillContext),  
}

---

# 12. Attack Evaluation Flow

Example: **Rogue attacking with a rapier**

Engine steps:

1. Build AttackContext  
   weapon = rapier  
  
2. Evaluate graph  
  
3. Determine ability modifier  
   finesse → max(str, dex)  
  
4. Apply proficiency  
  
5. Apply weapon bonus  
  
6. Apply archery / fighting styles  
  
7. Apply sneak attack if valid

Result:

attack_bonus = +8  
damage = 1d8 + 4 + 3d6

---

# 13. Inventory Interaction

Inventory feeds the context graph.

Example:

Weapon equipped:

rapier +1

Fragment:

rapier_attack_bonus = 1  
rapier_damage_bonus = 1

Condition:

ctx.weapon.id == rapier

---

# 14. Why Context Graphs Matter

Without them you end up writing logic like:

if weapon.is_finesse() {  
    ...  
}

scattered everywhere.

Context graphs keep rules **data-driven and composable**.

---

# 15. Final Engine Architecture

Putting everything together:

Game Data  
   races  
   classes  
   feats  
   items  
   spells  
  
Character State  
   abilities  
   inventory  
   conditions  
  
Rule Sources  
   ↓  
RuleFragments  
   ↓  
Typed Rule Graph  
   ↓  
Stacking Aggregators  
   ↓  
Base Character Graph  
   ↓  
Contextual Graph Evaluation  
   ↓  
Action Result

---

# 16. What This Enables

This architecture cleanly supports:

weapon attacks  
spell attacks  
saving throws  
skill checks  
damage rolls  
conditions  
temporary effects

all with the same engine.

---

If you'd like, I can also show the **complete Rust crate architecture** for a production-quality 5e rules engine (about 10 modules) that cleanly separates:

- rules engine
    
- game data
    
- character model
    
- inventory
    
- actions
    

This is usually the step where the whole design becomes much clearer.

please do

Below is a **clean, production-quality Rust crate architecture** for a 5e rules engine implementing everything we've discussed:

- trait-light **data-driven rules**
    
- **typed rule graphs**
    
- **stacking aggregators**
    
- **rule graph fragments**
    
- **contextual evaluation**
    
- **inventory + equipment**
    
- clean **domain / rules separation**
    

The goal is that:

- adding a new **spell/item/feat** requires **no engine code changes**
    
- rules remain **composable**
    
- evaluation remains **fast**
    

This is roughly the architecture used by serious RPG systems.

---

# 1. Top-Level Crate Layout

A typical project ends up around **8–12 modules**:

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

Important principle:

engine  ← reusable rule engine  
rules   ← 5e-specific rules  
domain  ← character + inventory

The **rules engine should know nothing about D&D.**

---

# 2. Engine Module (Generic Rule Graph)

This module implements the **typed dependency graph system**.

engine/  
├─ graph.rs  
├─ node.rs  
├─ value.rs  
├─ stack.rs  
├─ fragment.rs  
├─ evaluator.rs  
└─ context.rs

Responsibilities:

|File|Purpose|
|---|---|
|value.rs|typed rule values|
|node.rs|graph nodes|
|stack.rs|stacking rules|
|fragment.rs|rule graph fragments|
|graph.rs|dependency graph|
|context.rs|contextual evaluation|
|evaluator.rs|graph evaluation|

Example types:

pub struct RuleGraph {  
    nodes: HashMap<NodeId, Node>,  
    contributions: Vec<Contribution>,  
}

pub struct RuleFragment {  
    pub nodes: Vec<Node>,  
    pub constants: Vec<ConstantNode>,  
    pub contributions: Vec<Contribution>,  
}

---

# 3. Value System

Defined in:

engine/value.rs

Example:

#[derive(Clone, Debug)]  
pub enum Value {  
    Int(i32),  
    Bool(bool),  
    Dice(Dice),  
}

This is used by the entire rule engine.

---

# 4. Stacking Rules

Defined in:

engine/stack.rs

Example:

pub enum StackRule {  
    Sum,  
    Max,  
    Min,  
    Override,  
    BooleanOr,  
    BooleanAnd,  
}

Nodes declare stacking behavior:

pub struct Node {  
    pub id: NodeId,  
    pub stack: StackRule,  
    pub deps: Vec<NodeId>,  
}

---

# 5. Rule Fragments

Defined in:

engine/fragment.rs

Fragments allow each feature to inject rules.

pub struct RuleFragment {  
    pub nodes: Vec<Node>,  
    pub constants: Vec<ConstantNode>,  
    pub contributions: Vec<Contribution>,  
}

Example fragment for **Ring of Protection**:

RuleFragment {  
    constants: vec![  
        ConstantNode {  
            id: NodeId("ring_protection_bonus"),  
            value: Value::Int(1),  
        }  
    ],  
    contributions: vec![  
        Contribution {  
            target: NodeId("ac_bonus"),  
            source: NodeId("ring_protection_bonus"),  
        }  
    ],  
}

---

# 6. Rules Module (5e Mechanics)

This module defines **what nodes exist in the game**.

rules/  
├─ abilities.rs  
├─ combat.rs  
├─ armor_class.rs  
├─ skills.rs  
├─ saving_throws.rs  
└─ spellcasting.rs

Example:

rules/armor_class.rs

Defines nodes:

pub const AC_FORMULA: NodeId = NodeId("ac_formula");  
pub const AC_BONUS: NodeId = NodeId("ac_bonus");  
pub const AC: NodeId = NodeId("armor_class");

Graph nodes:

Node {  
    id: AC,  
    stack: StackRule::Override,  
    deps: vec![AC_FORMULA, AC_BONUS],  
}

Rules module defines **the base graph skeleton**.

Fragments attach to it.

---

# 7. Character Module

Domain model for characters.

character/  
├─ character.rs  
├─ ability_scores.rs  
├─ class_levels.rs  
├─ features.rs  
└─ conditions.rs

Example:

pub struct Character {  
    pub abilities: AbilityScores,  
    pub levels: Vec<ClassLevel>,  
    pub feats: Vec<FeatId>,  
    pub inventory: Inventory,  
}

Character produces **rule fragments**.

impl Character {  
    pub fn fragments(&self) -> Vec<RuleFragment> {  
        let mut frags = vec![];  
  
        frags.extend(self.abilities.fragments());  
        frags.extend(self.inventory.fragments());  
  
        frags  
    }  
}

---

# 8. Inventory Module

Handles items, containers, equipment.

inventory/  
├─ inventory.rs  
├─ item_instance.rs  
├─ equipment.rs  
└─ containers.rs

Example:

pub struct Inventory {  
    pub items: HashMap<ItemInstanceId, ItemInstance>,  
}

Item instances:

pub struct ItemInstance {  
    pub item_id: ItemId,  
    pub location: ItemLocation,  
    pub equipped: bool,  
}

Inventory emits fragments from **equipped items**.

---

# 9. Game Data Module

Loads game content.

data/  
├─ races.rs  
├─ classes.rs  
├─ feats.rs  
├─ spells.rs  
└─ items.rs

Example YAML:

id: ring_of_protection  
  
constants:  
  ring_protection_bonus: 1  
  
contributions:  
  - target: ac_bonus  
    source: ring_protection_bonus

Loader converts to `RuleFragment`.

---

# 10. Actions Module (Contextual Graphs)

Handles contextual calculations.

actions/  
├─ attack.rs  
├─ damage.rs  
├─ skill_check.rs  
├─ saving_throw.rs  
└─ spell_cast.rs

Defines contexts:

pub struct AttackContext<'a> {  
    pub weapon: &'a Weapon,  
}

Evaluation:

pub fn compute_attack(  
    engine: &RuleEngine,  
    ctx: AttackContext,  
) -> AttackResult

This runs the contextual graph.

---

# 11. Full Engine Pipeline

Character evaluation pipeline:

Game Data  
   ↓  
Character State  
   ↓  
Collect RuleFragments  
   ↓  
Merge Into RuleGraph  
   ↓  
Evaluate Base Graph  
   ↓  
Cached Character Stats

Action evaluation pipeline:

Action Requested  
   ↓  
Create RuleContext  
   ↓  
Evaluate Context Nodes  
   ↓  
Produce Result

---

# 12. Example End-to-End Flow

Player attacks with **Rapier +1**.

Engine steps:

1) Character fragments  
   race  
   class  
   feats  
   inventory  
  
2) Merge fragments into rule graph  
  
3) Evaluate base graph  
   ability scores  
   proficiency bonus  
   conditions  
  
4) Create AttackContext  
   weapon = rapier  
  
5) Evaluate contextual nodes  
   attack_bonus  
   damage

Result:

Attack bonus: +8  
Damage: 1d8 + 4 + 1

---

# 13. Why This Architecture Works

It keeps three systems cleanly separated.

### Rule engine (generic)

Handles:

dependency graphs  
stacking  
evaluation

---

### Game rules (5e)

Defines:

node types  
formulas  
mechanics

---

### Game data

Defines:

races  
classes  
spells  
items

No code changes needed.

---

# 14. Approximate Code Size

Typical project size:

|Module|Lines|
|---|---|
|engine|~1500|
|rules|~1500|
|character|~800|
|inventory|~600|
|actions|~800|
|data loaders|~600|

Total:

~5000–6000 lines

Small enough to maintain.

---

# 15. Biggest Advantage

Once built, adding rules becomes trivial.

Example new magic item:

Boots of Elvenkind

Add YAML:

advantage on stealth checks

Engine already understands it.

---

✅ If you'd like, I can also show the **three hardest 5e rule problems** that rule engines must solve correctly:

1. **AC formula conflicts** (armor vs Mage Armor vs Unarmored Defense)
    
2. **bonus stacking categories** (5e's subtle stacking rules)
    
3. **temporary effects system** (spells, buffs, durations)
    

These are the areas where most hobby rule engines break.