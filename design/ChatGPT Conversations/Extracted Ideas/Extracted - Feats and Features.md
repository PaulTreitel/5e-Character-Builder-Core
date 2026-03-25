# Links

### Part 1
1. [[Part 1 Raw#8. Feature Graph (Very Powerful)|Feature struct]]
2. [[Part 1 Raw#5. Feature Table|Feature SQL]]
3. [[Part 1 Raw#10. Character Feature Table|Character feature SQL]]
4. [[Part 1 Raw#3. The Feature System (Where Homebrew Lives)|Feature struct]] plus [[Part 1 Raw#5. Why This Works for Homebrew|JSON example]]
5. [[Part 1 Raw#4. Features|Another couple structs]]
6. [[Part 1 Raw#7. Feature Resolver|Feature collection]]

### Part 2
1. [[Part 2 Raw#6. Features (The Core Rule Unit)|Another struct]] and feature collection

### Part 4
1. [[Part 4 Raw#2. Class Features as Separate Definitions|Class feature YAML]] and more
2. [[Part 4 Raw#Granting capabilities|Example feats]]
3. [[Part 4 Raw#4. Feat Category Split (2024)|2024 Feat Categories]] (also 8 and 9)

### Part 5
1. [[Part 5 Raw#5. Skill Replacement Rules|Skill replacement choices]]
2. [[Part 5 Raw#5. Features|Feat and feature YAML]]

### Part 6
1. [[Part 6 Raw#4. Fighting Style Choice|Fighting style YAML]] (also 9)
2. [[Part 6 Raw#14. Full Rule Example Using Expressions|Rage feature YAML]]

# Common Data Structure Elements

### struct Feature
- ID, name, description, source
- effects: `Vec<Effect>`
- a `FeatCategory`

### enum FeatCategory
- origin, general, fighting style, epic boon

### Loading Feature Data
- must be able to use a wide and complex mechanical expression system
	- interact with rest system, grant limited resources, grant actions, affect attack information, grant proficiencies, etc
	- be able to do those things conditionally as well
	- in other words use the entire `RuleEffect` system in its data representation and loading
	- also things like "grant skill unless already granted, then select another"


# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
struct Feature {
    id: String,
    effects: Vec<Effect>,
    grants: Vec<String>,
}
```

#### #2
```SQL
features
-------------
id TEXT PRIMARY KEY
pack_id TEXT
name TEXT
description TEXT
data JSON
```

#### #3
```SQL
character_features
-------------
character_id TEXT
feature_id TEXT
source TEXT
```

#### #4
```Rust
pub struct Feature {
    pub id: FeatureId,
    pub name: String,
    pub description: String,

    pub effects: Vec<Effect>,
}
```

```JSON
{
  "name": "Frostborn",
  "effects": [
    { "type": "resistance", "damage": "cold" },
    { "type": "spell", "spell": "armor_of_agathys" }
  ]
}
```

#### #5
```Rust
pub struct Feature {
    pub id: String,
    pub effects: Vec<Effect>,
}

pub struct Feat {
    pub id: String,
    pub features: Vec<String>,
}
```

### Part 2

#### #1
```Rust
pub struct Feature {  
    pub id: FeatureId,  
    pub effects: Vec<Effect>,  
}
```

### Part 4

#### #1
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

more YAML descriptions of features (see link)

#### #2
heavy armor proficiency feat
```YAML
operation: grant  
target: proficiency.armor.heavy  
value: true
```

GWM bonus
```YAML
effect:  
  operation: add  
  target: combat.damage_bonus  
  value: 10  
  
condition:  
  weapon_tag: heavy
```

#### #3
```Rust
pub enum FeatCategory {  
    Origin,  
    General,  
    FightingStyle,  
    EpicBoon,  
}

pub struct FeatDefinition {  
    pub id: FeatId,  
    pub category: FeatCategory,  
    pub rules: Vec<RuleEffect>,  
}
```

### Part 5

#### #1
```Rust
FeatureChoice {  
    feature_id: "background_skill",  
    selected_skill: Skill::Stealth  
}
```

#### #2
Defense fighting style
```YAML
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

Sharpshooter
```YAML
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

### Part 6

#### #1
Fighter fighting style choice
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

Archery fighting style
```YAML
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

Sharpshooter
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

#### #2
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

# (Pseudo)Code

### Part 1

#### #6
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

### Part 2

#### #1
```Rust
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
```
