# Links

### Part 1
1. [[Part 1 Raw#2. Recommended Domain Model|Class levels]]
2. [[Part 1 Raw#7. Class Example|Class struct, JSON, and subclass]]
3. [[Part 1 Raw#9. Database Design|Class SQL]]
4. [[Part 1 Raw#6. Rule Database|Class struct]]

### Part 2
1. [[Part 2 Raw#3. Core Domain Types (Strongly Typed 5e)|Class levels]]
2. [[Part 2 Raw#8. Rule Database|Class struct]]

### Part 4
1. [[Part 4 Raw#1. Class Progression as a Level Table|Class progression YAML]]
2. [[Part 4 Raw#2. Class Features as Separate Definitions|Class feature YAML]]
3. [[Part 4 Raw#4. Spell List Representation|Spell list]]
4. [[Part 4 Raw#5. Subclasses|Subclass YAML]]
5. [[Part 4 Raw#2. Character Model|Class level]]

### Part 5
1. [[Part 5 Raw#3. Classes|Class and subclass YAML]]

### Part 6
1. [[Part 6 Raw#19. Where Rules Live|Class feature]]
2. [[Part 6 Raw#1. Rage (Barbarian)|Several class feature YAML example]]

### Part 7
1. [[Part 7 Raw#4. Class Levels (Multiclass Support)|Class SQL]]

# Common Data Structure Elements

### struct Class
- id and name
- hit die
- level features: `HashMap<u8, Vec<FeatureId>>`

### struct ClassLevel
- class ID
- level integer
- optional subclass

### struct Subclass
- class and subclass IDs
- level features just like classes

# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
pub struct ClassLevel {
    pub class: ClassId,
    pub level: u8,
    pub subclass: Option<SubclassId>,
}
```

#### #2
```Rust
pub struct Class {
    pub id: ClassId,
    pub name: String,

    pub hit_die: u8,
    pub primary_abilities: Vec<Ability>,

    pub level_features: HashMap<u8, Vec<FeatureId>>,
}
```

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

```Rust
pub struct Subclass {
    pub id: SubclassId,
    pub class: ClassId,

    pub level_features: HashMap<u8, Vec<FeatureId>>,
}
```

#### #3
```SQL
character_classes
-----------------
character_id
class_id
level
subclass_id
```

#### #4
```Rust
pub struct Class {
    pub id: String,
    pub level_features: HashMap<u8, Vec<String>>,
}
```

### Part 2

#### #1
```Rust
pub struct ClassLevel {  
    pub class: ClassId,  
    pub level: u8,  
    pub subclass: Option<SubclassId>,  
}
```

#### #2
```Rust
pub struct Class {  
    pub id: ClassId,  
    pub hit_die: u8,  
    pub level_features: HashMap<u8, Vec<FeatureId>>,  
}
```



### Part 4

#### #1
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

#### #2
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

#### #3
```YAML
id: spell_list.wizard  
type: spell_list  
  
spells:  
  - spell.fireball  
  - spell.magic_missile  
  - spell.detect_magic
```

#### #4
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

#### #5

```Rust
pub struct ClassLevel {  
    pub class: ClassId,  
    pub level: u8,  
}
```

### Part 5

#### #1
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

### Part 6

#### #1
```Rust
pub struct ClassFeature {  
    pub id: FeatureId,  
    pub rules: Vec<Rule>,  
}
```

#### #2
Barbarian rage
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

Sneak attack
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

Wizard spellcasting
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

Fighter fighting style
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
```

Wildshape
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

### Part 7

#### #1
```SQL
CREATE TABLE character_classes (  
    character_id INTEGER,  
    class_id INTEGER,  
    level INTEGER,  
    PRIMARY KEY(character_id, class_id)  
);
```
