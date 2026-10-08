# Links

### Part 2
1. [[Part 2 Raw#14. Rule Phases (Very Rust-Friendly)|Rule phase first data structures]]

### Part 5
1. [[Part 5 Raw#Previously you mentioned implementing rule phases. How does that work and how does it fit in to our existing architecture?|Rule phase introduction]] (long)

### Part 6
1. [[Part 6 Raw#4. Rule Phase|Rule phases in the Rule struct context]]
2. [[Part 6 Raw#18. Relationship to the Rule Index|In the rule index]]

# Common Data Structure Elements

### enum RulePhase
- variant 1: base, override, modifier, derived, finalize
	- graph for computing base values (base AC, base ability score?, etc)
	- graph for overrides (e.g. barkskin)
	- graph for modifiers (e.g. magic items, spell effects)
	- ???
- variant 2: base stats, stat modifiers, proficiencies, derived stats, other things (conditions, magic items, feature modifiers), actions
	- splits into layers for computing base stats, the modifiers, add proficiency bonuses, compute stats derived from that, apply other effects, actions
	- action rule phase is for things granting actions (like CD or enter rage)

### struct RuleIndex
- fields are maps, one for each rule phase
	- map stats `->` rules determining that stat

### Changes to Other Structs
- features now have a list of `(RulePhase, Effect)` pairs
- `Rule` struct has a field for the phase

# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
pub enum RulePhase {  
    Abilities,  
    Proficiencies,  
    DerivedStats,  
    Spellcasting,  
}

pub struct Feature {  
    pub effects: Vec<(RulePhase, Effect)>  
}
```

### Part 5

#### #1
```Rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]  
pub enum RulePhase {  
    Base,  
    Override,  
    Modifier,  
    Derived,  
    Finalize,  
}

pub struct Rule {  
    pub id: String,  
    pub phase: RulePhase,  
    pub inputs: Vec<StatId>,  
    pub output: StatId,  
    pub op: RuleOp,  
}

// ring of protection example
Rule {  
    id: "ring_protection_ac".into(),  
    phase: RulePhase::Modifier,  
    inputs: vec![StatId::ArmorClass],  
    output: StatId::ArmorClass,  
    op: RuleOp::Add(1),  
}

// barkskin example
Rule {  
    id: "barkskin_ac",  
    phase: RulePhase::Override,  
    inputs: vec![StatId::ArmorClass],  
    output: StatId::ArmorClass,  
    op: RuleOp::Max(16),  
}

pub struct RuleEngine {  
    graphs: HashMap<RulePhase, RuleGraph>,  
}
```

### Part 6

#### #1
```Rust
pub enum RulePhase {  
    BaseStats,  
    AbilityModifiers,  
    Proficiency,  
    FeatureModifiers,  
    Conditions,  
    DerivedStats,  
    Actions,  
}
```

#### #2
```Rust
pub struct RuleIndex {  
    pub phase_rules: HashMap<RulePhase, Vec<RuleId>>,  
    pub stat_rules: HashMap<StatId, Vec<RuleId>>,  
    pub action_rules: HashMap<ActionType, Vec<RuleId>>,  
    pub condition_rules: HashMap<ConditionId, Vec<RuleId>>,  
}
```

# (Pseudo)Code

### Part 5

### #1
```Rust
impl RuleEngine {  
    pub fn evaluate(&self, ctx: &mut CharacterContext) {  
        let phases = [  
            RulePhase::Base,  
            RulePhase::Override,  
            RulePhase::Modifier,  
            RulePhase::Derived,  
            RulePhase::Finalize,  
        ];  
  
        for phase in phases {  
            if let Some(graph) = self.graphs.get(&phase) {  
                graph.evaluate(ctx);  
            }  
        }  
    }  
}
```
