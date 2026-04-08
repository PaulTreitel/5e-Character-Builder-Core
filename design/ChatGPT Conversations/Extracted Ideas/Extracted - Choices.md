# Links

### Part 1
1. [[Part 1 Raw#8. Choice System Table|choice SQL and JSON example]]

### Part 5
1. [[Part 5 Raw#5. Skill Replacement Rules|Replaceable Skill Choices]]

### Part 6
1. [[Part 6 Raw#4. Fighting Style Choice|Fighting Style Options]] (see also 7)
2. [[Part 6 Raw#1. The `ChoiceSet` Data Structure|Choice sets]]

### Part 7
1. [[Part 7 Raw#5. Generic Choice Storage|DB Choice Storage]]
2. [[Part 7 Raw#2. Limited Known Forms / Options|ChoiceSet improvement]]

# Extracted Data Structures and Representations

### Part 1

#### #1
```SQL
choices
-------------
id TEXT PRIMARY KEY
entity_id TEXT
description TEXT
data JSON
```

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

```SQL
character_selections
-------------
character_id TEXT
entity_id TEXT
type TEXT
```

### Part 5

#### #1
```Rust
FeatureChoice {  
    feature_id: "background_skill",  
    selected_skill: Skill::Stealth  
}
```

### Part 6

#### #1
fighting style
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

weapon mastery
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

#### #2
```Rust
pub struct ChoiceSet {  
    pub id: ChoiceId,  
    pub source: RuleSource,  
    pub choice_type: ChoiceType,  
    pub limit: ChoiceLimit,  
    pub filter: Option<ChoiceFilter>,  
}

pub enum ChoiceType {  
    Feat,  
    FightingStyle,  
    Spell,  
    WeaponMastery,  
    WildshapeForm,  
    Subclass,  
}

pub struct ChoiceLimit {  
    pub count: u8  
}

pub enum ChoiceLimit {  
    Fixed(u8),  
    Scaling(String) // e.g. "@fighter.masteries"  
}

pub struct ChoiceFilter {  
    pub feat_category: Option<FeatCategory>,  
    pub spell_list: Option<SpellListId>,  
    pub creature_type: Option<CreatureType>,  
    pub max_cr: Option<f32>,  
}

// character storage
pub struct ChoiceSelection {  
    pub choice_id: ChoiceId,  
    pub selected: Vec<ChoiceValue>  
}

pub enum ChoiceValue {  
    Feat(FeatId),  
    Spell(SpellId),  
    FightingStyle(FightingStyleId),  
    Creature(CreatureId),  
}
```

example:
```YAML
choices:  
  
  fighting_style:  
    selected:  
      - archery  
  
  wildshape_forms:  
    selected:  
      - wolf  
      - brown_bear  
      - giant_spider
```

### Part 7

#### #1
```SQL
CREATE TABLE character_choices (  
    id INTEGER PRIMARY KEY,  
    character_id INTEGER,  
    choice_id INTEGER,  
    selection_type TEXT  
);

CREATE TABLE choice_values (  
    choice_row_id INTEGER,  
    value_type TEXT,  
    value_id INTEGER  
);
```

#### #2
```Rust
pub struct ChoiceSet {  
    pub id: ChoiceSetId,  
    pub options: Vec<ContentKey>,  
    pub max_selections: u8,  
}
```

