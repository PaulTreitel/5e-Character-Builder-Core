# Links

### Part 1
1. [[Part 1 Raw#6. Requirement System|Requirement system]]

### Part 2
1. [[Part 2 Raw#11. Fragments Enable Conditional Rules|Conditional rule fragments]]

### Part 3
1. [[Part 3 Raw#8. Item Effects as Rule Sources|Item effect activation condition]]

### Part 4
1. [[Part 4 Raw#3. Activation Conditions|conditional activations]]

### Part 5
1. [[Part 5 Raw#11. Interaction With Inventory|wildshape item suppression]]
2. [[Part 5 Raw#2. Shared Rule Structures (YAML)|General rule YAML structure]]

### Part 6
1. [[Part 6 Raw#Conditional|YAML expression for condition]]
2. [[Part 6 Raw#4. Rust Representation|RuleExpr enum, including conditionals]] (and a condition enum)
3. [[Part 6 Raw#14. Full Rule Example Using Expressions|Rage YAML and expression example]]
4. [[Part 6 Raw#1. Rage (Barbarian)|Alternate rage]]

# Common Data Structure Elements

# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
enum Requirement {
    Class(String),
    Level(u8),
    StatAbove { stat: Stat, value: u8 },
}

fn check_requirement(character: &Character, req: &Requirement) -> bool
```

### Part 2

#### #1
```Rust
ConditionalFragment {  
    condition: Condition::Raging,  
    fragment: RuleFragment { ... }  
}

pub struct ConditionalContribution {  
    pub target: NodeId,  
    pub source: NodeId,  
    pub condition: fn(&RuleContext) -> bool,  
}

// archery fighting style
ConditionalContribution {  
    target: NodeId("attack_bonus_total"),  
    source: NodeId("archery_bonus"),  
    condition: |ctx| ctx.weapon.map(|w| w.is_ranged()).unwrap_or(false),  
}
```

### Part 3

#### #1
```Rust
// condition for 
ActivationCondition::Equipped
```

### Part 4

#### #1
example feat (GWM)
```YAML
effect:  
  operation: add  
  target: combat.damage_bonus  
  value: 10  
  
condition:  
  weapon_tag: heavy
```

example magic item
```YAML
condition:  
  attuned: true
```

example spell
```YAML
condition:  
  active_spell: barkskin
```

### Part 5

#### #1
```Rust
RuleCondition::NotWildShapeRestricted
```

#### #2
```YAML
Rule:  
  id: string  
  phase: base | override | modifier | derived | finalize  
  target: stat_id  
  operation: operation_type  
  value: expression  
  category: optional_bonus_category  
  condition: optional_condition
```

### Part 6

#### #1
```YAML
expr: if  
args:  
  - condition:  
      has_condition: raging  
  - 2  
  - 0
```

#### #2
```Rust
pub enum RuleExpr {  
  
    Constant(i32),  
  
    Stat(StatId),  
  
    Add(Vec<RuleExpr>),  
    Sub(Box<RuleExpr>, Box<RuleExpr>),  
    Mul(Box<RuleExpr>, Box<RuleExpr>),  
    Div(Box<RuleExpr>, Box<RuleExpr>),  
  
    Min(Vec<RuleExpr>),  
    Max(Vec<RuleExpr>),  
  
    If {  
        condition: Condition,  
        then_expr: Box<RuleExpr>,  
        else_expr: Box<RuleExpr>,  
    },  
  
    Dice {  
        count: Box<RuleExpr>,  
        sides: Box<RuleExpr>,  
    },  
}

pub enum Condition {  
  
    HasCondition(String),  
  
    EquippedItem(String),  
  
    HasFeature(String),  
  
    StatGreaterThan(StatId, i32),  
}
```

#### #3
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

#### #4
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

