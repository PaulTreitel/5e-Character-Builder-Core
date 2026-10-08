# Links

### Part 2
1. [[Part 2 Raw#4. Evaluation Context|Evaluation context]]
	1. [[Part 2 Raw#7. Graph Evaluation|Examples for the eval context]] (down through 11)
2. [[Part 2 Raw#2. Introducing Rule Context|Rule Context]] (through 4)
3. [[Part 2 Raw#5. Contextual Contributions|Contextual contributions]]
	1. [[Part 2 Raw#10. Actions Module (Contextual Graphs)|Example AttackContext]]

### Part 6
1. [[Part 6 Raw#5. Rule Filters|Rule filters]]

# Common Data Structure Elements

### struct/enum RuleContext
- different fields for different possible contexts (weapon/attack, spell, skill)
	- fields are either an object of the corresponding type (eg `Spell`) or a specific type (eg `SpellContext` )
- the struct version is all `Option` types so it makes more sense to use the enum and if needed have a `Vec<RuleContext>`

# Extracted Data Structures and Representations

### Part 1

#### #1
```Rust
// NodeIds like NodeId("proficiency_bonus") or NodeId("shield bonus")
pub struct Context {  
    values: HashMap<NodeId, Value>,  
}  
```

#### #2
```Rust
pub struct RuleContext<'a> {  
    pub weapon: Option<&'a Weapon>,  
    pub spell: Option<&'a Spell>,  
    pub skill: Option<Skill>,  
    pub target: Option<&'a Creature>,  
}
```

#### #3
```Rust
pub struct ConditionalContribution {  
    pub target: NodeId,  
    pub source: NodeId,  
    pub condition: fn(&RuleContext) -> bool,  
}
```

#### #4
```Rust
// similar to struct version above
pub enum RuleContext<'a> {  
    Attack(AttackContext<'a>),  
    Damage(DamageContext<'a>),  
    Spell(SpellContext<'a>),  
    Skill(SkillContext),  
}

pub struct AttackContext<'a> {  
    pub weapon: &'a Weapon,  
}
```

### Part 6

#### #1
```Rust
pub struct RuleFilter {  
    pub class_level: Option<ClassLevelFilter>,  
    pub equipped_item: Option<ItemId>,  
    pub condition_active: Option<ConditionId>,  
    pub action_type: Option<ActionType>,  
}
```

# (Pseudo)Code

### Part 1

#### #1
```Rust
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

// create context
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

// Example: Ability mod
const STR_SCORE: NodeId = NodeId("strength_score");  
const STR_MOD: NodeId = NodeId("strength_mod");

fn strength_mod(ctx: &Context) -> Value {  
    let score = ctx.get_int(STR_SCORE);  
    Value::Int((score - 10) / 2)  
}

// Example: compute attack bonus
fn sum_attack_bonus(ctx: &Context) -> Value {  
    let prof = ctx.get_int(NodeId("proficiency_bonus"));  
    let ability = ctx.get_int(NodeId("str_mod"));  
    let item = ctx.get_int(NodeId("item_bonus"));  
  
    Value::Int(prof + ability + item)  
}
```

#### #2
```Rust
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
```

#### #3
```Rust
ConditionalContribution {  
    target: NodeId("attack_bonus_total"),  
    source: NodeId("archery_bonus"),  
    condition: |ctx| ctx.weapon.map(|w| w.is_ranged()).unwrap_or(false),  
}
```

#### #4
```Rust
pub fn compute_attack(  
    engine: &RuleEngine,  
    ctx: AttackContext,  
) -> AttackResult
```
