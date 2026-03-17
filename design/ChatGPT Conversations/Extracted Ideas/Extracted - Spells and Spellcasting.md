
# Links

### Part 1
- [[Part 1 Raw#***How does this change with a spellcasting system as well as tracking resource usage such as current HP, spent hit dice, used spell slots, and so on?***|Original question for class spellcasting system]]

### Part 2
- [[Part 2 Raw#5. Runtime State (Mutable Gameplay Data)|Character Runtime State]] (and headings 7, 8, 11)
- [[Part 2 Raw#12. Spellcasting Model|Spellcasting Model]] (and heading 13)

### Part 4
- [[Part 4 Raw#3. Spellcasting Progression Tables|Spell Slot Progression Table]] (and spell list representation)
- [[Part 4 Raw#6. Representing Spell Upcasting|Spell Upcasting]]
- [[Part 4 Raw#Please do [Stat Namespacing], but be brief about it|More Upcasting]]
- [[Part 4 Raw#5. Spell System|Spell System Structs]]
- (potential) [[Part 4 Raw#2. SpellcastingSystem Trait|SpellcastingSystem Trait]]
	- [[Part 4 Raw#6. Why Traits Make Sense Here|Why Trait Might be Worth it]]
- [[Part 4 Raw#4. Data Can Also Use Capabilities|Spells in Capacity vs Edition]]

### Part 5
- [[Part 5 Raw#9. Spells|Spell YAML]]

### Part 6
- Several small YAML spell examples embedded in other stuff (mostly the YAML expression engine)
- [[Part 6 Raw#5. Magic Missile (Spell with Upcasting)|Magic Missile YAML Example]]

### Part 7
- [[Part 7 Raw#7. Spellbook / Known Spells|Spellbook SQL]]

# Common Data Structure Elements

### structs Spell and SpellDefinition
- namespaced identifier (probably a `NamespacedId` or `StringId` or `StaticId` or whatever I end up calling it)
- name (string)
- spell level (int)
- spell school (probably an enum `SpellSchool`)
- description
- cast time (needs an enum)
- duration (needs an enum)
- record the spell effects as a `Vec<RuleEffect>` or similar
- also need upcasting representation
	- `Vec<UpcastEntry>` where the entry just records effects for a single slot level
	- an `UpcastRule` enum representing different types of scaling (adding dice, adding targets, etc)
- should special information be represented? Would also need expression in the upcasting system
	- like number of missiles in *Magic Missile*

### struct Spellcasting
- spellcasting ability
- slot table?
- spell list
- number of prepared (or known?) spells
- list of prepared (or known?) spells

### structs for Upcasting
- separate entries for each slot level
	- slot level `-> Vec<some upcast effect>` table, effectively
	- could be `RuleEffect`, could be a specific spell upcast type, like below
- enum for types of upcasting
	- add dice, add targets, add effect
- upcasting rules also need to be expressible in the YAML data representation, ideally without a crazy complexity/nested depth
	- likely ties into an overall expression language in the YAML
	- could structure it as like `add_dice: "1d8"` which then needs to be interpreted by the rules loader, or express scaling dice as a nested `dice: / number: 1 / size: 8`

### structs for Prepared/Known Spells
- need to support 3 variants:
	- prepared with full list known (e.g., druid, paladin, cleric)
	- prepared = known (e.g., warlock, bard)
	- prepared $\subseteq$ known (wizard)
- This can probably be done by maintaining just prepared and known lists
	- for cleric etc the known list is auto-filled with the full list
	- for warlock, bard, either fill the two with the same spells or ignore one (e.g. simply treat spells known as spells prepared and ignore the spells prepared field)
	- for the wizard, maintain both
- prepared spell could also potentially represent a spell with possible upcast effects loaded in

### struct ActiveSpell
- represent a spell that is active on the character?
- perhaps belongs as part of another system, like rule effects with sources?

### Spell Slot Table
- need 2 systems, one for recording the overall slot progression table for a class and one for recording the character's actual spell slot table, which is a function of multiclassing rules as well
- there are 4 general spell slot tables, fullcaster, half-caster, third-caster, and warlock

### trait SpellcastingSystem
- functions `can_cast()`, `cast_spell()`, and `collect_spell_effects()`
- should be designed to support multiple spellcasting systems like the spell points variant

# Extracted Data Structures and Representations

### Part 1

#### Sheet Tracking

```Rust
pub struct CharacterSheet {
	// ...
	pub spellcasting: Option<Spellcasting>,
	// ...
}

pub struct CharacterState {
	// ...
	pub spell_slots_used: HashMap<u8, u8>,
	pub prepared_spells: Vec<String>,
	// ...
}
```

#### Spellcasting

```Rust
pub struct Spellcasting {
    pub ability: Ability,

    pub slots: SpellSlotTable,

    pub prepared_spells: usize,

    pub spell_list: String,
}

pub struct SpellSlotTable {
    pub slots: HashMap<u8, u8>,
}
```

#### Spells

```Rust
pub struct Spell {
    pub id: String,
    pub level: u8,
    pub school: String,
}
```

### Part 2

#### Spellcasting

```Rust
pub struct Spellcasting {  
    pub ability: Ability,  
    pub spell_list: SpellListId,  
    pub slots: SpellSlotTable,  
}

pub struct SpellSlotTable {  
    pub slots: HashMap<u8, u8>  
}
```

### Part 4

#### Progression Table

index = spell level, value = number of slots

```YAML
spellcasting:  
  ability: intelligence  
  progression:  
    1:  
      slots: [2]  
    2:  
      slots: [3]  
    3:  
      slots: [4,2]  
    4:  
      slots: [4,3]  
    5:  
      slots: [4,3,2]
```

#### Spell List

```YAML
id: spell_list.wizard  
type: spell_list  
  
spells:  
  - spell.fireball  
  - spell.magic_missile  
  - spell.detect_magic
```

#### Spell Upcasting

*Fireball* example

```YAML
id: spell.fireball  
type: spell  
metadata:  
  name: Fireball  
mechanics:  
  level: 3  
  damage:  
    dice: 8d6  
    type: fire  
  scaling:  
    upcast:  
      mode: spell_level  
      effect:  
        damage:  
          add_dice: 1d6
```

*Cure Wounds* example

```YAML
mechanics:  
  level: 1  
  healing:  
    dice: 1d8  
    ability_modifier: true  
  scaling:  
    upcast:  
      mode: spell_level  
      effect:  
        healing:  
          add_dice: 1d8
```

Cantrip scaling

```YAML
scaling:  
  cantrip:  
    table:  
      5:  
        damage: +1d10  
      11:  
        damage: +2d10  
      17:  
        damage: +3d10
```

#### Spell Definition

```Rust
pub struct SpellDefinition {  
    pub id: SpellId,  
    pub name: String,  
    pub base_level: u8,  
    pub school: SpellSchool,  
  
    pub description: String,  
  
    pub effects: Vec<RuleEffect>,  
  
    pub upcast: Vec<UpcastEntry>,  
}  
  
pub struct UpcastEntry {  
    pub slot_level: u8,  
    pub additional_effects: Vec<RuleEffect>,  
}
```

YAML example

```YAML
id: magic_missile  
name: Magic Missile  
base_level: 1  
  
effects:  
  - damage:  
      dice: "1d4+1"  
      missiles: 3  
  
upcast:  
  - slot_level: 2  
    additional_effects:  
      - add_missiles: 1  
  
  - slot_level: 3  
    additional_effects:  
      - add_missiles: 2
```

prepared spell (used to represent an upcast spell)

```Rust
pub struct PreparedSpell {  
    pub spell_id: SpellId,  
    pub slot_level: u8,  
    pub resolved_effects: Vec<RuleEffect>,  
}
```

#### Upcast Patterns

YAML examples (*Cure Wounds* and *Hold Person*)

```YAML
effects:  
  - heal:  
      dice: "1d8"  
      ability: spellcasting  
  
upcast:  
  per_level:  
    add_dice: "1d8"
```

```YAML
upcast:  
  per_level:  
    additional_targets: 1
```

structs

```Rust
pub enum UpcastRule {  
    AddDice { dice: Dice },  
    AddTargets(u32),  
    AddEffect(RuleEffect),  
}  
  
pub struct SpellUpcast {  
    pub slot_level: u8,  
    pub rules: Vec<UpcastRule>,  
}
```

#### Spell Definition, Upcast, and Active Spells

```Rust
pub struct SpellDefinition {  
    pub id: SpellId,  
    pub name: String,  
  
    pub base_level: u8,  
  
    pub description: String,  
  
    pub effects: Vec<RuleEffect>,  
  
    pub upcast: Vec<SpellUpcast>,  
}

pub struct SpellUpcast {  
    pub slot_level: u8,  
    pub additional_effects: Vec<RuleEffect>,  
}

pub struct ActiveSpell {  
    pub spell_id: SpellId,  
    pub slot_level: u8,  
    pub caster: CharacterId,  
}
```

#### SpellcastingSystem Trait

For validating casting, handling slots, creating active spells, applying upcasting, and general spell rule effects. Also useful for variant spellcasting (e.g., spell points system)

```Rust
pub trait SpellcastingSystem {  
    fn can_cast(  
        &self,  
        character: &Character,  
        spell: SpellId,  
        slot_level: u8,  
        db: &RulesDatabase,  
    ) -> bool;  
  
    fn cast_spell(  
        &self,  
        character: &mut Character,  
        spell: SpellId,  
        slot_level: u8,  
        db: &RulesDatabase,  
    ) -> ActiveSpell;  
  
    fn collect_spell_effects(  
        &self,  
        character: &Character,  
        db: &RulesDatabase,  
    ) -> Vec<RuleEffect>;  
}
```

### Part 5: YAML Examples

Spell

```YAML
id: mage_armor  
type: spell  
  
level: 1  
school: abjuration  
  
casting_time: action  
duration: 8h  
  
rules:  
  
  - id: mage_armor_ac  
    phase: override  
    target: armor_class  
    operation: max_expr  
    value:  
      expr: add  
      args:  
        - 13  
        - stat: dex_mod
```

Spell upcasting

```YAML
id: cure_wounds  
type: spell  
  
level: 1  
  
damage:  
  dice: 1d8  
  modifier: spellcasting_mod  
  
upcast:  
  
  per_level:  
    damage_dice: 1d8
```

### Part 6: YAML Examples

There are several examples embedded throughout

#### Spell with Scaling: *Magic Missile*

```YAML
id: magic_missile  
  
level: 1  
school: evocation  
  
rules:  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: cast_magic_missile  
  
scaling:  
  
  upcast:  
  
    - level: 2  
      effect:  
        missiles: +1  
  
    - level: 3  
      effect:  
        missiles: +2
```

alternate scaling representation

```YAML
scaling:  
  per_slot_level:  
    missiles: +1
```

### Part 7: SQL

```SQL
CREATE TABLE character_spells (  
    character_id INTEGER,  
    spell_id INTEGER,  
    prepared BOOLEAN  
);
```

# (Pseudo)Code

### Part 1

```Rust
fn remaining_slots(sheet: &CharacterSheet, state: &CharacterState, level: u8) -> u8 {
    sheet.spellcasting
        .as_ref()
        .unwrap()
        .slots
        .slots[&level] - state.spell_slots_used.get(&level).copied().unwrap_or(0)
}
```

### Part 2

```Rust
impl CharacterState {  
	  
	// ...
	  
    pub fn spend_spell_slot(&mut self, level: u8) {  
        *self.spell_slots_used.entry(level).or_insert(0) += 1;  
    }  
}
```

### Part 4
```Rust
fn build_spell_effects(  
    spell: &SpellDefinition,  
    slot_level: u8  
) -> Vec<RuleEffect> {  
    let mut effects = spell.effects.clone();  
    for up in &spell.upcast {  
        if up.slot_level <= slot_level {  
            effects.extend(up.additional_effects.clone());  
        }  
    }  
    effects  
}
```