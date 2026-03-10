# Please do \[Show rule indexing and caching]. If this is related to caching the character sheet as mentioned earlier, incorporate that as well.

Yes—**rule indexing, caching, and character-sheet caching** are closely related. The goal is to avoid recomputing the entire rule graph every time something small changes (like equipping an item). Mature RPG engines usually combine three techniques:

1. **Rule indexing**
2. **Incremental rule graph recomputation**
3. **Character sheet caching**

Your architecture already supports these well; we just add a few structures.

---

### 1. The Problem

Without caching, stat computation looks like:
```
Character  
   ↓  
collect_rule_effects()  
   ↓  
build_rule_graph()  
   ↓  
evaluate all stats
```
If a user toggles a shield on/off in a UI, this might run **dozens of times per second**.

For complex characters you may have:
```
100–300 rule effects  
30–80 derived stats
```
Rebuilding everything each time is wasteful.

---

### 2. Core Idea

Split evaluation into **two phases**:
```
Rule collection (rare)  
Stat evaluation (frequent)
```
And cache both.

---

### 3. Rule Index

Instead of storing rules in a single list:
```Rust
Vec<RuleEffect>
```
we index them by **target stat**.
```Rust
use std::collections::HashMap;  
  
pub struct RuleIndex {  
    pub by_stat: HashMap<Stat, Vec<RuleEffect>>,  
}
```
Builder:
```Rust
impl RuleIndex {  
    pub fn build(effects: Vec<RuleEffect>) -> Self {  
        let mut by_stat = HashMap::new();  
  
        for effect in effects {  
            let stat = effect.target_stat();  
  
            by_stat.entry(stat)  
                .or_insert_with(Vec::new)  
                .push(effect);  
        }  
  
        Self { by_stat }  
    }  
}
```
Now when computing **ArmorClass**, we only look at rules affecting AC.

---

### 4. Dependency Graph

Some stats depend on others:
```
DEX → Initiative  
STR → CarryingCapacity  
ProficiencyBonus → AttackBonus
```
Represent this as:
```Rust
pub struct StatDependencyGraph {  
    pub edges: HashMap<Stat, Vec<Stat>>,  
}
```
Example:
```
Dexterity → Initiative  
Strength → CarryingCapacity
```
This allows **partial recomputation**.

---

### 5. Cached Rule Graph

Instead of rebuilding graphs repeatedly, store them.
```Rust
pub struct CachedRuleGraph {  
    pub index: RuleIndex,  
    pub values: HashMap<Stat, i32>,  
}
```
Compute once:
```Rust
CachedRuleGraph::build(character)
```
Then reuse.

---

### 6. Character Sheet Cache

Now add a cache for final derived values.
```Rust
pub struct CharacterSheetCache {  
    pub stats: HashMap<Stat, StatBreakdown>,  
}
```
So a character becomes:
```Rust
pub struct Character {  
    pub state: CharacterState,  
  
    pub rule_graph: CachedRuleGraph,  
    pub sheet_cache: CharacterSheetCache,  
}
```
---

### 7. Dirty Flags

When something changes, we **invalidate only affected stats**.

Add:
```Rust
use std::collections::HashSet;  
  
pub struct DirtyStats {  
    pub stats: HashSet<Stat>,  
}
```
Example change:
```
equip shield
```
Marks:
```
ArmorClass
```
dirty.

---

### 8. Incremental Recompute

Algorithm:
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
```
Then update sheet cache.

---

### 9. Dependency Propagation

If a base stat changes, recompute dependents.

Example:
```
Strength changed
```
Needs:
```
CarryingCapacity  
Encumbrance
```
Use dependency graph:
```Rust
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
---

### 10. Inventory Optimization

Inventory changes are common.

Example:
```
add potion
```
Only affects:
```
InventoryWeight  
Encumbrance  
Speed
```
Dirty flags:
```
InventoryWeight
```
Dependency graph expands:
```
Encumbrance  
Speed
```
Everything else remains cached.

---

### 11. Character Sheet Snapshot

UI often needs a **complete character sheet**.

Store:
```Rust
pub struct CharacterSheet {  
    pub stats: HashMap<Stat, i32>,  
    pub breakdowns: HashMap<Stat, StatBreakdown>,  
}
```
Update only dirty stats:
```Rust
fn update_sheet(  
    cache: &mut CharacterSheetCache,  
    graph: &CachedRuleGraph,  
    dirty: &DirtyStats,  
) {  
    for stat in &dirty.stats {  
        cache.stats.insert(  
            *stat,  
            graph.values[stat]  
        );  
    }  
}
```
---

### 12. Event-Driven Updates

When something changes:
```
equip item  
apply condition  
level up  
cast spell
```
We emit an event:
```
CharacterEvent
```
Example:
```Rust
pub enum CharacterEvent {  
    ItemEquipped(ItemInstanceId),  
    ItemRemoved(ItemInstanceId),  
    ConditionAdded(ConditionId),  
    AbilityScoreChanged(Stat),  
}
```
Events mark dirty stats.

---

### 13. Putting It All Together

Full flow:
```
Character change  
    ↓  
event emitted  
    ↓  
dirty stats marked  
    ↓  
dependency propagation  
    ↓  
incremental rule recompute  
    ↓  
character sheet cache updated
```
---

### 14. Performance Impact

Without caching:
```
100+ rules  
50+ stats
```
Full recompute per change.

With indexing + caching:
```
1–5 stat recalculations
```
Typical speedup:
```
10x–100x
```
This becomes important in:
```
web apps  
live character builders  
combat simulators
```
---

### 15. Memory Cost

Very small.

Typical caches:
```
Stat values: ~50 integers  
Breakdowns: ~50 small structs  
Rule index: ~200 rules
```
This is trivial for modern systems.

---

### 16. Why This Fits Your Architecture

Because your system already uses:
```
RuleEffect  
RuleGraph  
Stat enum
```
We only added:
```
RuleIndex  
StatDependencyGraph  
CharacterSheetCache  
DirtyStats
```
No major redesign required.

---

### 17. What Mature RPG Engines Add Next

If you continued evolving this system, the next improvements are typically:

1. **rule compilation**
2. **precomputed dependency graphs**
3. **immutable character snapshots**

These are what allow systems like **Foundry VTT or large character builders** to evaluate very complex characters instantly.

---

✅ **Final Result**

Your architecture now supports:
```
data-driven rules  
multiple editions  
inventory trees  
spells and conditions  
rule provenance  
incremental stat recomputation  
character sheet caching
```
while still staying **relatively small and Rust-idiomatic**.

---

If you'd like, I can also show **one final structural diagram of the entire engine** (modules + data flow + caches). At this point the system is complex enough that a diagram becomes extremely helpful for implementation.

# Please cover those three improvements \[Rule Compilation, Precomputed Dependency Graphs, Immutable Character Snapshots]

The three improvements I mentioned — **rule compilation**, **precomputed dependency graphs**, and **immutable character snapshots** — are the next level of optimization and architectural clarity. They are commonly used in larger RPG engines and character builders because they make the system both **faster and easier to reason about**.

Importantly, they **fit naturally into the architecture you've already designed**, so they don’t require a redesign.

---

### 1. Rule Compilation

#### Problem

Right now rules are interpreted repeatedly.

Example rule:
```Rust
RuleEffect::Modifier {  
    target: Stat::ArmorClass,  
    value: 2,  
}
```
During evaluation, the engine repeatedly:

1. Matches enum variants
2. Applies stacking rules
3. Updates node state

This overhead is small but accumulates with **hundreds of rules**.

---

#### Solution

Convert rules into **compiled evaluators** once.

Instead of evaluating raw rules every time, convert them into lightweight functions.

##### Compiled Rule
```Rust
pub struct CompiledRule {  
    pub target: Stat,  
    pub apply: fn(&mut RuleNode),  
    pub source: RuleSource,  
}
```
Compilation step:
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
Now rule application is simply:
```Rust
for rule in compiled_rules {  
    (rule.apply)(&mut node);  
}
```
No pattern matching required during evaluation.

---

#### When Compilation Happens

Compile rules when:
```
character state changes
```
Examples:
```
level up  
equip item  
gain feat  
cast spell
```
This happens far less often than stat queries.

---

### 2. Precomputed Dependency Graph

Earlier we introduced dependencies like:
```
DEX → Initiative  
STR → CarryingCapacity  
InventoryWeight → Encumbrance → Speed
```
Right now those dependencies might be manually defined.

A better approach is **precomputing them from rules**.

---

#### Dependency Detection

Each rule indicates:
```
which stat it reads  
which stat it writes
```
Add metadata:
```Rust
pub struct CompiledRule {  
    pub target: Stat,  
    pub dependencies: Vec<Stat>,  
    pub apply: fn(&mut RuleNode),  
}
```
Example:
```
Initiative = DexModifier
```
Produces:
```Rust
dependencies = [Stat::Dexterity]  
target = Stat::Initiative
```
---

#### Building the Graph
```Rust
pub struct StatDependencyGraph {  
    pub edges: HashMap<Stat, Vec<Stat>>,  
}
```
Builder:
```Rust
for rule in compiled_rules {  
    for dep in rule.dependencies {  
        graph.edges.entry(dep)  
            .or_default()  
            .push(rule.target);  
    }  
}
```
Now if **Dexterity changes**, we know exactly which stats must update.

---

#### Why This Matters

Without dependency tracking:
```
recompute everything
```
With it:
```
recompute only affected stats
```
Typical updates become **1–5 stat recalculations**.

---

### 3. Immutable Character Snapshots

Mutable characters cause problems:
```
UI reads stats  
combat modifies stats  
spell effects apply
```
Race conditions or inconsistent states can occur.

Instead, use **immutable snapshots**.

---

#### Snapshot Concept

Every change produces a **new snapshot** of the character state.
```Rust
pub struct CharacterSnapshot {  
    pub state: CharacterState,  
    pub compiled_rules: Vec<CompiledRule>,  
    pub stat_cache: HashMap<Stat, i32>,  
}
```
Snapshots are:
```
read-only  
thread-safe  
cacheable
```
---

#### Snapshot Creation

Whenever the character changes:
```
equip item  
apply condition  
gain level
```
Create a new snapshot.
```Rust
pub fn rebuild_snapshot(character: &CharacterState) -> CharacterSnapshot
```
Internally this:

1. collects rules
2. compiles them
3. builds dependency graph
4. initializes stat cache

---

#### Why Snapshots Are Powerful

##### 1. Thread Safety

Multiple systems can read the same snapshot:
```
UI  
combat simulator  
export tools
```
No locks required.

---

##### 2. Undo/Redo

Snapshots enable:
```
undo last change  
compare builds  
time travel debugging
```
Just keep previous snapshots.

---

##### 3. Combat Simulations

Example:
```
clone snapshot  
apply hypothetical spell  
evaluate result  
discard
```
No state mutation needed.

---

### 4. Combined Flow

With all three improvements the system works like this:
```
Character State  
      │  
      ▼  
Rule Collection  
      │  
      ▼  
Rule Compilation  
      │  
      ▼  
Dependency Graph Build  
      │  
      ▼  
CharacterSnapshot  
      │  
      ▼  
Stat Cache
```
Queries only hit the **snapshot and cache**.

---

### 5. Update Flow

When something changes:
```
item equipped  
condition added  
level up
```
Process:
```
update CharacterState  
      ↓  
collect rules  
      ↓  
compile rules  
      ↓  
rebuild dependency graph  
      ↓  
create new CharacterSnapshot
```
---

### 6. Performance Impact

Typical character:
```
150 rules  
60 stats
```
Without improvements:
```
evaluate entire graph every query
```
With improvements:
```
compiled rule application  
incremental stat recompute  
snapshot cache reuse
```
Performance becomes effectively **constant-time for most stat queries**.

---

### 7. Memory Impact

Very small.

Typical snapshot:
```
stat cache ≈ 60 integers  
compiled rules ≈ 150 entries  
dependency graph ≈ small adjacency list
```
Total usually **under 100 KB**.

---

### 8. How This Fits Your Current Modules

Your project structure evolves slightly:
```
engine/  
   rule_graph.rs  
   rule_compiler.rs  
   dependency_graph.rs  
   snapshot.rs  
   caching.rs
```
New core structs:
```
CompiledRule  
StatDependencyGraph  
CharacterSnapshot  
StatCache
```
Everything else remains the same.

---

### 9. Why These Improvements Are Used in Real Engines

They enable:
```
instant character sheet updates  
fast web APIs  
combat simulations  
build comparisons
```
Systems like large character builders often compute **thousands of rule evaluations per minute**, so these optimizations become essential.

---

### 10. Resulting Final Architecture

Your engine now has four layers:
```
Rules Data (YAML)  
        │  
        ▼  
Character State  
        │  
        ▼  
Compiled Rules + Dependency Graph  
        │  
        ▼  
Character Snapshot  
        │  
        ▼  
Stat Cache / Character Sheet
```
The rule graph remains simple, but the system becomes **extremely fast and scalable**.

---

If you'd like, I can also show the **complete final module architecture (files + structs + responsibilities)** for the engine after incorporating _all_ the improvements we've discussed. This usually ends up around **20–25 small modules**, which is a very maintainable size for a project like this.

# Previously you mentioned implementing rule phases. How does that work and how does it fit in to our existing architecture?

Rule **phases** are a way to structure when different rule computations happen so that your rule engine produces **deterministic results**, avoids circular dependencies, and stays **fast enough to recompute often** (which is important for a character sheet UI).

Your current architecture already has:

- **Trait-free rule definitions**
- **Dependency graph evaluation**
- **Cached derived character sheet**
- **Inventory + item effects**
- **Stacking rules**
- **Edition support (2014 / 2024)**

Rule phases fit naturally into this design as a **lightweight ordering layer on top of the dependency graph**.

---

### 1. The Problem Rule Phases Solve

Some rules must happen **before others** or the results become inconsistent.

Example (AC calculation):

1. Base AC from armor
2. AC replacement effects (`Barkskin`, `Mage Armor`)
3. Additive bonuses (`Shield`, ring of protection)
4. Conditional bonuses (`+2 AC vs ranged attacks`)
5. Clamping (`minimum AC`, `maximum AC`)

If these run in arbitrary order you can get incorrect results.

Example:
```
Barkskin: AC cannot be less than 16  
Shield: +5 AC
```
Correct:
```
base AC = 13  
barkskin -> max(13,16) = 16  
shield -> 21
```
Wrong ordering:
```
base AC = 13  
shield -> 18  
barkskin -> max(18,16) = 18
```
Different result.

---

### 2. Rule Phases Concept

Instead of one giant rule graph, rules are grouped into **phases executed sequentially**.
```
Phase 1: Base values  
Phase 2: Replacements / overrides  
Phase 3: Additive modifiers  
Phase 4: Multipliers / scaling  
Phase 5: Final clamps
```
Each phase has its own **dependency graph**.
```
Phase 1  →  Phase 2  →  Phase 3  →  Phase 4  →  Phase 5
```
Values from earlier phases become inputs for later ones.

---

### 3. Example Phases for a D&D Engine

A practical set:
```
Phase 0: Entity State  
Phase 1: Base Values  
Phase 2: Overrides / Replacements  
Phase 3: Additive Modifiers  
Phase 4: Derived Stats  
Phase 5: Finalization
```
##### Phase 0 — Entity State

Raw character state.
```
strength score  
equipped armor  
inventory  
conditions  
level
```
---

##### Phase 1 — Base Values

Rules that establish base values.

Examples:
```
Base AC from armor  
Base HP from class  
Base speed from species  
Spell slots by level
```
Example rule:
```
AC = armor.base_ac + dex_mod
```
---

##### Phase 2 — Overrides

Rules that **replace a value or set minimums**.

Examples:
```
Barkskin  
Mage Armor  
Unarmored Defense
```
Example:
```
AC = max(AC, 16)
```
---

##### Phase 3 — Additive Modifiers

Most rules live here.

Examples:
```
Ring of Protection +1 AC  
Shield spell +5 AC  
Bless +1d4 attack rolls  
Fighting style bonuses
```
---

##### Phase 4 — Derived Stats

Values computed from others.

Examples:
```
attack bonus  
spell save DC  
passive perception  
initiative
```
---

##### Phase 5 — Finalization

Clamp or clean up values.

Examples:
```
minimum speed 0  
minimum HP 0  
proficiency bonus clamp
```
---

### 4. How It Fits Into Your Existing Dependency Graph

Instead of one graph:
```
RuleGraph
```
You now have:
```Rust
RuleGraph[PHASE_COUNT]
```
Example:
```Rust
RuleGraph<Base>  
RuleGraph<Override>  
RuleGraph<Modifier>  
RuleGraph<Derived>  
RuleGraph<Finalize>
```
Evaluation becomes:
```Rust
for phase in phases:  
    evaluate_graph(phase)
```
---

### 5. Phase Enum
```Rust
#[derive(Clone, Copy, PartialEq, Eq, Hash)]  
pub enum RulePhase {  
    Base,  
    Override,  
    Modifier,  
    Derived,  
    Finalize,  
}
```
---

### 6. Rule Definition With Phase

Rules simply declare which phase they belong to.
```Rust
pub struct Rule {  
    pub id: String,  
    pub phase: RulePhase,  
    pub inputs: Vec<StatId>,  
    pub output: StatId,  
    pub op: RuleOp,  
}
```
Example rule:
```
Ring of Protection
```

```Rust
Rule {  
    id: "ring_protection_ac".into(),  
    phase: RulePhase::Modifier,  
    inputs: vec![StatId::ArmorClass],  
    output: StatId::ArmorClass,  
    op: RuleOp::Add(1),  
}
```
---

### 7. Barkskin Example (Override Phase)
```Rust
Rule {  
    id: "barkskin_ac",  
    phase: RulePhase::Override,  
    inputs: vec![StatId::ArmorClass],  
    output: StatId::ArmorClass,  
    op: RuleOp::Max(16),  
}
```
Because overrides run before modifiers, the result is correct.

---

### 8. Engine Structure
```
rule_engine/  
 ├── rule_engine.rs  
 ├── rule_graph.rs  
 ├── rule_phase.rs  
 └── evaluator.rs
```
---

##### Engine Struct
```Rust
pub struct RuleEngine {  
    graphs: HashMap<RulePhase, RuleGraph>,  
}
```
---

##### Evaluation
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
---

### 9. Interaction With Cached Character Sheets

Your previous caching system becomes:
```
Character  
   ↓  
RuleEngine  
   ↓  
DerivedStats  
   ↓  
CachedCharacterSheet
```
Recompute only when:
```
inventory changes  
level changes  
conditions change  
equipped items change  
spell effects change
```
Then run the phased engine.

---

### 10. Why Phases Work Well With Data-Driven Rules

They give:

##### Deterministic behavior

Rules always apply in predictable order.

---

##### Simpler rule definitions

Each rule only needs:
```
phase  
inputs  
output  
operation
```
No complex precedence logic.

---

##### Prevents cycles

Cycles across phases cannot occur.

Example:
```
Derived → Modifier → Derived
```
Impossible.

---

##### Faster evaluation

Graphs are smaller.
```
instead of 500 rules  
→ 5 graphs of ~100 rules
```
---

### 11. Real Example: AC Calculation Pipeline
```
Phase Base  
    armor AC  
    dex mod  
  
Phase Override  
    barkskin  
    mage armor  
  
Phase Modifier  
    shield  
    ring of protection  
    cover bonuses  
  
Phase Derived  
    conditional AC  
  
Phase Finalize  
    clamp
```
---

### 12. When a Rule Depends on a Later Phase

Rare but possible.

Example:
```
Passive perception = perception + 10
```
This is fine because:
```
perception → modifier phase  
passive perception → derived phase
```
So the dependency flows forward.

---

### 13. What This Prevents

Without phases you risk:
```
AC rule depends on AC  
AC depends on modifier  
modifier depends on AC
```
Phases eliminate this ambiguity.

---

### 14. The Big Picture

Your engine now looks like:
```
Character State  
     │  
     ▼  
Inventory System  
     │  
     ▼  
Rule Engine  
  ├─ Base Graph  
  ├─ Override Graph  
  ├─ Modifier Graph  
  ├─ Derived Graph  
  └─ Finalize Graph  
     │  
     ▼  
Derived Stats  
     │  
     ▼  
Cached Character Sheet
```
This is **very close to how professional RPG engines handle rule evaluation**.

---

✅ **Key result:**  
Phases give you **predictable rule ordering without adding complexity or traits**.

---

If you'd like, the next useful step would be showing **one complete end-to-end rule flow** (armor + barkskin + shield + ring + dex + fighting style) implemented in this system so you can see how the full pipeline executes.

# How will the 2024 limitations on wildshapes fit into our earlier discussion of 2024 rule changes?

The **2024 Wild Shape changes** actually fit **very naturally** into the architecture we already designed (phased rule engine + data-driven rules + edition flags). The important thing is that **2024 Wild Shape is no longer a stat block replacement system** like 2014—it’s closer to a **template overlay** applied to the character.

So the implementation becomes **much simpler** in the system we built.

I'll break this into:

1. What changed in 2024
2. What that means architecturally
3. How it fits into the rule phases
4. How to represent it in rules data
5. What changes (if any) are needed to the engine

---

### 1. What Changed in 2024 Wild Shape

#### 2014 Wild Shape

The druid **replaces their character sheet** with the beast stat block.

Replace:
```
STR  
DEX  
CON  
AC  
HP  
speed  
senses  
attacks
```
Keep:
```
INT  
WIS  
CHA  
proficiency bonus  
skills (some cases)
```
This creates major problems:

- partial stat replacement
- skill recalculation
- equipment interaction
- AC overrides
- HP pools

It’s basically **character swapping**.

---

#### 2024 Wild Shape

In 2024 the druid **keeps their character stats**.

The form provides **modifiers instead of replacements**.

Examples (simplified):
```
temporary hit points  
movement types  
natural attacks  
size change  
special abilities
```
So Wild Shape becomes:
```
Character  
   + form bonuses  
   + form abilities
```
Not:
```
Character → replaced by creature
```
This is a **huge simplification for rule engines**.

---

### 2. Architectural Implication

Instead of:
```
Character → BeastStatBlock
```
we implement:
```
Character  
  + ActiveForm
```
Where the form is essentially a **rule package**.
```Rust
ActiveEffect {  
    rules: Vec<Rule>  
}
```
So the bear form might contain rules like:
```
add temporary HP  
grant climb speed  
grant natural attack  
change size
```
This fits directly into our **existing rule system**.

---

### 3. How It Fits Into Rule Phases

Wild Shape rules land in several phases.

##### Phase 0 — State
```
active_form = Bear
```
---

##### Phase 1 — Base

Some forms may set base movement types.

Example:
```
climb_speed = 40
```
---

##### Phase 2 — Overrides

If a form sets minimum AC or special defenses.

Example:
```
AC >= 13
```
---

##### Phase 3 — Modifiers

Typical Wild Shape bonuses:
```
temporary HP  
damage bonuses  
attack bonuses  
movement bonuses
```
---

##### Phase 4 — Derived

New attacks:
```
Claw attack  
Bite attack
```
---

### Phase 5 — Finalization

Clamps and cleanup.

---

### 4. Representing Wild Shape in Data

Wild Shape becomes a **FormDefinition**.
```Rust
pub struct FormDefinition {  
    pub id: String,  
    pub name: String,  
    pub size: Option<Size>,  
    pub rules: Vec<Rule>,  
    pub granted_actions: Vec<Action>,  
}
```
Example:
```Rust
FormDefinition {  
    id: "bear_form",  
    name: "Bear",  
    size: Some(Size::Large),  
  
    rules: vec![  
        Rule::temp_hp("wildshape_temp_hp"),  
        Rule::set_speed("climb", 40),  
    ],  
  
    granted_actions: vec![  
        Action::natural_attack("claw"),  
        Action::natural_attack("bite"),  
    ]  
}
```
---

### 5. Character State

We just add:
```Rust
pub struct CharacterState {  
    pub active_form: Option<FormId>,  
}
```
When evaluating rules:
```Rust
if active_form != None:  
    load form rules
```
---

### 6. Form Rules Enter the Rule Engine

During evaluation:
```
base rules  
+ item rules  
+ feat rules  
+ spell rules  
+ form rules
```
Graph becomes:
```
RuleGraph = all active rules
```
---

### 7. Temporary HP Handling

2024 Wild Shape grants **temporary HP instead of beast HP**.

This works perfectly with the resource system we already discussed.

Example rule:
```
temp_hp = druid_level × X
```
Representation:
```Rust
RuleOp::GrantTempHP {  
    amount: Expr::Mul(  
        StatId::DruidLevel,  
        3  
    )  
}
```
---

### 8. Natural Attacks

Natural attacks are just **actions added to the character**.

Example:
```
Claw attack  
damage: 1d8 + WIS
```
Representation:
```Rust
pub struct ActionDefinition {  
    pub id: String,  
    pub attack_type: AttackType,  
    pub damage: DamageExpr,  
}
```
Wild Shape simply **adds actions**.

---

### 9. Movement Changes

Forms grant new speeds.

Example:
```
climb speed  
swim speed  
burrow speed
```
Rules:
```Rust
set_speed(climb, 40)
```
or
```Rust
add_speed(swim, 30)
```
---

### 10. Size Changes

Size becomes a stat.

Rule:
```
size = Large
```
Or:
```
size = max(size, Large)
```
Used later for:
```
grapple rules  
reach  
space
```
---

### 11. Interaction With Inventory

Forms usually **suppress equipment**.

Instead of deleting equipment effects, we use **conditions**.

Example rule:
```
while wildshape:  
    disable equipment effects
```
Representation:
```Rust
RuleCondition::NotWildShapeRestricted
```
or
```
equipment_effect.requires_humanoid_form
```
---

### 12. Supporting BOTH 2014 and 2024

Our edition system handles this easily.
```Rust
Rule {  
    edition: Edition::E2014  
}
```
vs
```Rust
Rule {  
    edition: Edition::E2024  
}
```
---

#### 2014 Wild Shape Implementation

You would still support it using:
```
StatOverrideLayer
```
Example:
```
beast STR replaces character STR
```
But this system is **much more complex**, which is why 2024 removed it.

---

### 13. Engine Changes Required

Almost none.

We only add:
```
ActiveForm  
FormDefinition
```
Evaluation:
```Rust
rules = character_rules  
rules += item_rules  
rules += feat_rules  
rules += spell_rules  
rules += form_rules
```
---

### 14. Final Architecture
```
Character  
  │  
  ├─ Inventory  
  ├─ Features  
  ├─ Conditions  
  ├─ Active Spells  
  └─ Active Form  
          │  
          ▼  
     FormDefinition  
          │  
          ▼  
       Rule Set  
          │  
          ▼  
Rule Engine (phases)  
          │  
          ▼  
Derived Stats  
          │  
          ▼  
Character Sheet Cache
```
---

### 15. Why 2024 Wild Shape Is Much Easier for Engines

Because it becomes **just another effect source**, like:
```
spell effects  
magic items  
feats  
conditions
```
So the engine doesn't need special-case logic.

---

✅ **Key takeaway**

The architecture we built already supports 2024 Wild Shape almost perfectly because it treats **rules as modular effect sources**.

---

If you'd like, I can also show **the single hardest rule-engine problem in 5e** that we have not addressed yet (and that almost every character builder struggles with):

**proficiency resolution for skills/tools with overlapping sources and expertise.**

# Please do \[Show proficiency resolution incl overlapping sources, expertise], in brief

The **hardest rule-engine problem in 5e character builders** is usually **proficiency resolution** for:

- skills
- tools
- saving throws
- weapons/armor
- languages

Because **multiple rules can grant or modify the same proficiency**, and they interact in complicated ways:
```
• grant proficiency  
• grant expertise (double proficiency)  
• grant half-proficiency  
• replace proficiency choices  
• conditional proficiencies  
• “if already proficient, choose another”
```
This creates **overlapping rule sources with priority and stacking rules**, which is much harder than simple numeric modifiers.

Below is the minimal architecture that fits into the system we designed.

---

### 1. The Core Problem

Example character:
```
Background: Stealth proficiency  
Rogue: Stealth proficiency  
Rogue Expertise: Stealth expertise  
Feat: Skill Expert (choose Stealth)  
Bard: Jack of All Trades (half proficiency to untrained skills)
```
Result must be:
```
Stealth → Expertise (2× proficiency)
```
Not:
```
3× proficiency
```
So the engine must **collapse multiple rules into a single final proficiency tier**.

---

### 2. Represent Proficiency as a Tier

Instead of stacking modifiers, represent proficiency as a **tiered state**.
```Rust
enum ProficiencyTier {  
    None,  
    Half,  
    Proficient,  
    Expertise,  
}
```
This mirrors the rules hierarchy.
```
None < Half < Proficient < Expertise
```
---

### 3. Use “Max Tier” Resolution

Rules don’t stack; they **raise the tier**.

Example rule:
```Rust
RuleOp::SetProficiency {  
    skill: Skill::Stealth,  
    tier: ProficiencyTier::Proficient  
}
```
Expertise rule:
```Rust
RuleOp::SetProficiency {  
    skill: Skill::Stealth,  
    tier: ProficiencyTier::Expertise  
}
```
Evaluation becomes:
```
final tier = max(all granted tiers)
```
---

### 4. Jack of All Trades (Half Proficiency)

This is conditional.

Rule:
```
if skill tier == None  
    tier = Half
```
Representation:
```Rust
RuleOp::ConditionalHalfProficiency
```
Evaluation order:
```
Base proficiency rules  
→ Expertise rules  
→ Half proficiency rules
```
This fits nicely in **rule phases**.

---

### 5. Skill Replacement Rules

Some features say:
```
Gain proficiency in a skill.  
If already proficient, choose another skill.
```
This is **not a rule engine problem**, it is a **character build choice**.

So store it in character state:
```Rust
FeatureChoice {  
    feature_id: "background_skill",  
    selected_skill: Skill::Stealth  
}
```
The rule engine then simply applies:
```
Stealth proficiency
```
---

### 6. Final Skill Bonus Calculation

Once proficiency tier is resolved:
```
skill_bonus =  
    ability_mod  
  + proficiency_bonus × tier_multiplier
```
Multiplier table:
```
None        → 0  
Half        → 0.5  
Proficient  → 1  
Expertise   → 2
```
---

### 7. Where It Fits in Rule Phases
```
Phase Base  
    skill ability mods  
  
Phase Modifier  
    proficiency grants  
    expertise grants  
  
Phase Derived  
    compute skill bonuses
```
---

### 8. Why This Works Well

It avoids the biggest pitfalls:

Bad design (common in character builders):
```
+ proficiency  
+ proficiency  
+ expertise  
+ half proficiency
```
Correct design:
```
compute tier  
then apply multiplier
```
---

### 9. Final Result in the Engine
```
Rules grant tiers  
→ engine resolves highest tier  
→ derived stat multiplies proficiency bonus
```
Simple, deterministic, and **data-driven**.

---

✅ **Key idea:**  
Treat proficiency as a **state machine (tier)** instead of **stackable modifiers**.

---

If you'd like, I can also show the **second most difficult rule-engine problem in 5e systems** (which affects AC, attack rolls, and damage):

**bonus stacking rules ("bonuses of the same name don't stack").**

It’s a subtle problem and very relevant to the rule graph design we built.

# Is that problem \[bonus stacking rules] not solved by tracking effect sources like we previously discussed and eliminating duplicates?

Yes — **tracking effect sources and eliminating duplicates solves a large part of the stacking problem**, and it fits well with the architecture we designed earlier. However, **it does not solve the entire problem**, because 5e stacking rules are **not only about duplicates**. There are three distinct cases:

1. Duplicate effects (same source applied twice)  
2. Same-type bonuses that don't stack  
3. Overrides or mutually exclusive effects

Your earlier **effect-source tracking** handles case **#1** perfectly. The remaining cases require a bit more structure.

I'll briefly explain the full model.

---

### 1. Case 1 — Duplicate Effects (Solved by Source Tracking)

Example:
```
Bless applied twice  
Haste applied twice  
Same magic item added twice
```
If effects include a **source identifier**, duplicates can be ignored.

Example effect:
```Rust
EffectInstance {  
    source_id: "spell_bless",  
    target: AttackRoll,  
    modifier: +1d4,  
}
```
Engine rule:
```
Only one active effect per source_id
```
So applying Bless twice produces only one instance.

This is exactly what you suggested and **works well**.

---

### 2. Case 2 — Same-Type Bonuses That Don't Stack

This is the more subtle rule.

Examples:
```
Two effects both give +2 AC as "cover"  
Two features both give +1 AC as "fighting style"  
Two speed bonuses of the same type
```
The rules often say:
```
Bonuses of the same name don't stack.
```
Example:
```
Shield of Faith (+2 AC)  
Cover (+2 AC)
```
These **do stack**, because they are different types.

But:
```
Two instances of half cover
```
do **not** stack.

---

#### Solution: Bonus Categories

Each modifier has a **bonus category**.
```Rust
enum BonusCategory {  
    Untyped,  
    Cover,  
    FightingStyle,  
    Spell,  
    Item,  
}
```
Rules:
```
Same category → keep highest  
Different category → stack
```
Example evaluation:
```
Half cover +2  
Three quarters cover +5
```
Result:
```
+5
```
Because both are `Cover`.

---

### 3. Case 3 — Overrides (Mutually Exclusive Rules)

Examples:
```
Mage Armor  
Barkskin  
Unarmored Defense  
Heavy Armor
```
These aren't additive bonuses — they **replace base calculations**.

Example:
```
AC = 13 + DEX (Mage Armor)  
AC = 10 + DEX + WIS (Monk)  
AC = armor AC (Armor)
```
These must compete as **alternative base formulas**.

Earlier we solved this with:
```Rust
RulePhase::Override
```
and sometimes:
```
max()  
priority
```
Example:
```Rust
AC = max(current_ac, 16)   // barkskin
```
So overrides are handled separately from stacking.

---

### 4. Putting It Together

Your rule engine effectively has **three protection layers**:

##### 1️⃣ Source Deduplication

Prevents:
```
same spell twice  
same item twice  
same effect twice
```
---

##### 2️⃣ Bonus Categories

Controls stacking:
```
same type → highest only  
different types → stack
```
---

##### 3️⃣ Rule Phases / Overrides

Handles replacement logic:
```
base AC  
replacement formulas  
additive bonuses
```
---

### 5. Example: AC Calculation

Effects:
```
Mage Armor → AC = 13 + DEX  
Shield of Faith → +2 AC (Spell)  
Ring of Protection → +1 AC (Item)  
Half Cover → +2 AC (Cover)  
Three Quarters Cover → +5 AC (Cover)
```
Evaluation:
```
Base Phase  
    AC = 13 + DEX  
  
Modifier Phase  
    +2 (Spell)  
    +1 (Item)  
    +5 (Cover)  // replaces +2 cover
```
Final result:
```
AC = base + 8
```
---

### 6. Why Source Tracking Alone Isn't Enough

Consider:
```
Half Cover +2  
Three Quarters Cover +5
```
They are **different sources**, so deduplication wouldn't remove one.

But rules say:
```
only the best cover applies
```
Which is why **bonus categories** exist.

---

### 7. Minimal Data Model

A modifier becomes:
```Rust
Modifier {  
    stat: StatId,  
    value: ModifierValue,  
    category: BonusCategory,  
    source_id: SourceId,  
}
```
Evaluation algorithm:

1. remove duplicate sources  
2. group modifiers by category  
3. keep highest in each category  
4. sum remaining modifiers

---

### 8. Why This Works Well With the Rule Graph

Because rules produce **effects**, not final numbers.
```
Rules  
  ↓  
Effects  
  ↓  
Stacking Resolver  
  ↓  
Final Modifier
```
So stacking logic is **centralized**, not embedded in each rule.

---

✅ **Summary**

Your earlier idea solves **duplicate effects**, but full stacking support requires:
```
Source tracking  
+ bonus categories  
+ override phases
```
Together these correctly model almost all 5e stacking rules.

---

If you'd like, I can also show the **single data structure improvement** used in many RPG engines that makes stacking, overrides, conditions, and durations **dramatically simpler** to manage. It fits perfectly with the system we've built and prevents a lot of subtle bugs later.

# How will the CLI interact with the architecture we've built thus far to function? How will the GUI differ in this respect?

Your architecture so far separates nicely into **three layers**:
```
Rules Data Layer  
Engine Layer  
Interface Layer (CLI / GUI / Web)
```
The **CLI and GUI do not interact with the rules engine differently in principle**. The difference is **how they drive the engine and present results**.

The important idea is:
```
Interfaces do not implement rules.  
Interfaces mutate character state and request evaluation.
```
The rule engine remains **completely UI-agnostic**.

---

### 1. The Core Architecture We Built

Your application roughly looks like this:
```
rules/  
    classes  
    species  
    spells  
    feats  
    items  
    forms  
  
engine/  
    rule_engine  
    rule_graph  
    stacking_resolver  
    proficiency_resolver  
  
character/  
    character  
    inventory  
    resources  
    choices  
  
sheet/  
    derived_stats  
    cached_character_sheet  
  
interfaces/  
    cli  
    gui
```
Data flow:
```
User Action  
     ↓  
Character State Mutation  
     ↓  
Rule Engine Evaluation  
     ↓  
Derived Stats  
     ↓  
Character Sheet  
     ↓  
UI Display
```
---

### 2. What the CLI Actually Does

The CLI is basically a **command dispatcher** that mutates character state.

Example command:
```
add-item longsword
```
CLI flow:
```
parse command  
→ update character  
→ run rule engine  
→ print result
```
Pseudo-flow:
```Rust
fn handle_command(cmd: Command, character: &mut Character) {  
  
    match cmd {  
  
        Command::AddItem(id) => {  
            character.inventory.add(id);  
        }  
  
        Command::LevelUp => {  
            character.level += 1;  
        }  
  
        Command::Equip(item) => {  
            character.inventory.equip(item);  
        }  
    }  
  
    engine.evaluate(character);  
}
```
Then the CLI prints the sheet:
```
Strength: 18  
AC: 19  
HP: 42  
Attack Bonus: +7
```
So the CLI is **stateless UI glue**.

---

### 3. CLI Interaction Pattern

The CLI tends to follow **command → recompute → print**.

Example session:
```
> create-character  
> set-class fighter  
> set-level 5  
> add-item chain-mail  
> equip chain-mail  
> show-sheet
```
Each command does:
```
mutate state  
→ recompute sheet
```
This is cheap because the engine is **incremental and cached**.

---

### 4. What the GUI Does Differently

The GUI has **persistent state and event-driven updates**.

Instead of commands:
```
user clicks button  
user changes dropdown  
user edits field
```
Example:
```
User selects feat
```
GUI flow:
```
UI event  
→ update character state  
→ invalidate cache  
→ recompute derived stats  
→ re-render UI
```
Pseudo:
```Rust
fn on_feat_selected(feat: FeatId) {  
    character.add_feat(feat);  
  
    engine.evaluate(&mut character);  
  
    ui.render_sheet(&character.sheet);  
}
```
---

### 5. The Key Difference

##### CLI
```
Command driven  
Stateless between commands  
Full output each time
```
Example:
```
> show-sheet
```
---

##### GUI
```
Event driven  
Persistent UI state  
Partial UI updates
```
Example:
```
only AC widget updates
```
---

### 6. Character Sheet Caching Becomes Important

Because the GUI may update frequently.

Example:
```
user toggles armor
```
Only a few stats change.

So we rely on:
```
CachedCharacterSheet
```
Flow:
```
State Change  
→ Dirty Flags  
→ Recompute affected stats  
→ Update UI components
```
---

### 7. CLI vs GUI Data Structures

The **character object is identical**.
```
Character {  
    state  
    inventory  
    features  
    resources  
    sheet_cache  
}
```
Both interfaces call:
```
engine.evaluate(character)
```
So the engine is reused.

---

### 8. CLI Module Structure

Example:
```
cli/  
    main.rs  
    commands.rs  
    parser.rs
```
Command enum:
```Rust
enum Command {  
  
    CreateCharacter,  
    SetClass(ClassId),  
    AddItem(ItemId),  
    EquipItem(ItemId),  
    ShowSheet,  
}
```
Execution:
```Rust
fn execute(command: Command) {  
    command_handler(command, &mut character);  
}
```
---

### 9. GUI Module Structure

Example:
```
gui/  
    app.rs  
    character_view.rs  
    inventory_view.rs  
    feature_view.rs
```
UI components:
```
CharacterSheetView  
InventoryView  
SpellView  
FeatureView
```
Each listens to state changes.

---

### 10. Inventory Example

CLI:
```
> add-item backpack  
> add-item rope  
> put rope backpack
```
GUI:
```
drag rope → backpack
```
Both produce the same internal mutation:
```
inventory.move_item(rope, backpack)
```
Then:
```
engine.evaluate(character)
```
---

### 11. Spellcasting Example

CLI:
```
> cast bless
```
Adds:
```Rust
ActiveEffect {  
    source: SpellId::Bless  
}
```
Then recompute.

GUI:
```
User presses **Cast Bless button**.
```
Same mutation occurs.

---

### 12. Why This Architecture Works Well

Because **UI never touches rules**.

Rules remain:
```
data driven  
deterministic  
testable
```
You can even test the engine **without any interface**.

Example unit test:
```Rust
#[test]  
fn mage_armor_sets_ac() {  
  
    character.add_spell_effect("mage_armor");  
  
    engine.evaluate(&mut character);  
  
    assert_eq!(character.ac(), 16);  
}
```
---

### 13. Supporting a Future Web App

Because of the separation:
```
engine = library crate
```
Interfaces become:
```
cli/  
gui/  
web-api/
```
The web app would work like:
```
HTTP request  
→ mutate character  
→ evaluate engine  
→ return JSON sheet
```
Example:
```JSON
{  
  "ac": 19,  
  "hp": 42,  
  "attack_bonus": 7  
}
```
---

### 14. Final Architecture
```
                RULE DATA  
            (json / yaml / db)  
                    │  
                    ▼  
              RULE ENGINE  
      (graphs, phases, stacking)  
                    │  
                    ▼  
            CHARACTER STATE  
                    │  
                    ▼  
         CACHED CHARACTER SHEET  
           │          │         │  
           ▼          ▼         ▼  
         CLI         GUI      Web API
```
---

✅ **Key idea**

The **CLI and GUI do not interact with the rule engine differently** — they only differ in how they **trigger state changes and display results**.

---

If you'd like, I can also show the **cleanest Rust crate layout for this entire project**, which helps avoid circular dependencies and keeps the rule engine reusable for CLI, GUI, and web versions.

# Pull everything together to show the data representations for all of the core elements (classes, subclasses, spells, feats, items, etc, etc) in their relevant formats (YAML/DB).

Below is a **cohesive data model** for representing **all core 5e elements** in the architecture we built. The design goals were:

- **Trait-free, data-driven rules**
- **Works with rule phases + dependency graph**
- **Transfers cleanly between YAML ↔ database**
- **Supports 2014 and 2024 rules**
- **Separates rules from descriptive text**

I’ll show:

1. Overall rules data architecture
2. Core shared schemas
3. YAML representations for each game element
4. Equivalent database schema
5. How the engine loads and uses this data

---

### 1. Rules Data Architecture

Everything in the system is treated as a **Feature Source** that provides rules.
```
class  
subclass  
species  
background  
feat  
spell  
item  
condition  
wildshape form
```
Each source contains:
```
id  
edition  
metadata  
choices  
rules  
description
```
Conceptually:
```
FeatureSource  
   ├─ metadata  
   ├─ rules  
   ├─ choices  
   └─ text
```
The rule engine **only reads rules**, not descriptive text.

---

### 2. Shared Rule Structures (YAML)

#### Rule
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
Example:
```YAML
- id: ring_protection_ac  
  phase: modifier  
  target: armor_class  
  operation: add  
  value: 1  
  category: item
```
---

#### Expressions

Expressions allow rules to reference character data.
```YAML
value:  
  expr: add  
  args:  
    - stat: dex_mod  
    - 13
```
Example:
```YAML
value:  
  expr: mul  
  args:  
    - stat: druid_level  
    - 3
```
---

#### Conditions

Rules can be conditional.
```YAML
condition:  
  has_condition: raging
```
or
```YAML
condition:  
  equipped: shield
```
---

### 3. Classes
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
---

### 4. Subclasses
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
---

### 5. Features

Features are reusable rule packages.
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
---

### 6. Feats
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
---

### 7. Species (Race / Species System)

Supports both editions.
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
---

### 8. Backgrounds
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
---

### 9. Spells
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
---

### 10. Spell Upcasting

Represented explicitly.
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
Example:
```
level 3 cast  
= 3d8 + modifier
```
---

### 11. Items
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
---

### 12. Weapons
```YAML
id: longsword  
type: weapon  
  
damage: 1d8  
damage_type: slashing  
  
properties:  
  - versatile  
  
rules: []
```
---

### 13. Armor
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
---

### 14. Containers
```YAML
id: backpack  
type: container  
  
capacity:  
  weight: 30
```
Inventory items themselves **have no location in rules data** — location is runtime state.

---

### 15. Conditions
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
---

### 16. Wild Shape Forms (2024)
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
---

### 17. Database Representation

A relational DB version would normalize the same structure.

##### Tables
```
entities  
rules  
expressions  
choices  
text_blocks
```
---

##### entities
```
id  
type  
edition  
metadata_json
```
---

##### rules
```
id  
entity_id  
phase  
target  
operation  
category  
condition_json  
expression_id
```
---

##### expressions
```
id  
type  
args_json
```
---

##### text_blocks
```
entity_id  
description  
source
```
---

### 18. Character Runtime Data

Character state is **not part of rules data**.

Runtime state:
```
Character {  
    level  
    inventory  
    features  
    active_spells  
    conditions  
    resources  
}
```
---

### 19. Engine Loading

At startup:
```
YAML files  
   ↓  
deserialize  
   ↓  
RuleRegistry
```

```
RuleRegistry  
  classes  
  feats  
  spells  
  items  
  features
```
---

### 20. Final Architecture
```
rules_data/  
   classes.yaml  
   subclasses.yaml  
   species.yaml  
   feats.yaml  
   spells.yaml  
   items.yaml  
   features.yaml  
   conditions.yaml  
   forms.yaml  
  
engine/  
   rule_engine  
   rule_graph  
   stacking_resolver  
  
character/  
   character  
   inventory  
   choices  
  
sheet/  
   derived_stats  
   cached_sheet  
  
interfaces/  
   cli  
   gui  
   web
```
---

✅ This design gives you:

- **clean rule definitions**
- **edition compatibility**
- **graph evaluation**
- **easy YAML editing**
- **optional DB storage**
- **transferable data**

---

If you'd like, I can also show the **final missing piece** for a full system like this:  
the **complete `StatId` list and derived stat graph**, which defines every stat the rule engine can compute (AC, attacks, spell DC, passive perception, etc.).

# Please do \[Show complete StatId list and graph], then suggest how how to format homebrew data into a single file to be passed around.

I'll do this in two parts:

1. **The complete `StatId` universe + derived stat graph** used by the rule engine
2. **A clean portable format for homebrew content** that fits into the same architecture and can be shared as a single file

The goal is to ensure the system remains:
```
data-driven  
edition-aware  
rule-engine compatible  
easy to extend with homebrew
```
---

### Part 1 — The `StatId` Universe

The rule engine operates on a **fixed set of stat identifiers**.  
Rules modify these stats, and derived rules compute others.

This keeps the engine simple and avoids dynamic stat creation.

##### Core Enum
```Rust
pub enum StatId {  
  
    // ability scores  
    Strength,  
    Dexterity,  
    Constitution,  
    Intelligence,  
    Wisdom,  
    Charisma,  
  
    // ability modifiers  
    StrengthMod,  
    DexterityMod,  
    ConstitutionMod,  
    IntelligenceMod,  
    WisdomMod,  
    CharismaMod,  
  
    // core character  
    Level,  
    ProficiencyBonus,  
    HitPointsMax,  
    HitPointsCurrent,  
    TemporaryHitPoints,  
  
    // defenses  
    ArmorClass,  
    Initiative,  
  
    // speed  
    SpeedWalk,  
    SpeedFly,  
    SpeedSwim,  
    SpeedClimb,  
    SpeedBurrow,  
  
    // size  
    Size,  
  
    // attacks  
    MeleeAttackBonus,  
    RangedAttackBonus,  
    SpellAttackBonus,  
  
    // spellcasting  
    SpellSaveDC,  
  
    // passive scores  
    PassivePerception,  
    PassiveInsight,  
    PassiveInvestigation,  
  
    // skills  
    SkillAcrobatics,  
    SkillAnimalHandling,  
    SkillArcana,  
    SkillAthletics,  
    SkillDeception,  
    SkillHistory,  
    SkillInsight,  
    SkillIntimidation,  
    SkillInvestigation,  
    SkillMedicine,  
    SkillNature,  
    SkillPerception,  
    SkillPerformance,  
    SkillPersuasion,  
    SkillReligion,  
    SkillSleightOfHand,  
    SkillStealth,  
    SkillSurvival,  
  
    // saving throws  
    SaveStrength,  
    SaveDexterity,  
    SaveConstitution,  
    SaveIntelligence,  
    SaveWisdom,  
    SaveCharisma,  
}
```
This enum becomes the **entire dependency graph vocabulary**.

Rules always reference these IDs.

Example rule:
```YAML
target: armor_class  
operation: add  
value: 1
```
---

### Derived Stat Graph

Derived rules compute stats based on dependencies.

Example graph:
```
Strength  
  └── StrengthMod  
  
Dexterity  
  └── DexterityMod  
       └── Initiative  
       └── ArmorClass  
       └── SkillStealth
```
##### Ability Modifiers
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
---

##### Proficiency Bonus
```YAML
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
---

##### Skill Calculation

Skills depend on:
```
ability mod  
+ proficiency tier
```
Example:
```YAML
target: skill_stealth  
operation: set_expr  
value:  
  expr: add  
  args:  
    - stat: dexterity_mod  
    - stat: stealth_proficiency_bonus
```
---

##### Passive Perception
```YAML
target: passive_perception  
operation: set_expr  
value:  
  expr: add  
  args:  
    - 10  
    - stat: skill_perception
```
---

##### Spell Save DC
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
---

##### Armor Class (Simplified)

AC pipeline:
```
Base AC  
→ overrides (mage armor, barkskin)  
→ modifiers (shield, ring, cover)
```
Example derived rule:
```YAML
target: armor_class  
operation: set_expr  
value:  
  expr: add  
  args:  
    - stat: base_ac  
    - stat: dexterity_mod
```
---

### Derived Stat Evaluation Pipeline

Final pipeline:
```
Phase 0: state  
Phase 1: base  
Phase 2: overrides  
Phase 3: modifiers  
Phase 4: derived  
Phase 5: finalize
```
Graph evaluation happens **inside each phase**.

---

### Part 2 — Portable Homebrew Format

Now we need a **single shareable file** that can contain:
```
classes  
subclasses  
spells  
items  
feats  
features  
conditions  
forms
```
This file must load directly into the rules registry.

---

### Homebrew Package Format

Use a **bundle file**:
```
*.dndpack
```
Internally it is just **YAML**.

Example:
```YAML
package:  
  name: "Arcanist Expansion"  
  version: "1.0"  
  author: "Jane Doe"  
  
content:  
  
  spells:  
    - id: gravity_lance  
      level: 3  
      school: evocation  
  
      rules:  
        - target: spell_attack_bonus  
          phase: modifier  
          operation: add  
          value: 1  
  
  feats:  
    - id: arcane_marksman  
  
      rules:  
        - target: ranged_attack_bonus  
          phase: modifier  
          operation: add  
          value: 1  
  
  items:  
    - id: cloak_of_the_void  
  
      rarity: rare  
  
      rules:  
        - target: armor_class  
          phase: modifier  
          operation: add  
          value: 2
```
---

### Loading Homebrew

Engine startup:
```
core rules  
+ homebrew packs
```
Load order:
```
core  
→ supplements  
→ homebrew
```
Registry merges them.

---

### Preventing ID Conflicts

Require **namespaced IDs**.

Example:
```
core.spell.fireball  
homebrew.gravity_lance
```
or
```
jane.gravity_lance
```
---

### Homebrew Distribution

A `.dndpack` file can contain:
```
rules.yaml  
art/  
text/
```
Optional ZIP structure:
```
gravity-expansion.dndpack  
 ├─ rules.yaml  
 ├─ README.md  
 └─ art/
```
The engine loads only `rules.yaml`.

---

### CLI Support

CLI command:
```
load-pack gravity-expansion.dndpack
```
Registry adds rules dynamically.

---

### GUI Support

GUI can install packs:
```
Settings → Install Homebrew
```
Then reload rules.

---

### Final Unified Data Flow
```
core rules (YAML)  
      │  
      ▼  
rule registry  
      ▲  
      │  
homebrew packs (.dndpack)  
      │  
      ▼  
rule engine  
      │  
      ▼  
character state  
      │  
      ▼  
derived stats  
      │  
      ▼  
character sheet
```
---

### Why This Design Works Well

It supports:
```
core rules  
supplements  
homebrew  
custom campaigns
```
without modifying the engine.

Everything remains **data-driven**.

---

✅ **Final Result**

You now have:

- a **fixed stat vocabulary**
- a **deterministic rule evaluation graph**
- **portable rules data**
- **shareable homebrew packages**

---

If you'd like, the next extremely useful step would be showing **the final 5e system component we have not yet modeled explicitly**:

**actions and combat resolution (attacks, damage rolls, advantage/disadvantage, and effects)** — which interact with this stat system but require a slightly different data structure.

# What are the tradeoffs of a single YAML dndpack vs a ZIP file and which one might be better for this system? Show how the loader code would unpack the file.

There are two realistic formats for a **`.dndpack` homebrew package** in this system:

1. **Single YAML file**
2. **ZIP archive containing YAML + optional assets**

Both work with the rule engine we designed, but they have different tradeoffs.

I'll compare them first, then show **Rust-style loader code** for both.

---

### 1. Single YAML `.dndpack`

Example file:
```YAML
package:  
  name: "Arcanist Expansion"  
  version: "1.0"  
  author: "Jane Doe"  
  
content:  
  
  spells:  
    - id: jane.gravity_lance  
      level: 3  
      school: evocation  
  
      rules:  
        - target: spell_attack_bonus  
          phase: modifier  
          operation: add  
          value: 1  
  
  feats:  
    - id: jane.arcane_marksman  
  
      rules:  
        - target: ranged_attack_bonus  
          phase: modifier  
          operation: add  
          value: 1
```
---

#### Advantages

##### Simplicity

No archive parsing needed.
```
read file  
deserialize YAML  
done
```
This is extremely reliable and easy to debug.

---

##### Human Editable

Users can easily open and edit the file.

Good for:
```
homebrew sharing  
Git repositories  
modding
```
---

##### CLI Friendly

Example:
```
dnd load-pack arcanist.dndpack
```
No extraction required.

---

##### Version Control Friendly

Diffs remain readable.
```Diff
+ spell: gravity_lance  
+ rule: +1 spell attack
```
---

#### Disadvantages

##### Cannot Bundle Assets

You cannot easily include:
```
images  
icons  
spell art  
documentation
```
---

##### Large Files

Big packs may become very large.

---

##### Harder for GUI Distribution

If you want a mod manager with assets, YAML alone becomes limiting.

---

### 2. ZIP `.dndpack`

Structure:
```
gravity-expansion.dndpack  
 ├─ rules.yaml  
 ├─ README.md  
 └─ art/  
      gravity_lance.png
```
The `.dndpack` extension is just a **ZIP archive**.

---

#### Advantages

##### Asset Support

You can include:
```
icons  
portraits  
spell art  
sound effects
```
Very useful for a GUI.

---

##### Modular Structure

Large rule sets can be split:
```
rules/  
   spells.yaml  
   feats.yaml  
   classes.yaml
```
---

##### Better Distribution

Most game mod systems use this approach.

Examples:

- RimWorld
- Factorio
- Foundry VTT

---

#### Disadvantages

##### More Complex Loader

You must:
```
open zip  
locate rules file  
parse YAML
```
---

##### Harder to Edit

Users must unzip first.

---

##### Version Control Worse

Binary archive diffs are meaningless.

---

### 3. Which Is Better for This System?

For **this architecture**, the best option is usually:

**ZIP `.dndpack` with YAML inside**

because the system will likely evolve to include:
```
icons  
spell descriptions  
localization  
reference text
```
However:

A **single YAML format is excellent for development and testing**.

Many engines support both.

---

### Recommended Hybrid

Support both:
```
.dndpack (zip)  
.yaml (raw rules)
```
Loader detects which type.

---

### 4. Loader Architecture

High level:
```
load_pack(path)  
  ├─ if yaml → parse  
  └─ if zip → extract rules.yaml → parse
```
---

### Rust Data Structures
```Rust
use serde::Deserialize;  
  
#[derive(Deserialize)]  
pub struct DndPack {  
    pub package: PackageMeta,  
    pub content: PackContent,  
}  
  
#[derive(Deserialize)]  
pub struct PackageMeta {  
    pub name: String,  
    pub version: String,  
    pub author: Option<String>,  
}  
  
#[derive(Deserialize)]  
pub struct PackContent {  
    pub spells: Option<Vec<Spell>>,  
    pub feats: Option<Vec<Feat>>,  
    pub items: Option<Vec<Item>>,  
}
```
---

### 5. YAML Loader
```Rust
use std::fs;  
  
pub fn load_yaml_pack(path: &str) -> Result<DndPack, Box<dyn std::error::Error>> {  
  
    let data = fs::read_to_string(path)?;  
  
    let pack: DndPack = serde_yaml::from_str(&data)?;  
  
    Ok(pack)  
}
```
Very simple.

---

### 6. ZIP Loader

Use the `zip` crate.
```Rust
use std::fs::File;  
use zip::ZipArchive;  
use std::io::Read;  
  
pub fn load_zip_pack(path: &str) -> Result<DndPack, Box<dyn std::error::Error>> {  
  
    let file = File::open(path)?;  
  
    let mut archive = ZipArchive::new(file)?;  
  
    let mut rules_file = archive.by_name("rules.yaml")?;  
  
    let mut yaml_data = String::new();  
  
    rules_file.read_to_string(&mut yaml_data)?;  
  
    let pack: DndPack = serde_yaml::from_str(&yaml_data)?;  
  
    Ok(pack)  
}
```
---

### 7. Unified Loader

Detect file type automatically.
```Rust
pub fn load_pack(path: &str) -> Result<DndPack, Box<dyn std::error::Error>> {  
  
    if path.ends_with(".yaml") || path.ends_with(".yml") {  
        return load_yaml_pack(path);  
    }  
  
    if path.ends_with(".dndpack") {  
        return load_zip_pack(path);  
    }  
  
    Err("Unsupported pack format".into())  
}
```
---

### 8. Registry Integration

After loading:
```Rust
pub fn register_pack(pack: DndPack, registry: &mut RuleRegistry) {  
  
    if let Some(spells) = pack.content.spells {  
        for spell in spells {  
            registry.spells.insert(spell.id.clone(), spell);  
        }  
    }  
  
    if let Some(feats) = pack.content.feats {  
        for feat in feats {  
            registry.feats.insert(feat.id.clone(), feat);  
        }  
    }  
}
```
---

### 9. Engine Startup

Startup loads:
```
core rules  
+ installed packs
```
Example:
```Rust
for path in packs_directory {  
  
    let pack = load_pack(path)?;  
  
    register_pack(pack, &mut registry);  
}
```
---

### 10. Recommended Final System

Best long-term design:
```
.dndpack (ZIP archive)  
  
required:  
  rules.yaml  
  
optional:  
  text.yaml  
  art/  
  localization/
```
Development mode also supports:
```
*.yaml
```
---

### Final Recommendation

For **this engine architecture**:

|Format|Best For|
|---|---|
|YAML|development + modding|
|ZIP `.dndpack`|distribution + GUI|

So the **best practical design is supporting both**.

---

If you'd like, the next extremely useful step would be showing the **full RuleExpression system** (the mini expression language used inside YAML rules). That piece is what allows rules like:
```YAML
value:  
  expr: add  
  args:  
    - stat: dex_mod  
    - 13
```
to remain **fully data-driven without scripting**.