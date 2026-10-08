# Part 2
## Item Example: Ring of Protection

[[Part 2 Raw#10. This Enables Data-Driven Rules|Link]]
```YAML
id: ring_of_protection  
  
constants:  
  ring_protection_bonus: 1  
  
contributions:  
  - target: ac_bonus  
    source: ring_protection_bonus  
  - target: saving_throw_bonus  
    source: ring_protection_bonus
```

# Part 3
## Item Example: Longsword

[[Part 3 Raw#1. Rules Content (static)|Link]]
```YAML
id: longsword  
name: Longsword  
tags: [weapon, martial]  
weight: 3  
damage: 1d8  
properties: [versatile]
```
## Item Example: Potion of Healing

[[Part 3 Raw#7. Inventory Data in Large Systems|Link]]
```YAML
id: item_45  
definition: potion_healing  
stack: 3  
location: backpack
```
## Broader System

[[Part 3 Raw#1. Core Design Principle Structured Definitions with IDs|Link]] (extended)
### Spell Example: Fireball
```YAML
id: spell.fireball  
type: spell  
  
metadata:  
  name: Fireball  
  source: PHB  
  page: 241  
  
text:  
  summary: A bright streak flashes to a point you choose.  
  description: |  
    A bright streak flashes from your pointing finger  
    to a point you choose within range and then  
    blossoms with a low roar into an explosion of flame.  
  
mechanics:  
  level: 3  
  school: evocation  
  casting_time: action  
  range: 150 ft  
  
  components:  
    verbal: true  
    somatic: true  
    material:  
      description: bat guano and sulfur  
  
  damage:  
    dice: 8d6  
    type: fire  
    save: dexterity
```
### Effects
```YAML
# example 1
effects:  
  - type: modifier  
    target: stat.attack_bonus  
    value: 1
    
# example 2
effects:  
  - type: damage_bonus  
    trigger: heavy_weapon_attack  
    value: 10
```
### Human Text
```YAML
text:  
  summary: You have mastered heavy weapons.  
  description: |  
    Before you make a melee attack with a heavy weapon,  
    you can choose to take a -5 penalty to the attack roll.  
    If the attack hits, you add +10 to the damage.
```
### General Concepts
```YAML
# Source Tracking
metadata:  
  source: PHB  
  page: 165
  
# Tag Examples
tags:  
  - weapon  
  - martial  
  - melee
    
tags:  
  - potion  
  - consumable
    
# ID References
spell: spell.fireball
# in a feature:
grants:  
  spells:  
    - spell.fireball  
    - spell.magic_missile
      
# avoid deeply nested structures (may be unavoidable though)
```



# Part 4
## Class Example: Wizard

[[Part 4 Raw#1. Class Progression as a Level Table|Link]]
### Class
```YAML
id: class.wizard  
type: class  
  
metadata:  
  name: Wizard  
  source: PHB  
  
mechanics:  
  hit_die: d6  
  
  progression:  
    1:  
      proficiency_bonus: 2  
      features:  
        - feature.spellcasting  
        - feature.arcane_recovery  
  
    2:  
      features:  
        - feature.arcane_tradition  
  
    3: {}  
  
    4:  
      features:  
        - feature.ability_score_improvement
```

### Class Features
Class features should be defined separately
```YAML
id: feature.arcane_recovery  
type: feature  
  
metadata:  
  name: Arcane Recovery  
  source: PHB  
  
text:  
  description: |  
    Once per day when you finish a short rest...  
  
mechanics:  
  recovery:  
    spell_slot_levels: half_wizard_level
```

### Spell Slot Progression
Use an explicit table:
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

### Spell Lists
Class references spell list which is defined separately
```YAML
spell_list: spell_list.wizard

# list definition
id: spell_list.wizard  
type: spell_list  
  
spells:  
  - spell.fireball  
  - spell.magic_missile  
  - spell.detect_magic
```

### Subclasses
```YAML
id: subclass.evocation  
type: subclass  
  
parent_class: class.wizard  
  
mechanics:  
  
  progression:  
  
    2:  
      features:  
        - feature.evocation_savant  
  
    6:  
      features:  
        - feature.potent_cantrip
```

## Spell Upcasting

[[Part 4 Raw#7. Example Cure Wounds|Link]]
### Example: Fireball
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

### Example: Cure Wounds
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

### Scaling Mode: Cantrip Example
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

## Effects: Stat Paths, Modifier Operations, Activation Conditions

[[Part 4 Raw#1. Stat Paths (Structured Stat Namespace)|Link]]
### Stat Path Effects
Use namespaced stats.
```YAML
# Example: ring of protection armor bonus
effect:  
  type: modifier  
  target: defense.armor_class  
  value: 1
# Example: Barkskin
target: defense.armor_class  
operation: override  
value: 16
```

### Modifier Operations
Support operations for add, multiply (e.g. *haste* speed x2), set minimum (e.g. *barkskin* AC), set maximum, override (e.g. *mage armor* AC), and grant (e.g. proficiency with heavy armor).
```YAML
# Minimum
operation: minimum  
target: defense.armor_class  
value: 16

# Override
operation: override  
target: defense.base_ac  
formula: 13 + ability.dexterity.mod
```

### Activation Conditions
```YAML
# Example: GWM damage bonus
effect:  
  operation: add  
  target: combat.damage_bonus  
  value: 10  
  
condition:  
  weapon_tag: heavy
  
# Example: attunement
condition:  
  attuned: true
```

### General Examples
```YAML
# Ring of Protection
effects:  
  - operation: add  
    target: defense.armor_class  
    value: 1  
  
  - operation: add  
    target: defense.saving_throw.all  
    value: 1
    
# Bless
effects:  
  - operation: add_dice  
    target: combat.attack_roll  
    dice: 1d4  
  
  - operation: add_dice  
    target: defense.saving_throw  
    dice: 1d4
    
# Poisoned
effects:  
  - operation: disadvantage  
    target: combat.attack_roll
```

[[Part 4 Raw#1. Minimal Spell Upcasting Representation|Magic Missile Example]]
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

[[Part 4 Raw#5. Two Extra Upcast Patterns Worth Supporting|Upcast Patterns]]
```YAML
# die scaling
effects:  
  - heal:  
      dice: "1d8"  
      ability: spellcasting  
  
upcast:  
  per_level:  
    add_dice: "1d8"
    
# target scaling
upcast:  
  per_level:  
    additional_targets: 1
```

# Part 5
## Rule Structure and Expression

[[Part 5 Raw#2. Shared Rule Structures (YAML)|Link]]
```YAML
Rule:  
  id: string  
  phase: base | override | modifier | derived | finalize  
  target: stat_id  
  operation: operation_type  
  value: expression  
  category: optional_bonus_category  
  condition: optional_condition

# Example
- id: ring_protection_ac  
  phase: modifier  
  target: armor_class  
  operation: add  
  value: 1  
  category: item
```

Expressions
```YAML
value:  
  expr: add  
  args:  
    - stat: dex_mod  
    - 13
      
# Example
value:  
  expr: mul  
  args:  
    - stat: druid_level  
    - 3
```

Rule conditions
```YAML
condition:  
  has_condition: raging
  
condition:  
  equipped: shield
```

## Class Example: Fighter

[[Part 5 Raw#3. Classes|Link]]
### Class
```YAML
id: fighter  
type: class  
edition: [2014, 2024]  
  
hit_die: d10  
primary_abilities: [strength, dexterity]  
  
proficiencies:  
  armor: [light, medium, heavy, shield]  
  weapons: [simple, martial]  
  saving_throws: [strength, constitution]  
  
levels:  
  
  1:  
    features:  
      - fighting_style  
      - second_wind  
  
  2:  
    features:  
      - action_surge  
  
  3:  
    subclass: fighter_subclass
```

### Subclasses
```YAML
id: champion  
type: subclass  
class: fighter  
  
features:  
  
  3:  
    - improved_critical  
  
  7:  
    - remarkable_athlete  
  
  10:  
    - additional_fighting_style
```

## Feats and Features

[[Part 5 Raw#5. Features|Link]]
```YAML
# Defense Fighting Style
id: fighting_style_defense  
type: feature  
  
rules:  
  
  - id: defense_ac  
    phase: modifier  
    target: armor_class  
    operation: add  
    value: 1  
    category: fighting_style
```

```YAML
# Sharpshooter
id: sharpshooter  
type: feat  
  
prerequisites:  
  - proficiency: martial_weapons  
  
rules:  
  
  - id: sharpshooter_range  
    phase: modifier  
    target: ranged_attack_ignore_cover  
    operation: set  
    value: true  
  
  - id: sharpshooter_power_attack  
    phase: modifier  
    target: ranged_attack_bonus  
    operation: add  
    value: -5  
  
description: |  
  You have mastered ranged weapons...
```

## Race and Background

[[Part 5 Raw#7. Species (Race / Species System)|Link]]
```YAML
id: dwarf_hill  
type: species  
  
ability_bonuses:  
  constitution: 2  
  wisdom: 1  
  
speed: 25  
  
features:  
  - darkvision  
  - dwarven_resilience
```

```YAML
id: acolyte  
type: background  
  
skill_proficiencies:  
  - insight  
  - religion  
  
languages:  
  - choice: 2  
  
features:  
  - shelter_of_the_faithful
```

## Spells and Upcasting

[[Part 5 Raw#9. Spells|Link]]
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
```

Upcasting
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

## Items and Inventory

[[Part 5 Raw#11. Items|Link]]
### Magic Item
```YAML
id: ring_of_protection  
type: item  
  
rarity: rare  
requires_attunement: true  
  
rules:  
  
  - id: ring_ac  
    phase: modifier  
    target: armor_class  
    operation: add  
    value: 1  
    category: item  
  
  - id: ring_saves  
    phase: modifier  
    target: saving_throw_bonus  
    operation: add  
    value: 1
```

### Weapons
```YAML
id: longsword  
type: weapon  
  
damage: 1d8  
damage_type: slashing  
  
properties:  
  - versatile  
  
rules: []
```

### Armor
```YAML
id: chain_mail  
type: armor  
  
base_ac: 16  
  
rules:  
  
  - id: armor_base  
    phase: base  
    target: armor_class  
    operation: set  
    value: 16
```

### Containers
```YAML
id: backpack  
type: container  
  
capacity:  
  weight: 30
```

## Conditions
[[Part 5 Raw#15. Conditions|Link]]
```YAML
id: prone  
type: condition  
  
rules:  
  
  - id: prone_attack_penalty  
    phase: modifier  
    target: attack_roll  
    operation: add  
    value: -2  
  
  - id: prone_advantage_melee  
    phase: modifier  
    target: melee_attack_advantage  
    operation: set  
    value: true
```

## 2024 Wildshape

[[Part 5 Raw#16. Wild Shape Forms (2024)|Link]]
```YAML
id: bear_form  
type: form  
  
size: large  
  
rules:  
  
  - id: bear_temp_hp  
    phase: modifier  
    target: temporary_hp  
    operation: add  
    value:  
      expr: mul  
      args:  
        - stat: druid_level  
        - 3  
  
  - id: bear_climb  
    phase: base  
    target: climb_speed  
    operation: set  
    value: 40
```

## Derived Stats

[[Part 5 Raw#Derived Stat Graph|Link]]

Ability Modifier
```YAML
- id: ability_mod  
  phase: derived  
  target: strength_mod  
  operation: set_expr  
  value:  
    expr: floor_div  
    args:  
      - expr: sub  
        args: [stat: strength, 10]  
      - 2
```

Proficiency Bonus
```
- id: proficiency_bonus  
  phase: derived  
  target: proficiency_bonus  
  operation: set_expr  
  value:  
    expr: add  
    args:  
      - 2  
      - expr: floor_div  
        args:  
          - expr: sub  
            args: [stat: level, 1]  
          - 4
```

Stealth
```YAML
target: skill_stealth  
operation: set_expr  
value:  
  expr: add  
  args:  
    - stat: dexterity_mod  
    - stat: stealth_proficiency_bonus
```

Passive Perception
```YAML
target: passive_perception  
operation: set_expr  
value:  
  expr: add  
  args:  
    - 10  
    - stat: skill_perception
```

Spell Save DC
```YAML
target: spell_save_dc  
operation: set_expr  
value:  
  expr: add  
  args:  
    - 8  
    - stat: proficiency_bonus  
    - stat: spellcasting_ability_mod
```

## Homebrew Package Formatting

[[Part 5 Raw#Homebrew Package Format|Link]]
```YAML
package:  
  name: "Arcanist Expansion"  
  version: "1.0"  
  author: "Jane Doe"  
  
content:  
  
  spells:  
    # ...
  
  feats:  
    # ... 
  
  items:  
    # ...
```

# Part 6
## Full Expression System
[[Part 6 Raw#2. Expression Syntax in YAML|Link]]

Structure
```YAML
expr: operation  
args: [...]

# Example
value:  
  expr: add  
  args:  
    - stat: dexterity_mod  
    - 13
```
Support arithmetic operations add, sub, mul, div, floor_div, min, max, clamp. 

Conditional
```YAML
# Equivalent to ternary raging ? 2 : 0
expr: if  
args:  
  - condition:  
      has_condition: raging  
  - 2  
  - 0
```

Dice
```YAML
# 1d8
expr: dice  
args:  
  - 1  
  - 8
    
# upcast usage
expr: dice  
args:  
  - expr: add  
    args:  
      - 1  
      - stat: spell_slot_level  
  - 8
```

Stat Reference
```YAML
expr: add  
args:  
  - stat: strength_mod  
  - stat: proficiency_bonus
```

### Examples

Mage Armor
```YAML
rules:  
  - target: armor_class  
    phase: override  
    operation: max_expr  
    value:  
      expr: add  
      args:  
        - 13  
        - stat: dexterity_mod
```

Proficiency Bonus
```YAML
expr: add  
args:  
  - 2  
  - expr: floor_div  
    args:  
      - expr: sub  
        args:  
          - stat: level  
          - 1  
      - 4
```

2024 Wildshape Temp HP
```YAML
expr: mul  
args:  
  - stat: druid_level  
  - 3
```

Spell Upcast
```YAML
expr: dice  
args:  
  - expr: add  
    args:  
      - 1  
      - stat: upcast_levels  
  - 8
```

### Half-Full Example: Rage Damage Bonus
```YAML
id: barbarian_rage  

rules:  
  
  - target: melee_damage_bonus  
    phase: modifier  
    operation: add  
    value:  
      expr: if  
      args:  
        - condition:  
            has_condition: raging  
        - expr: max  
          args:  
            - 2  
            - expr: floor_div  
              args:  
                - stat: level  
                - 9  
        - 0
```

## More Examples

[[Part 6 Raw#1. YAML Rules (Data Layer)|Link]]

Chain Mail
```YAML
id: chain_mail  
type: armor  
  
rules:  
  - phase: base  
    target: armor_class  
    operation: set  
    value: 16
```

Defense Fighting Style
```YAML
id: fighting_style_defense  
  
rules:  
  - phase: modifier  
    target: armor_class  
    operation: add  
    value: 1  
    category: fighting_style
```

Ring of Protection
```YAML
id: ring_of_protection  
  
rules:  
  - phase: modifier  
    target: armor_class  
    operation: add  
    value: 1  
    category: item
```

Shield Spell
```YAML
id: shield  
  
rules:  
  - phase: modifier  
    target: armor_class  
    operation: add  
    value: 5  
    category: spell
```

Dexterity Modifier (Derived)
```YAML
id: dexterity_mod  
  
rules:  
  - phase: derived  
    target: dexterity_mod  
    operation: set_expr  
    value:  
      expr: floor_div  
      args:  
        - expr: sub  
          args:  
            - stat: dexterity  
            - 10  
        - 2
```

## Wildshape Selector

[[Part 6 Raw#3. Add a Selection Rule|Link]]
```YAML
rules:  
  - type: grant_form_selection  
    system: wildshape  
    count: 4  
    filter:  
      type: beast  
      max_cr: "@druid.wildshape_cr"
```

## Full(er) Examples

[[Part 6 Raw#1. Rage (Barbarian)|Link]]

### Barbarian Rage
```YAML
id: barbarian_rage  
source:  
  type: class_feature  
  class: barbarian  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_resource  
      resource: rage  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: rage_activate  
  
  - phase: conditions  
    filter:  
      condition_active: raging  
    effect:  
      type: modifier  
      target: damage_roll  
      operation: add  
      value: "@barbarian.rage_damage"  
  
  - phase: conditions  
    filter:  
      condition_active: raging  
    effect:  
      type: modifier  
      target: defense  
      stat: damage_resistance  
      value: [bludgeoning, piercing, slashing]
```

### Sneak Attack
```YAML
id: rogue_sneak_attack  
source:  
  type: class_feature  
  class: rogue  
  
rules:  
  
  - phase: actions  
    filter:  
      action_type: attack  
      weapon_property: finesse_or_ranged  
  
    effect:  
      type: modifier  
      target: damage_roll  
      operation: add_dice  
      value: "@rogue.sneak_attack_dice"
```

### Wizard Spellcasting
```YAML
id: wizard_spellcasting  
source:  
  type: class_feature  
  class: wizard  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_resource  
      resource: spell_slots  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_spell_list  
      spell_list: wizard  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_choice  
      choice:  
        type: spell_selection  
        list: wizard  
        count: "@wizard.starting_spells"
```

### Fighting Style
```YAML
id: fighter_fighting_style  
source:  
  type: class_feature  
  class: fighter  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_choice  
      choice:  
        type: fighting_style  
        options:  
          - archery  
          - defense  
          - dueling  
          - great_weapon_fighting
            
# Example option YAML
id: fighting_style_archery  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: modifier  
      target: attack_roll  
      weapon_category: ranged  
      operation: add  
      value: 2
```

### Magic Missile
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
  per_slot_level:  
    missiles: +1
```

### 2024 Wildshape
```YAML
id: druid_wildshape  
source:  
  type: class_feature  
  class: druid  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_resource  
      resource: wildshape_uses  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: wildshape_transform  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_form_selection  
      system: wildshape  
      count: "@druid.wildshape_known_forms"  
      filter:  
        creature_type: beast  
        max_cr: "@druid.wildshape_cr"
```

### Weapon Mastery
```YAML
id: fighter_weapon_mastery  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: grant_choice  
      choice:  
        type: weapon_mastery  
        count: "@fighter.mastery_count"
```

Example mastery:
```YAML
id: weapon_mastery_cleave  
  
rules:  
  
  - phase: actions  
    filter:  
      weapon_property: heavy  
    effect:  
      type: grant_action  
      action: cleave_attack
```

### Sharpshooter Feat
```YAML
id: feat_sharpshooter  
  
rules:  
  
  - phase: feature_modifiers  
    effect:  
      type: modifier  
      target: ranged_attack_cover  
      operation: ignore  
  
  - phase: actions  
    effect:  
      type: grant_action  
      action: sharpshooter_power_attack
```

# Part 7
## Module Metadata
```YAML
module:  
  id: xgte  
  name: Xanathar's Guide to Everything  
  version: 1.0  
  
requires:  
  - phb  
  
optional:  
  - tcoe
```
