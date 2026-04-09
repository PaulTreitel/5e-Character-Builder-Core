# Links

### Part 2
1. [[Part 2 Raw#2. Why a Dependency Graph Rule Engine Is Needed|Dependency graph introduction]]
2. [[Part 2 Raw#**Please Do [Show Rust Implementation of Dependency Graph Rule Engine]***|Graph example implementation]]
3. [[Part 2 Raw#***Show me Typed rule graphs + stacking rules as mentioned earlier***|Typed rule graphs and bonus stacking rules]]
4. see also: [[Extracted - Rule Graph Fragments|rule graph fragments]]
5. [[Part 2 Raw#Please Do [Show Contextual Rule Graphs]|Contextual rule graphs]]
6. [[Part 2 Raw#2. Engine Module (Generic Rule Graph)|General structuring]]

### Part 3
1. [[Part 3 Raw#Correct Graph Structure|Example AC graph structure]]

### Part 4
1. [[Part 4 Raw#9. Rule Graph|Code architecture: rule graph]]
2. [[Part 4 Raw#2. Core Engine Files||Engine/rule_graph structs]]
3. [[Part 4 Raw#4. Rule Node With Provenance|Graph provenance tracking]] (see also 9)

### Part 5
1. [[Part 5 Raw#4. Dependency Graph|Stat dependency graph and caching]]
2. [[Part 5 Raw#Please cover those three improvements [Rule Compilation, Precomputed Dependency Graphs, Immutable Character Snapshots]|Rule compilation and precomputed graphs]] (long)
3. [[Part 5 Raw#4. How It Fits Into Your Existing Dependency Graph|Integrating rule phases into the rule graph]]

### Part 7
1. [[Part 7 Raw#Please do [show rule module dependency graphs].|Module dependency graphs]] (long)

# Common Data Structure Elements

### struct Node
- identifier
- dependencies: `Vec<NodeId>`
- computing things
	- function pointer to compute it given a context
	- an expected `ValueType` and a stacking rule
		- no clear method of computation: involves rule `Contribution`s

### struct RuleNode
- identifier
- dependencies: `Vec<RuleId>`
- function pointer to apply to a character sheet
- source of the rule (for provenance tracking)
- alternate type: 
	- stat-computation node with a base value (integer)
	- modifiers `Vec<ModifierRule>` (undefined struct)
	- overrides `Vec<OverrideRule>` (undefined struct)

### struct RuleGraph
- map from node identifiers to nodes
- above plus a list of `Contribution`s
- alternate graph to match alternate `RuleNode`:
	- map from stats to nodes

### structs Contribution and ConditionalContribution
- target and source node identifiers
- seems meant to relate dependency and depended-upon relations
- conditional adds a function pointer to determine if the condition is met based on a `RuleContext`

### struct RuleContext
- option: map from node identifiers to `Value`s
	- stores the values that have already been computed from the graph
- option: context is specific optional contexts
	- e.g., weapon context (dex vs str, for example) becomes `Option<&'a Weapon>` or `Option<AttackContext>`

### enums Value and ValueType
- the value type of a graph node (integer or boolean)
	- return type of a node computation function
- `ValueType` used for what a node's *expected* type is
	- held by a node to confirm the expected

### struct CompiledRule
- target stat
- dependency list? `Vec<Stat>`
- function pointer to apply it to a node
- rule source for provenance

### enum StackRule
- sum, max, min, override, boolean or, boolean and

### struct StatDependencyGraph
- map from stats to the list of stats they depend on

### struct CachedRuleGraph
- stores computed mapping from stats to their values

### structs ModuleGraph and ModuleManifest
- building a rule graph-like structure for dependencies between modules

# Extracted Data Structures and Representations

### Part 2

#### #1
```Rust
pub struct RuleNode {  
    pub id: RuleId,  
    pub depends_on: Vec<RuleId>,  
    pub apply: fn(&mut CharacterSheet),  
}
```

#### #2
```Rust
#[derive(Clone, Debug)]  
pub enum Value {  
    Int(i32),  
    Bool(bool),  
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]  
pub struct NodeId(pub &'static str);

pub struct Node {  
    pub id: NodeId,  
    pub deps: Vec<NodeId>,  
    pub compute: fn(&Context) -> Value,  
}

use std::collections::HashMap;  
  
pub struct Context {  
    values: HashMap<NodeId, Value>,  
}

pub struct RuleGraph {  
    nodes: HashMap<NodeId, Node>,  
}
```

#### #3
```Rust
#[derive(Clone, Debug)]  
pub enum Value {  
    Int(i32),  
    Bool(bool),  
}

// nodes track expected data type
#[derive(Clone, Copy)]  
pub enum ValueType {  
    Int,  
    Bool,  
}

pub enum StackRule {  
    Sum,  
    Max,  
    Min,  
    Override,  
    BooleanOr,  
    BooleanAnd,  
}

pub struct Node {  
    pub id: NodeId,  
    pub value_type: ValueType,  
    pub stack: StackRule,  
    pub deps: Vec<NodeId>,  
}

pub struct Contribution {  
    pub target: NodeId,  
    pub source: NodeId,  
}

// example
Contribution {  
    target: NodeId("ac_bonus"),  
    source: NodeId("ring_protection_bonus"),  
}

// for constant rule sources
pub struct ConstantNode {  
    pub id: NodeId,  
    pub value: Value,  
}

// example
ConstantNode {  
    id: NodeId("ring_protection_bonus"),  
    value: Value::Int(1),  
}
```

#### #5
```Rust
pub struct RuleContext<'a> {  
    pub weapon: Option<&'a Weapon>,  
    pub spell: Option<&'a Spell>,  
    pub skill: Option<Skill>,  
    pub target: Option<&'a Creature>,  
}

pub struct Node {  
    pub id: NodeId,  
    pub deps: Vec<NodeId>,  
    pub compute: fn(&Context, &RuleContext) -> Value,  
}

pub struct ConditionalContribution {  
    pub target: NodeId,  
    pub source: NodeId,  
    pub condition: fn(&RuleContext) -> bool,  
}
```

#### #6
```Rust
pub struct RuleGraph {  
    nodes: HashMap<NodeId, Node>,  
    contributions: Vec<Contribution>,  
}

pub struct RuleFragment {  
    pub nodes: Vec<Node>,  
    pub constants: Vec<ConstantNode>,  
    pub contributions: Vec<Contribution>,  
}

#[derive(Clone, Debug)]  
pub enum Value {  
    Int(i32),  
    Bool(bool),  
    Dice(Dice),  
}

pub enum StackRule {  
    Sum,  
    Max,  
    Min,  
    Override,  
    BooleanOr,  
    BooleanAnd,  
}

pub struct Node {  
    pub id: NodeId,  
    pub stack: StackRule,  
    pub deps: Vec<NodeId>,  
}

pub struct RuleFragment {  
    pub nodes: Vec<Node>,  
    pub constants: Vec<ConstantNode>,  
    pub contributions: Vec<Contribution>,  
}

pub struct AttackContext<'a> {  
    pub weapon: &'a Weapon,  
}

// example
pub fn compute_attack(  
    engine: &RuleEngine,  
    ctx: AttackContext,  
) -> AttackResult
```

### Part 4

#### #1
```Rust
pub struct RuleGraph {  
    pub nodes: HashMap<Stat, RuleNode>,  
}

pub struct RuleNode {  
    pub base: i32,  
    pub modifiers: Vec<ModifierRule>,  
    pub overrides: Vec<OverrideRule>,  
}
```

#### #2
```Rust
pub struct RuleGraph {  
    nodes: HashMap<Stat, RuleNode>,  
}

pub struct RuleNode {  
    pub base: i32,  
    pub modifiers: Vec<ModifierRule>,  
    pub overrides: Vec<OverrideRule>,  
}
```

#### #3
```Rust
pub struct RuleNode {  
    pub base: i32,  
    pub base_source: Option<RuleSource>,  
  
    pub modifiers: Vec<ModifierRule>,  
    pub overrides: Vec<OverrideRule>,  
}
```

### Part 5

#### #1
```Rust
pub struct StatDependencyGraph {  
    pub edges: HashMap<Stat, Vec<Stat>>,  
}

pub struct CachedRuleGraph {  
    pub index: RuleIndex,  
    pub values: HashMap<Stat, i32>,  
}
```

#### #2
Rule compilation
```Rust
pub struct CompiledRule {  
    pub target: Stat,  
    pub apply: fn(&mut RuleNode),  
    pub source: RuleSource,  
}
```

Graph precomputation
```Rust
pub struct CompiledRule {  
    pub target: Stat,  
    pub dependencies: Vec<Stat>,  
    pub apply: fn(&mut RuleNode),  
}

pub struct StatDependencyGraph {  
    pub edges: HashMap<Stat, Vec<Stat>>,  
}
```

### Part 7

#### #1
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

```Rust
pub struct ModuleManifest {  
    pub id: ModuleId,  
    pub name: String,  
    pub version: String,  
  
    pub requires: Vec<ModuleId>,  
    pub optional: Vec<ModuleId>,  
}

pub struct ModuleGraph {  
    pub modules: HashMap<ModuleId, ModuleManifest>,  
}
```

# (Pseudo)Code

### Part 2

#### #1
```Rust
fn run_rules(nodes: Vec<RuleNode>, sheet: &mut CharacterSheet) {  
  
    let order = topo_sort(nodes);  
  
    for node in order {  
        (node.run)(sheet);  
    }  
}

// example
use petgraph::graph::DiGraph;  
let mut graph = DiGraph::<RuleNode, ()>::new();  
let ability = graph.add_node(ability_node);  
let modifier = graph.add_node(modifier_node);  
graph.add_edge(ability, modifier, ());
petgraph::algo::toposort(&graph, None)
```

#### #2
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

impl RuleGraph {  
    pub fn add_node(&mut self, node: Node) {  
        self.nodes.insert(node.id, node);  
    }  
}
```

MANUAL TOPOLOGICAL SORT
```Rust
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
```


```Rust
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
```

Example: ability modifiers
```Rust
const STR_SCORE: NodeId = NodeId("strength_score");  
const STR_MOD: NodeId = NodeId("strength_mod");

fn strength_mod(ctx: &Context) -> Value {  
    let score = ctx.get_int(STR_SCORE);  
    Value::Int((score - 10) / 2)  
}

graph.add_node(Node {  
    id: STR_MOD,  
    deps: vec![STR_SCORE],  
    compute: strength_mod,  
});
```

Example: AC
```Rust
fn armor_class(ctx: &Context) -> Value {  
    let base = ctx.get_int(NodeId("armor_base"));  
    let dex = ctx.get_int(NodeId("dex_mod"));  
    let shield = ctx.get_int(NodeId("shield_bonus"));  
  
    Value::Int(base + dex + shield)  
}

graph.add_node(Node {  
    id: NodeId("armor_class"),  
    deps: vec![  
        NodeId("armor_base"),  
        NodeId("dex_mod"),  
        NodeId("shield_bonus"),  
    ],  
    compute: armor_class,  
});
```

Example: Shield
```Rust
graph.add_node(Node {  
    id: NodeId("shield_bonus"),  
    deps: vec![],  
    compute: |_| Value::Int(2),  
});
```

Example: attack bonus aggregator
```Rust
fn sum_attack_bonus(ctx: &Context) -> Value {  
    let prof = ctx.get_int(NodeId("proficiency_bonus"));  
    let ability = ctx.get_int(NodeId("str_mod"));  
    let item = ctx.get_int(NodeId("item_bonus"));  
  
    Value::Int(prof + ability + item)  
}
```

Example: item contributions
```Rust
for item in inventory.equipped_items() {  
    for rule in &item.rules {  
        graph.add_node(rule.to_node());  
    }  
}
```

#### #3

evaluator
```Rust
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
```

example: attack bonus
```Rust
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
```

Example: belt of giant strength
```Rust
Node {  
    id: NodeId("strength_score"),  
    value_type: ValueType::Int,  
    stack: StackRule::Max,  
    deps: vec![  
        NodeId("strength_base"),  
        NodeId("strength_override")  
    ],  
}
```

Example: AC
```Rust
Node {  
    id: NodeId("ac_formula"),  
    stack: StackRule::Max,  
    deps: vec![  
        NodeId("armor_formula"),  
        NodeId("mage_armor_formula"),  
        NodeId("unarmored_formula")  
    ],  
}
```

advantage/disadvantage
```Rust
fn resolve_roll(ctx: &Context) -> RollMode {  
    let adv = ctx.get_bool("attack_advantage");  
    let dis = ctx.get_bool("attack_disadvantage");  
  
    match (adv, dis) {  
        (true, false) => RollMode::Advantage,  
        (false, true) => RollMode::Disadvantage,  
        _ => RollMode::Normal,  
    }  
}
```

Example: Ring of Protection
```Rust
ConstantNode {  
    id: NodeId("ring_protection_ac"),  
    value: Value::Int(1),  
}

Contribution {  
    target: NodeId("ac_bonus"),  
    source: NodeId("ring_protection_ac"),  
}
```

#### #5
contextual weapon attack bonus calculation
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

Example: Archery fighting style
```Rust
ConditionalContribution {  
    target: NodeId("attack_bonus_total"),  
    source: NodeId("archery_bonus"),  
    condition: |ctx| ctx.weapon.map(|w| w.is_ranged()).unwrap_or(false),  
}
```

#### #6
Example: Ring of Protection
```Rust
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
```

### Part 4

#### #1
```Rust
impl RuleNode {  
    pub fn value(&self) -> i32 {  
        if let Some(override_rule) = self.overrides.last() {  
            return override_rule.value;  
        }  
  
        let mut total = self.base;  
  
        total += stacking::apply_modifiers(&self.modifiers);  
  
        total  
    }  
}

pub fn collect_rule_effects(  
    character: &Character,  
    db: &RulesDatabase,  
) -> Vec<RuleEffect> {  
    let mut effects = Vec::new();  
  
    effects.extend(race_rules(character, db));  
    effects.extend(class_rules(character, db));  
    effects.extend(feat_rules(character, db));  
    effects.extend(item_rules(character, db));  
    effects.extend(condition_rules(character, db));  
    effects.extend(spell_rules(character, db));  
  
    effects  
}

pub fn build_graph(effects: Vec<RuleEffect>) -> RuleGraph {  
    let mut graph = RuleGraph::new();  
  
    for effect in effects {  
        graph.apply(effect);  
    }  
  
    graph  
}
```

#### #3
```Rust
pub fn evaluate(node: &RuleNode) -> StatBreakdown {  
  
    if let Some(override_rule) = node.overrides.last() {  
        return StatBreakdown {  
            final_value: override_rule.value,  
            components: vec![  
                StatComponent {  
                    value: override_rule.value,  
                    source: override_rule.source.clone(),  
                }  
            ],  
        };  
    }  
  
    let mut total = node.base;  
  
    let mut components = vec![];  
  
    for modifier in stacking::resolve(&node.modifiers) {  
        total += modifier.value;  
  
        components.push(StatComponent {  
            value: modifier.value,  
            source: modifier.source.clone(),  
        });  
    }  
  
    StatBreakdown {  
        final_value: total,  
        components,  
    }  
}
```

### Part 5

#### #1
See conversation section for additional context
```Rust
fn recompute_dirty(  
    graph: &mut CachedRuleGraph,  
    dirty: &DirtyStats,  
) {  
    for stat in &dirty.stats {  
        graph.values.insert(  
            *stat,  
            compute_stat(graph, *stat)  
        );  
    }  
}

fn propagate_dirty(  
    deps: &StatDependencyGraph,  
    dirty: &mut DirtyStats,  
) {  
    let mut queue: Vec<Stat> = dirty.stats.iter().cloned().collect();  
  
    while let Some(stat) = queue.pop() {  
        if let Some(children) = deps.edges.get(&stat) {  
            for child in children {  
                if dirty.stats.insert(*child) {  
                    queue.push(*child);  
                }  
            }  
        }  
    }  
}
```

#### #2

Rule compilation
```Rust
pub fn compile_rule(effect: RuleEffect) -> CompiledRule {  
    match effect {  
        RuleEffect::Modifier(rule) => {  
            CompiledRule {  
                target: rule.target,  
                apply: move |node| {  
                    node.modifiers.push(rule.clone());  
                },  
                source: rule.source,  
            }  
        }  
  
        RuleEffect::Override(rule) => {  
            CompiledRule {  
                target: rule.target,  
                apply: move |node| {  
                    node.overrides.push(rule.clone());  
                },  
                source: rule.source,  
            }  
        }  
    }  
}
```

Graph precomputation
```Rust
for rule in compiled_rules {  
    for dep in rule.dependencies {  
        graph.edges.entry(dep)  
            .or_default()  
            .push(rule.target);  
    }  
}
```

### Part 7

#### #1
```Rust
fn resolve_modules(modules: Vec<ModuleManifest>) -> Vec<ModuleId> {  
  
    let graph = build_graph(modules);  
  
    topological_sort(graph)  
}
```