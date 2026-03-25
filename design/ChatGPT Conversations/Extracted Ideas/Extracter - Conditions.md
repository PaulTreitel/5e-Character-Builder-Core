May need to consider conditions being broader than actual conditions. A second category of conditions representing character states like "raging" or "wildshaped." Or some other system of "applied states" that can be used by the [[Extracted - Rule Conditions|rule condition]] system.

# Links

### Part 4
1. [[Part 4 Raw#Conditions|Poisoned YAML]]
2. [[Part 4 Raw#6. Conditions|Conditions as rule effects???]]
3. [[Part 4 Raw#4. ConditionSystem Trait|Condition System trait]]

### Part 5
1. [[Part 5 Raw#15. Conditions|Prone YAML]]

### Part 7
1. [[Part 7 Raw#10. Active Conditions|condition SQL table]]

# Extracted Data Structures and Representations

### Part 4

#### #1
```YAML
effects:  
  - operation: disadvantage  
    target: combat.attack_roll
```

#### #2
```Rust
pub struct ConditionInstance {  
    pub condition: ConditionId,  
    pub source: ConditionSource,  
}
```

#### #3
```Rust
pub trait ConditionSystem {  
    fn apply_condition(  
        &self,  
        character: &mut Character,  
        condition: ConditionId,  
        source: RuleSource,  
    );  
  
    fn remove_condition(  
        &self,  
        character: &mut Character,  
        condition: ConditionId,  
    );  
  
    fn collect_condition_effects(  
        &self,  
        character: &Character,  
        db: &RulesDatabase,  
    ) -> Vec<RuleEffect>;  
}
```

### Part 5

#### #1
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

### Part 7
```SQL
CREATE TABLE character_conditions (  
    character_id INTEGER,  
    condition_id INTEGER,  
    source_rule INTEGER,  
    remaining_duration INTEGER  
);
```

