# Explain IDs being hashed/interned/indexed insofar as you haven't already, and also explain what interning is.

IDs in this system often end up **hashed, interned, or indexed** so that the engine can work efficiently with large rule datasets (spells, items, creatures, etc.). These are three related but different optimization strategies.

---

### 1. Hashing

Hashing is the most basic mechanism.

When you use something like:
```Rust
HashMap<SpellId, Spell>
```
the `SpellId` must implement `Hash`. The string inside the ID is converted into a **fixed-size hash value**, which the hash map uses to quickly locate the entry.

Example conceptually:
```
"fireball" → hash → 0x8F13A27C
```
That hash determines where the data lives in the map.

##### Why it helps

Without hashing, a lookup would require scanning every entry. With hashing, lookup is **O(1)** on average.

##### Tradeoff

Hashing still requires hashing the string each time you use it.

---

### 2. Interning

**Interning** means storing only **one copy of each unique string** and replacing all other occurrences with a small reference (usually an integer).

Example without interning:
```
"fireball"  
"fireball"  
"fireball"
```
Three separate strings exist in memory.

With interning:
```
string pool  
-----------  
0 → "fireball"  
  
IDs  
----  
SpellId(0)  
SpellId(0)  
SpellId(0)
```
All references point to the same stored string.

##### Benefits

1. **Memory reduction**
2. **Fast equality comparison**
3. **Faster hashing**

String comparison becomes:
```
0 == 0
```
instead of comparing character-by-character.

##### Typical representation
```Rust
pub struct SpellId(u32);
```
The number is an index into the **string pool**.

---

### 3. Indexing

Indexing is the next step beyond interning.

Instead of mapping strings to IDs only, the loader also builds **direct indexes to data**.

Example:
```
SpellId(42) → Spell
```
Implementation:
```Rrust
Vec<Spell>
```
where the index corresponds to the ID.

Example:
```
SpellId(0) → Magic Missile  
SpellId(1) → Fireball  
SpellId(2) → Shield
```
Then lookup becomes:
```Rust
spells[id.0]
```
which is extremely fast.

##### Benefits

- constant-time lookup
- no hashing required
- cache-friendly

This is common in **game engines and compilers**.

---

### 4. How These Work Together

A typical pipeline is:
```
YAML loader  
      ↓  
string IDs  
      ↓  
intern strings  
      ↓  
assign numeric IDs  
      ↓  
build indexed arrays
```
Example transformation:
```
"fireball"  
     ↓  
intern pool index  
     ↓  
SpellId(42)  
     ↓  
spells[42]
```
---

### 5. Where This Appears in the Architecture

These are used for:
```
SpellId  
FeatId  
CreatureId  
ItemId  
RuleId
```
and indexes like:
```Rust
HashMap<StatId, Vec<RuleId>>  
HashMap<ActionType, Vec<RuleId>>
```
---

### 6. Why This Matters for the Rules Engine

Rule engines frequently perform lookups like:
```Rust
rules affecting StatId::ArmorClass  
rules affecting ActionType::Attack  
rules affecting SpellId::Fireball
```
If IDs are interned/indexed, these lookups become extremely cheap.

---

### 7. Example of an Intern Pool

Conceptually:
```Rust
struct StringInterner {  
    map: HashMap<String, u32>,  
    strings: Vec<String>,  
}
```
When a string appears:
```Rust
"fireball"
```
the interner:

1. checks if it exists
2. returns its numeric ID

So every occurrence of `"fireball"` becomes:
```Rust
SpellId(42)
```
---

### 8. Advantages

Interning and indexing provide:

- lower memory usage
- faster comparisons
- faster lookups
- faster rule evaluation

These improvements matter when you have:

- hundreds of spells
- hundreds of items
- thousands of rules

---

### 9. Disadvantages

They introduce some complexity:

- need a loader step
- IDs are less human-readable
- debugging sometimes requires mapping IDs back to strings

However, these costs are usually small.

---

### 10. Practical Recommendation for This System

Start simple:
```Rust
Id<T>(String)
```
Later, if needed, switch to:
```Rust
Id<T>(u32)
```
with:

- string interning
- indexed data tables

Because the ID wrapper hides the representation, **the rest of the engine does not need to change**.

---

✅ **Summary**

|Technique|Purpose|
|---|---|
|Hashing|fast lookup in hash maps|
|Interning|store one copy of strings and compare by integer|
|Indexing|map IDs directly to array positions|

Together they allow a rules engine to work with **human-readable data files while executing with game-engine-level efficiency**.

# Show how the loader will work to handle this

Below is a **clean loader pipeline** showing how YAML rules/content are converted into **interned IDs and indexed data tables**. The goal is:

- keep **human-readable YAML**
- convert to **fast numeric IDs**
- build **engine-ready indexes**

This keeps runtime evaluation extremely fast while keeping content easy to author.

---

### 1. Loader Pipeline Overview

The loader runs in four steps.
```
Load YAML  
   ↓  
Deserialize into raw structs (string IDs)  
   ↓  
Intern all string IDs  
   ↓  
Build indexed data tables
```
Example transformation:
```Rust
"fireball"  
   ↓  
SpellId("fireball")  
   ↓  
SpellId(42)  
   ↓  
spells[42]
```
---

### 2. Raw Data Structures (Before Interning)

When YAML is first loaded, IDs are still **strings**.

Example spell YAML:
```YAML
id: fireball  
level: 3  
school: evocation
```
Deserialized as:
```Rust
pub struct RawSpell {  
    pub id: String,  
    pub level: u8,  
    pub school: String,  
}
```
These raw structs exist **only during loading**.

---

### 3. The String Interner

The interner maps **strings → numeric IDs**.
```Rust
use std::collections::HashMap;  
  
pub struct StringInterner {  
    map: HashMap<String, u32>,  
    strings: Vec<String>,  
}
```
---

##### Intern Function
```Rust
impl StringInterner {  
  
    pub fn intern(&mut self, s: &str) -> u32 {  
        if let Some(&id) = self.map.get(s) {  
            return id;  
        }  
  
        let id = self.strings.len() as u32;  
        self.strings.push(s.to_string());  
        self.map.insert(s.to_string(), id);  
  
        id  
    }  
  
}
```
Example:
```Rust
intern("fireball") → 42  
intern("magic_missile") → 43
```
---

### 4. Engine ID Types

Interned IDs become lightweight numeric wrappers.
```Rust
pub struct SpellId(pub u32);  
pub struct FeatId(pub u32);  
pub struct ItemId(pub u32);  
pub struct CreatureId(pub u32);  
pub struct RuleId(pub u32);
```
These correspond to **indexes in arrays**.

---

### 5. Converting Raw Data to Engine Data

During loading we convert raw structs into engine structs.

Example spell conversion:
```Rust
pub struct Spell {  
    pub id: SpellId,  
    pub level: u8,  
}
```
Conversion step:
```Rust
fn convert_spell(  
    raw: RawSpell,  
    interner: &mut StringInterner  
) -> Spell {  
  
    let id = SpellId(interner.intern(&raw.id));  
  
    Spell {  
        id,  
        level: raw.level,  
    }  
}
```
Now `"fireball"` becomes:
```Rust
SpellId(42)
```
---

### 6. Building Indexed Tables

Instead of `HashMap`, we store data in **indexed vectors**.
```Rust
pub struct GameData {  
    pub spells: Vec<Spell>,  
    pub feats: Vec<Feat>,  
    pub items: Vec<Item>,  
    pub creatures: Vec<Creature>,  
}
```
The index matches the ID.

Example:
```Rust
spells[0] → magic_missile  
spells[1] → shield  
spells[2] → fireball
```
Lookup becomes:
```Rust
fn get_spell(data: &GameData, id: SpellId) -> &Spell {  
    &data.spells[id.0 as usize]  
}
```
No hashing required.

---

### 7. Building Rule Indexes

After rules are loaded, the loader builds **rule lookup tables**.

Example structure:
```Rust
pub struct RuleIndex {  
    pub stat_rules: HashMap<StatId, Vec<RuleId>>,  
    pub action_rules: HashMap<ActionType, Vec<RuleId>>,  
    pub phase_rules: HashMap<RulePhase, Vec<RuleId>>,  
}
```
Example rule:
```
modifier: +2 attack  
target: ranged attacks
```
Loader adds:
```Rust
ActionType::Attack → rule_id
```
This avoids scanning all rules.

---

### 8. Full Loader Example

Simplified pipeline:
```Rust
pub fn load_game_data() -> GameData {  
  
    let mut interner = StringInterner::new();  
  
    let raw_spells = load_yaml::<RawSpell>("spells.yaml");  
  
    let spells = raw_spells  
        .into_iter()  
        .map(|s| convert_spell(s, &mut interner))  
        .collect();  
  
    GameData {  
        spells,  
        feats: Vec::new(),  
        items: Vec::new(),  
        creatures: Vec::new(),  
    }  
}
```
---

### 9. Final Runtime Data

After loading, the engine works only with:
```Rust
SpellId(u32)  
FeatId(u32)  
ItemId(u32)  
RuleId(u32)
```
and indexed tables:
```Rust
Vec<Spell>  
Vec<Feat>  
Vec<Item>  
Vec<Rule>
```
Runtime lookups become extremely fast:
```Rust
spell = spells[id]  
rules = stat_rule_index[StatId::ArmorClass]
```
---

### 10. Why This Design Works Well

It combines:

|Authoring|Runtime|
|---|---|
|human-readable YAML|numeric IDs|
|flexible rules data|indexed tables|
|string identifiers|fast comparisons|

So content creators write:
```
spell: fireball
```
but the engine executes:
```
SpellId(42)
```
with **constant-time lookups**.

---

✅ **Result:**  
You get **data-driven rules with game-engine performance**, while keeping the rule content readable and easy to maintain.

---

If you'd like, I can also show the **one small architectural tweak that eliminates most of the ID conversion boilerplate inside the loader**, which large Rust data engines typically use.

# Given all of this and all of the complex, branching, and optional choices we need to store for the character sheet, show how to design the database schemas to effectively store full character sheet data.

Below is a **practical relational schema** for storing full character sheets given everything we designed:

- many **optional choices**
- **branching selections** (feats → spell choices → etc.)
- **inventory trees**
- **resources**
- **conditions**
- **edition support**
- **interned/indexed IDs**

The design goal is:

- flexible enough for **all 5e character choices**
- simple enough for **SQLite**
- compatible with the **rule engine architecture**

The key idea is:

> The database stores **character state and selections**, not derived stats.

Derived values are recomputed and cached by the rule engine.

---

### 1. Core Design Principles

##### Store only:

- identity
- level/class
- player choices
- resources
- inventory
- conditions

##### Do NOT store:

- derived stats
- attack bonuses
- AC
- spell save DC

Those come from the rule engine.

---

### 2. Core Character Table
```SQL
CREATE TABLE characters (  
    id INTEGER PRIMARY KEY,  
    name TEXT NOT NULL,  
    ruleset TEXT NOT NULL,  
    level INTEGER NOT NULL,  
    species_id INTEGER,  
    background_id INTEGER  
);
```
Fields store only **base identity**.

---

### 3. Ability Scores
```SQL
CREATE TABLE ability_scores (  
    character_id INTEGER,  
    ability TEXT,  
    score INTEGER,  
    PRIMARY KEY(character_id, ability)  
);
```
Example rows:
```
character 1 | STR | 16  
character 1 | DEX | 14
```
---

### 4. Class Levels (Multiclass Support)
```SQL
CREATE TABLE character_classes (  
    character_id INTEGER,  
    class_id INTEGER,  
    level INTEGER,  
    PRIMARY KEY(character_id, class_id)  
);
```
Example:
```
char1 | fighter | 5  
char1 | wizard  | 3
```
---

### 5. Generic Choice Storage

This is the **most important table**.

It stores **all player selections**.
```SQL
CREATE TABLE character_choices (  
    id INTEGER PRIMARY KEY,  
    character_id INTEGER,  
    choice_id INTEGER,  
    selection_type TEXT  
);
```
Example:
```
choice: fighting_style  
selection_type: fighting_style
```
---

### 6. Choice Values

Stores **what was selected**.
```SQL
CREATE TABLE choice_values (  
    choice_row_id INTEGER,  
    value_type TEXT,  
    value_id INTEGER  
);
```
Example rows:
```
choice_row 1 | feat | sharpshooter  
choice_row 2 | spell | fireball  
choice_row 3 | creature | wolf
```
This allows **any kind of choice**.

---

### 7. Spellbook / Known Spells

Some systems want quick access.
```SQL
CREATE TABLE character_spells (  
    character_id INTEGER,  
    spell_id INTEGER,  
    prepared BOOLEAN  
);
```
Supports:

- known spells
- prepared spells

---

### 8. Wild Shape Forms (2024)
```SQL
CREATE TABLE character_wildshape_forms (  
    character_id INTEGER,  
    creature_id INTEGER  
);
```
Example:
```
char1 | wolf  
char1 | brown_bear  
char1 | giant_spider
```
---

### 9. Resources

Tracks things like:

- rage
- ki
- spell slots
- wild shape uses
```SQL
CREATE TABLE character_resources (  
    character_id INTEGER,  
    resource_id INTEGER,  
    current_value INTEGER  
);
```
Example:
```
char1 | rage | 2  
char1 | ki   | 5
```
Maximum values come from rules.

---

### 10. Active Conditions
```SQL
CREATE TABLE character_conditions (  
    character_id INTEGER,  
    condition_id INTEGER,  
    source_rule INTEGER,  
    remaining_duration INTEGER  
);
```
Example:
```
char1 | poisoned | spell | 10
```
---

### 11. Inventory Items

Items are **instances**, not definitions.
```SQL
CREATE TABLE item_instances (  
    id INTEGER PRIMARY KEY,  
    character_id INTEGER,  
    item_id INTEGER,  
    quantity INTEGER  
);
```
Example:
```
instance 12 | char1 | potion_healing | 3
```
---

### 12. Container Tree

Inventory containers use **parent references**.
```SQL
CREATE TABLE item_locations (  
    item_instance_id INTEGER,  
    parent_container INTEGER  
);
```
Example:
```
torch → backpack  
backpack → character_root
```
This produces a **tree structure**.

---

### 13. Equipped Items
```SQL
CREATE TABLE equipped_items (  
    character_id INTEGER,  
    item_instance_id INTEGER  
);
```
Equipment slots aren't needed if rules handle them.

---

### 14. Action Uses (Optional)

For tracking per-day abilities.
```SQL
CREATE TABLE action_uses (  
    character_id INTEGER,  
    action_id INTEGER,  
    uses_remaining INTEGER  
);
```
---

### 15. Character Metadata

Optional.
```SQL
CREATE TABLE character_meta (  
    character_id INTEGER,  
    key TEXT,  
    value TEXT  
);
```
Used for:

- notes
- appearance
- alignment

---

### 16. Example Stored Character

The DB might contain:

##### classes
```
fighter 5  
wizard 3
```
##### choices
```
feat → sharpshooter  
fighting_style → archery  
wizard_spell → fireball  
wildshape_form → wolf
```
##### inventory
```
backpack  
 ├ rope  
 ├ rations  
 └ potion
```
##### resources
```
rage = 2  
spell slots tracked
```
---

### 17. Runtime Sheet Generation

The runtime process becomes:
```
load character rows  
       ↓  
construct Character struct  
       ↓  
run rule engine  
       ↓  
produce derived sheet
```
---

### 18. Why This Schema Works

It solves the hardest problems:

|Problem|Solution|
|---|---|
|complex choices|generic choice tables|
|branching selections|choice_values|
|multiclass|class table|
|containers|parent pointers|
|optional features|resource tables|
|rule engine compatibility|ID-based storage|

---

### 19. Why This Design Scales Well

This schema supports:

- CLI
- GUI
- future web app
- multiplayer if desired
- homebrew rulesets

without needing migrations for new mechanics.

---

### 20. Size of Stored Data

Typical character:
```
characters          1 row  
ability_scores      6 rows  
classes             1–3 rows  
choices             10–30 rows  
spells              10–40 rows  
items               10–50 rows  
resources           5–15 rows
```
Very small footprint.

---

✅ **Summary**

The schema works because it stores:

identity  
+ choices  
+ resources  
+ inventory  
+ conditions

while leaving **all rule logic and derived stats to the rule engine**.

---

If you'd like, I can also show **the one schema improvement used by large RPG tools (Pathbuilder, Foundry, etc.) that makes character choice trees dramatically easier to query and debug.**

# how do I maintain ids that are consistent across the database and rules data? For example there may be new homebrew modules (or a different number of them) which would lead to non-consistent numeric ID assignments by the rules data loader.

This is a **very important design issue**, and most systems solve it by **separating external IDs from internal numeric IDs**. The rule engine can still use fast numeric IDs internally, but **the database should store stable string IDs**.

The short version:

> **Use stable string IDs in storage, convert them to numeric IDs at runtime.**

This keeps character data valid even if the rule dataset changes.

---

### 1. The Core Problem

Your rule loader assigns numeric IDs dynamically:
```
fireball → SpellId(42)  
shield   → SpellId(43)
```
But if a new module loads before it:
```
magic_missile → SpellId(42)  
fireball      → SpellId(43)
```
Now every stored reference breaks.

Example in DB:
```
character_spells  
character_id | spell_id  
1            | 42
```
But `42` might now mean **a different spell**.

---

### 2. The Correct Solution: Stable String IDs

Store **stable string identifiers** in the database.

Example:
```
spell_id = "fireball"
```
These come directly from the rule data YAML:
```YAML
id: fireball
```
Now the character DB stores:
```
character_spells  
character_id | spell_id  
1            | fireball
```
This remains valid regardless of module load order.

---

### 3. Runtime Mapping

When loading a character:

1. Load rule data
2. Build interned numeric IDs
3. Convert stored strings → numeric IDs

Example loader step:
```Rust
fn resolve_spell_id(  
    id_str: &str,  
    index: &HashMap<String, SpellId>  
) -> SpellId {  
    index[id_str]  
}
```
So runtime becomes:
```Rust
"fireball" → SpellId(42)
```
---

### 4. Two-Level ID System

The architecture becomes:
```
External ID (stable)  
        ↓  
Internal ID (fast)

Example:

"fireball"  
      ↓  
SpellId(42)  
      ↓  
spells[42]
```
External IDs are **stable across modules and installs**.

Internal IDs are **runtime-only**.

---

### 5. Recommended ID Format

Use **namespaced IDs**.

Example:
```
phb.fireball  
phb.shield  
xgte.toll_the_dead  
homebrew.shadow_fireball
```
This avoids collisions between modules.

---

### 6. Rule Data Example
```
id: phb.fireball  
name: Fireball  
level: 3  
school: evocation
```
---

### 7. Database Example
```
character_spells  
  
character_id | spell_id  
1            | phb.fireball
```
---

### 8. Loader Index

When rules load, build indexes:
```Rust
HashMap<String, SpellId>  
HashMap<String, ItemId>  
HashMap<String, FeatId>  
HashMap<String, CreatureId>
```
Example:
```Rust
"phb.fireball" → SpellId(42)
```
---

### 9. Character Loading

Character loader resolves references:
```
DB value: "phb.fireball"  
          ↓  
lookup in rule index  
          ↓  
SpellId(42)
```
---

### 10. What Happens if a Module Is Missing?

Example:
```
spell_id = "xgte.toll_the_dead"
```
But the Xanathar module is not installed.

The loader can:
```
UnknownSpell("xgte.toll_the_dead")
```
Then the UI can show:
```
Missing content: Xanathar's Guide to Everything
```
---

### 11. Optional Optimization: Hash the String

Instead of storing strings, you could store a **stable hash**.

Example:
```RUST
hash("phb.fireball") → 0x93A0C12F
```
Pros:

- smaller storage
- faster comparison

Cons:

- debugging harder
- collisions theoretically possible

Most systems keep **plain strings**.

---

### 12. What the Database Should Store

Store **string IDs** for:
```
spells  
feats  
classes  
subclasses  
items  
creatures  
rules  
conditions
```
Example schema:
```SQL
character_spells  
  
character_id INTEGER  
spell_id TEXT
```
---

### 13. What the Rule Engine Should Store

Runtime data uses **numeric IDs**:
```Rust
SpellId(u32)  
ItemId(u32)  
FeatId(u32)
```
These are assigned by the loader.

---

### 14. Example End-to-End

##### YAML
```YAML
id: phb.fireball
```
##### DB
```
character_spells  
spell_id = "phb.fireball"
```
##### Loader
```Rust
"phb.fireball" → SpellId(42)
```
##### Engine
```Rust
spells[42]
```
---

### 15. Why This Design Works

This is the same architecture used by:

- game engines
- compilers
- large RPG systems
- modding systems

because it guarantees:
```
character data stability  
+  
runtime performance
```
---

### 16. Bonus Improvement (Highly Recommended)

Use **typed string IDs** in Rust.

Instead of:
```
String
```
use:
```Rust
struct SpellKey(String);  
struct ItemKey(String);  
struct FeatKey(String);
```
This prevents mixing IDs accidentally.

---

✅ **Final Architecture**

Database:
```
stable string IDs
```
Runtime engine:
```
interned numeric IDs
```
Loader:
```
string → numeric mapping
```
This gives you:

- **stable saves**
- **modular rule packs**
- **fast rule evaluation**

---

If you'd like, I can also show the **one additional loader structure used by mod-friendly game engines that prevents broken references when a module is removed or updated**, which becomes very useful once homebrew packs start circulating.

# Please do \[Show Content Registry Loader for Module Consistency].

The improvement used by **mod-friendly engines** (and many RPG tools) is to introduce a **Content Registry / Module Registry** that tracks **where every rule element came from** and allows references to remain valid even if modules change.

This prevents three common problems:

1. **Broken references when a module is missing**
2. **ID collisions between modules**
3. **Difficulty diagnosing where content came from**

The key idea is:

> Every rule element is identified by **(module_id, local_id)** rather than just a string.

---

### 1. Fully Qualified IDs

Instead of just:
```
fireball
```
IDs become **fully qualified**:
```
phb.fireball  
xgte.toll_the_dead  
homebrew.shadow_fireball
```
Struct representation:
```Rust
pub struct ContentKey {  
    pub module: ModuleId,  
    pub local_id: String,  
}
```
Example:
```Rust
ContentKey {  
  module: "phb",  
  local_id: "fireball"  
}
```
This guarantees uniqueness.

---

### 2. Module Registry

When rules load, the engine builds a **module registry**.
```Rust
pub struct ModuleRegistry {  
    pub modules: HashMap<String, ModuleInfo>,  
}
```
Module info:
```Rust
pub struct ModuleInfo {  
    pub id: String,  
    pub version: String,  
    pub source: String,  
}
```
Example entries:
```
phb  
xgte  
tcoe  
homebrew_dragon_pack
```
---

### 3. Content Registry

Next the loader builds a **content registry**.
```Rust
pub struct ContentRegistry {  
    pub spells: HashMap<ContentKey, SpellId>,  
    pub feats: HashMap<ContentKey, FeatId>,  
    pub items: HashMap<ContentKey, ItemId>,  
}
```
Example:
```Rust
phb.fireball → SpellId(42)  
phb.shield → SpellId(43)  
xgte.toll_the_dead → SpellId(88)
```
This mapping happens **after all modules load**.

---

### 4. Database Storage

Characters store the **fully qualified ID**.

Example:
```
character_spells  
  
character_id | spell_key  
1            | phb.fireball
```
Or split fields:
```
module | local_id  
phb    | fireball
```
Both work.

---

### 5. Character Loading

When loading a character:
```
DB key: phb.fireball  
        ↓  
lookup in ContentRegistry  
        ↓  
SpellId(42)
```
Now the rule engine uses numeric IDs.

---

### 6. Handling Missing Modules

If a module is missing:
```
phb.fireball → found  
xgte.toll_the_dead → missing
```
The loader creates a **placeholder reference**.

Example:
```Rust
enum ResolvedSpell {  
    Found(SpellId),  
    Missing(ContentKey),  
}
```
The UI can display:
```
Missing content: Xanathar's Guide to Everything  
Spell: Toll the Dead
```
The rest of the character still loads correctly.

---

### 7. Handling Updated Modules

Modules may change versions.

Example:
```
homebrew_dragon_pack v1  
homebrew_dragon_pack v2
```
The module registry tracks this:
```Rust
ModuleInfo {  
  id: "homebrew_dragon_pack",  
  version: "2.0"  
}
```
The system can warn:
```
Character built with module v1  
Current module v2
```
---

### 8. Preventing ID Collisions

Without namespacing:
```
fireball
```
Two modules might define it.

With namespacing:
```
phb.fireball  
homebrew.fireball
```
Both coexist safely.

---

### 9. Loader Algorithm

Simplified loader:
```
load modules  
      ↓  
register modules  
      ↓  
load rule data  
      ↓  
create ContentKey(module_id, local_id)  
      ↓  
assign numeric IDs  
      ↓  
build ContentRegistry
```
Example code:
```Rust
fn register_spell(  
    module: ModuleId,  
    raw: RawSpell,  
    registry: &mut ContentRegistry,  
    spells: &mut Vec<Spell>,  
) {  
  
    let id = SpellId(spells.len() as u32);  
  
    let key = ContentKey {  
        module,  
        local_id: raw.id,  
    };  
  
    registry.spells.insert(key, id);  
  
    spells.push(convert_spell(raw, id));  
}
```
---

### 10. Optional Improvement: Stable Hash IDs

Some systems also compute a **stable hash**.
```Rust
hash("phb.fireball")
```
Used for:

- network syncing
- save file validation

But the canonical ID remains the string.

---

### 11. Debugging Advantage

The registry allows easy debugging.

Example:
```
SpellId(42)  
      ↓  
ContentKey  
      ↓  
phb.fireball
```
So the UI can show:
```
Fireball (PHB)
```
---

### 12. How This Fits the Rule Engine

The runtime engine still uses:
```Rust
SpellId(u32)  
ItemId(u32)  
FeatId(u32)
```
But external references use:
```
ContentKey
```
This keeps runtime performance high.

---

### 13. Why Large Systems Do This

This architecture is used by systems that support:

- mods
- DLC
- homebrew content
- cross-version compatibility

because it guarantees:
```
stable saves  
+  
safe module loading  
+  
clear debugging
```
---

### 14. Final Architecture

**Rule data**
```
modules  
   ↓  
content keys  
   ↓  
numeric IDs
```
**Database**
```
stores content keys
```
**Engine**
```
uses numeric IDs
```
**Registry**
```
maps keys ↔ IDs
```
---

✅ Result:

Your system becomes **fully modular**, allowing:

- official rules
- optional books
- homebrew packs
- shareable characters

without ever breaking saved characters.

---

If you'd like, I can also show a **very useful additional feature used by large RPG engines: dependency graphs between modules (PHB required by subclasses, spells requiring books, etc.)**, which prevents users from loading incomplete rule sets.

# Please do \[show rule module dependency graphs].

A **module dependency system** ensures that rule packs (PHB, Xanathar’s, homebrew, etc.) load safely and that characters cannot reference content whose required modules are missing. It also prevents subtle runtime failures in the rule engine.

The key idea:

> Each module declares **dependencies and compatibility**, and the loader resolves them before loading rules.

This is essentially a **dependency graph** similar to package managers.

---

### 1. Module Metadata

Each rule module contains a small metadata file.

Example:
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
Meaning:
```
Xanathar requires PHB  
Xanathar optionally integrates with Tasha's
```
---

### 2. Rust Representation
```Rust
pub struct ModuleManifest {  
    pub id: ModuleId,  
    pub name: String,  
    pub version: String,  
  
    pub requires: Vec<ModuleId>,  
    pub optional: Vec<ModuleId>,  
}
```
---

### 3. Dependency Graph

The loader constructs a **directed graph**.

Example:
```
PHB  
 │  
 ├── XGTE  
 │  
 └── TCOE  
      │  
      └── Homebrew Pack
```
Graph representation:
```Rust
pub struct ModuleGraph {  
    pub modules: HashMap<ModuleId, ModuleManifest>,  
}
```
---

### 4. Dependency Resolution

Before loading rule data:

1. Read all manifests
2. Build dependency graph
3. Verify requirements
4. Topologically sort modules

Example:
```
load order:  
  
1 PHB  
2 XGTE  
3 TCOE  
4 homebrew_dragon_pack
```
Topological sort guarantees that **dependencies load first**.

---

### 5. Loader Algorithm

Simplified loader flow:
```
scan modules  
      ↓  
load manifests  
      ↓  
build dependency graph  
      ↓  
validate dependencies  
      ↓  
topological sort  
      ↓  
load rule data in order
```
Example Rust outline:
```Rust
fn resolve_modules(modules: Vec<ModuleManifest>) -> Vec<ModuleId> {  
  
    let graph = build_graph(modules);  
  
    topological_sort(graph)  
}
```
---

### 6. Missing Dependency Handling

If a module requires something missing:

Example:
```
homebrew_spells requires xgte
```
But XGTE isn't installed.

Loader error:
```
Module "homebrew_spells" requires "xgte"
```
User can:
```
install XGTE  
or disable the module
```
---

### 7. Optional Dependencies

Optional dependencies allow modules to **enhance behavior if another module exists**.

Example:
```
TCOE optional dependency: XGTE
```
If both exist:
```
additional feats enabled
```
If not:
```
module still loads
```
---

### 8. Character Validation

When loading a character:

1. collect all referenced content keys
2. check which modules they belong to
3. verify modules are loaded

Example:
```
character references:  
  
phb.fireball  
xgte.toll_the_dead
```
If XGTE missing:
```
missing module: xgte
```
UI message:
```
Character requires Xanathar's Guide to Everything
```
---

### 9. Module Conflicts

Modules may override content.

Example:
```
homebrew_fireball overrides phb.fireball
```
Manifest:
```YAML
overrides:  
  - phb.fireball
```
Loader policy options:
```
error  
replace  
warn
```
---

### 10. Content Source Tracking

The content registry also tracks **which module provided each rule**.

Example:
```
SpellId(42)  
  → phb.fireball  
  → module: phb
```
This helps:

- debugging
- UI display
- module disabling

---

### 11. Database Impact

The database already stores:
```
module_id.local_id
```
Example:
```
phb.fireball
```
So dependency checking is straightforward.

---

### 12. Why This Matters for Homebrew

Without dependency tracking:
```
homebrew subclass  
      ↓  
references tcoe spell  
      ↓  
tcoe not installed  
      ↓  
runtime crash
```
With dependency graph:
```
homebrew module requires tcoe  
      ↓  
loader blocks module
```
---

### 13. Compatibility System (Optional)

Modules can declare compatible rule versions.

Example:
```
ruleset: 5e2014
```
or
```
ruleset: 5e2024
```
This prevents mixing incompatible rule systems.

---

### 14. Final Loader Architecture

Complete loader pipeline now becomes:
```
scan modules  
      ↓  
load manifests  
      ↓  
build dependency graph  
      ↓  
resolve module order  
      ↓  
load rule YAML  
      ↓  
assign numeric IDs  
      ↓  
build content registry  
      ↓  
build rule indexes
```
---

### 15. Resulting System Capabilities

This architecture supports:
```
official rulebooks  
homebrew modules  
partial rule sets  
shared character files  
version compatibility
```
without breaking the engine.

---

✅ **Summary**

The module dependency system adds:
```
ModuleManifest  
ModuleRegistry  
ModuleGraph
```
so the loader can guarantee:
```
safe module loading  
stable content IDs  
valid character references
```
This is the same strategy used by **large moddable game engines and tabletop VTTs**.

---

If you'd like, I can also show **one final architectural refinement that dramatically simplifies the entire rules/content system: a unified "Content Table" registry used by large RPG engines**, which removes most of the repeated loader code for spells, feats, items, etc.

# Please do \[Show unified Content Table for loading rules].

The final refinement used in many moddable engines is a **Unified Content Registry (or Content Table system)**. Instead of having separate loading logic for _spells, feats, items, classes, creatures,_ etc., the engine treats all rule elements as entries in a **generic content table**. Each type still has its own Rust struct, but the loader and ID management become centralized.

The main idea:

> All game content is registered through one system that manages IDs, modules, and lookup tables.

This removes a large amount of repeated loader code.

---

### 1. The Problem Without a Unified Registry

Without it, the loader often looks like:
```Rust
load_spells()  
load_feats()  
load_items()  
load_classes()  
load_creatures()  
load_backgrounds()
```
Each system repeats:

- string ID parsing
- module resolution
- ID assignment
- indexing
- error handling

This leads to **a lot of duplicated logic**.

---

### 2. Unified ContentKey

Every piece of content shares a common identifier.
```Rust
pub struct ContentKey {  
    pub module: ModuleId,  
    pub id: String,  
}
```
Examples:
```
phb.fireball  
phb.fighter  
phb.longsword  
xgte.toll_the_dead
```
---

### 3. Content Type Enum

The engine also tracks **what kind of content it is**.
```Rust
pub enum ContentType {  
    Spell,  
    Feat,  
    Item,  
    Class,  
    Subclass,  
    Background,  
    Species,  
    Creature,  
    Rule,  
}
```
---

### 4. Unified ContentId

Internally, each entry receives a numeric ID.
```Rust
pub struct ContentId(u32);
```
The registry maps:
```
(ContentType, ContentKey) → ContentId
```
---

### 5. The Content Registry

The central registry:
```Rust
pub struct ContentRegistry {  
  
    pub key_to_id: HashMap<(ContentType, ContentKey), ContentId>,  
  
    pub spells: Vec<Spell>,  
    pub feats: Vec<Feat>,  
    pub items: Vec<Item>,  
    pub classes: Vec<Class>,  
    pub subclasses: Vec<Subclass>,  
    pub creatures: Vec<Creature>,  
  
}
```
The vectors store the actual data.

The map resolves string IDs to numeric IDs.

---

### 6. Registering Content

All loaders use the same function.

Example:
```Rust
fn register<T>(  
    table: &mut Vec<T>,  
    registry: &mut ContentRegistry,  
    content_type: ContentType,  
    key: ContentKey,  
    value: T,  
) -> ContentId {  
  
    let id = ContentId(table.len() as u32);  
  
    table.push(value);  
  
    registry.key_to_id.insert((content_type, key), id);  
  
    id  
}
```
Now every loader becomes trivial.

---

### 7. Example Spell Loader

Instead of a large custom system:
```Rust
fn load_spell(  
    raw: RawSpell,  
    module: ModuleId,  
    registry: &mut ContentRegistry,  
) {  
  
    let key = ContentKey {  
        module,  
        id: raw.id.clone(),  
    };  
  
    let spell = convert_spell(raw);  
  
    register(  
        &mut registry.spells,  
        registry,  
        ContentType::Spell,  
        key,  
        spell,  
    );  
}
```
The same function works for **all content types**.

---

### 8. Lookup During Character Loading

Character DB entry:
```
spell_id = "phb.fireball"
```
Lookup:
```Rust
(ContentType::Spell, "phb.fireball")  
        ↓  
ContentId(42)  
        ↓  
registry.spells[42]
```
---

### 9. Rule Engine IDs

The rule engine can still use **typed wrappers**.

Example:
```Rust
pub struct SpellId(pub ContentId);  
pub struct FeatId(pub ContentId);  
pub struct ItemId(pub ContentId);
```
These are just wrappers for safety.

---

### 10. Why This Helps the Loader

Without a registry, you repeat:
```
string ID parsing  
module validation  
ID assignment  
duplicate detection  
lookup building
```
With the registry:
```
all handled in one place
```
---

### 11. Module Tracking Integration

The registry also tracks module ownership.

Example:
```Rust
pub struct ContentEntryMeta {  
    pub module: ModuleId,  
    pub content_type: ContentType,  
}
```
Useful for:
```
debugging  
UI source display  
module unloading
```
---

### 12. Debug Example
```
ContentId(42)  
   ↓  
Spell  
   ↓  
ContentKey  
   ↓  
phb.fireball
```
UI display:
```
Fireball (PHB)
```
---

### 13. Memory Layout

At runtime the engine becomes extremely simple:
```
ContentRegistry  
   ├─ Vec<Spell>  
   ├─ Vec<Feat>  
   ├─ Vec<Item>  
   ├─ Vec<Class>  
   └─ Vec<Creature>
```
IDs are just **array indices**.

This is **very cache-friendly**.

---

### 14. Loader Architecture After This Change

Full pipeline:
```
scan modules  
      ↓  
load module manifests  
      ↓  
resolve dependencies  
      ↓  
load YAML files  
      ↓  
register content in registry  
      ↓  
assign numeric IDs  
      ↓  
build rule indexes
```
The registry becomes the **center of the content system**.

---

### 15. Why Large RPG Systems Use This

It simplifies three major things:

1. **modding**
2. **loader complexity**
3. **runtime lookups**

Without it, content systems tend to grow messy over time.

---

### 16. How This Fits the Rule Engine

The rule engine never cares about string IDs.

It receives:
```
SpellId  
FeatId  
ItemId
```
and accesses:
```Rust
registry.spells[id]
```
So performance remains excellent.

---

### 17. Final Architecture Summary

**Content files**
```
YAML
```
↓

**Module system**
```
ModuleRegistry  
ModuleGraph
```
↓

**Unified content system**
```
ContentRegistry  
ContentKey  
ContentType  
ContentId
```
↓

**Runtime engine**
```
Vec<T> tables  
numeric IDs  
rule indexes
```
↓

**Character database**
```
stores ContentKey strings
```
---

✅ Result: you get a system that is

- **modular**
- **safe for homebrew**
- **fast at runtime**
- **simple to extend**

while keeping the rule engine clean.

---

If you'd like, I can also show the **final polished directory structure for the entire project (engine, loader, database, CLI, GUI, rules, modules)** that cleanly organizes everything we've discussed.

# Please do \[Show final directory structure for the code].

Below is a **clean, production-quality project layout** that pulls together everything we discussed:

- rule engine
- content registry
- module system
- loader
- database layer
- CLI
- GUI
- rules data
- homebrew modules

The structure keeps **runtime engine code simple** while isolating the complex loading and content systems.

---

### Top-Level Project Structure
```
dnd-engine/  
│  
├─ Cargo.toml  
├─ README.md  
│  
├─ crates/  
│  
│  ├─ engine/  
│  ├─ rules/  
│  ├─ content/  
│  ├─ loader/  
│  ├─ database/  
│  ├─ character/  
│  ├─ cli/  
│  └─ gui/  
│  
├─ modules/  
│  
│  ├─ phb/  
│  ├─ xgte/  
│  ├─ tcoe/  
│  └─ homebrew/  
│  
└─ examples/
```
This uses a **Cargo workspace** with multiple crates.

Benefits:

- fast compile times
- strong module boundaries
- easy reuse for CLI + GUI + web

---

### 1. Engine Crate

Core rule evaluation engine.
```
crates/engine/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ rules/  
│  ├─ rule.rs  
│  ├─ rule_effect.rs  
│  ├─ rule_condition.rs  
│  ├─ rule_phase.rs  
│  
├─ evaluation/  
│  ├─ rule_graph.rs  
│  ├─ resolver.rs  
│  ├─ stacking.rs  
│  
├─ stats/  
│  ├─ stat_id.rs  
│  ├─ stat_value.rs  
│  
└─ context/  
   ├─ character_context.rs  
   ├─ rule_context.rs
```
Responsibilities:
```
rule evaluation  
dependency graphs  
rule phases  
stat computation  
stacking rules
```
No YAML, no DB, no modules.

---

### 2. Content Crate

Defines **all core content types and IDs**.
```
crates/content/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ ids/  
│  ├─ content_id.rs  
│  ├─ content_key.rs  
│  ├─ module_id.rs  
│  
├─ registry/  
│  ├─ content_registry.rs  
│  
├─ types/  
│  ├─ spell.rs  
│  ├─ feat.rs  
│  ├─ item.rs  
│  ├─ class.rs  
│  ├─ subclass.rs  
│  ├─ species.rs  
│  ├─ background.rs  
│  ├─ creature.rs  
│  
└─ rules/  
   ├─ rule_definition.rs
```
Responsibilities:
```
content definitions  
IDs  
content registry  
shared types
```
---

### 3. Rules Crate

Contains **rule data structures**.
```
crates/rules/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ rules/  
│  ├─ rule.rs  
│  ├─ rule_effect.rs  
│  ├─ rule_condition.rs  
│  
├─ effects/  
│  ├─ modifier.rs  
│  ├─ grant_feature.rs  
│  ├─ override.rs  
│  
├─ choices/  
│  ├─ choice_set.rs  
│  ├─ choice_value.rs
```
Responsibilities:
```
rule containers  
choice systems  
rule data structures
```
---

### 4. Loader Crate

Handles:
```
YAML  
modules  
dependency graph  
content registry
```
Structure:
```
crates/loader/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ modules/  
│  ├─ manifest.rs  
│  ├─ module_graph.rs  
│  ├─ module_registry.rs  
│  
├─ yaml/  
│  ├─ raw_spell.rs  
│  ├─ raw_feat.rs  
│  ├─ raw_item.rs  
│  
├─ converters/  
│  ├─ spell_loader.rs  
│  ├─ feat_loader.rs  
│  ├─ item_loader.rs  
│  
└─ pipeline/  
   ├─ load_modules.rs  
   ├─ resolve_dependencies.rs  
   ├─ build_registry.rs
```
Responsibilities:
```
load modules  
parse YAML  
build content registry  
assign numeric IDs  
build rule graphs
```
---

### 5. Character Crate

Represents a **runtime character**.
```
crates/character/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ character/  
│  ├─ character.rs  
│  ├─ class_levels.rs  
│  
├─ choices/  
│  ├─ character_choices.rs  
│  
├─ resources/  
│  ├─ resource_pool.rs  
│  
├─ inventory/  
│  ├─ inventory.rs  
│  ├─ item_instance.rs  
│  ├─ item_location.rs  
│  
└─ conditions/  
   ├─ active_condition.rs
```
Responsibilities:
```
character state  
inventory  
resources  
conditions  
choices
```
---

### 6. Database Crate

Handles persistence.
```
crates/database/  
│  
├─ src/  
│  
├─ lib.rs  
│  
├─ schema/  
│  ├─ characters.sql  
│  ├─ inventory.sql  
│  ├─ resources.sql  
│  
├─ models/  
│  ├─ character_row.rs  
│  ├─ inventory_row.rs  
│  
└─ queries/  
   ├─ load_character.rs  
   ├─ save_character.rs
```
Responsibilities:
```
SQLite schema  
query helpers  
character loading
```
Stores **ContentKey strings**, not numeric IDs.

---

### 7. CLI Crate

Command-line interface.
```
crates/cli/  
│  
├─ src/  
│  
├─ main.rs  
│  
├─ commands/  
│  ├─ create_character.rs  
│  ├─ view_character.rs  
│  ├─ list_spells.rs  
│  
└─ display/  
   ├─ character_sheet.rs
```
Responsibilities:
```
text UI  
debug tools  
scriptable usage
```
---

### 8. GUI Crate

Graphical interface.
```
crates/gui/  
│  
├─ src/  
│  
├─ main.rs  
│  
├─ app/  
│  ├─ state.rs  
│  
├─ views/  
│  ├─ character_sheet.rs  
│  ├─ inventory.rs  
│  ├─ spellbook.rs  
│  
└─ widgets/  
   ├─ choice_selector.rs
```
Responsibilities:
```
visual character builder  
inventory UI  
spell management
```
---

### 9. Module Content Directory

Rule content lives outside the code.
```
modules/  
│  
├─ phb/  
│  ├─ module.yaml  
│  
│  ├─ spells/  
│  │  ├─ fireball.yaml  
│  │  └─ shield.yaml  
│  
│  ├─ classes/  
│  │  └─ fighter.yaml  
│  
│  ├─ feats/  
│  │  └─ sharpshooter.yaml  
│  
│  └─ items/  
│     └─ longsword.yaml  
│  
├─ xgte/  
│  
└─ homebrew/
```
Each module contains:
```
manifest  
rules  
content definitions
```
---

### 10. Homebrew Packs

Users can share:
```
dragon_pack.dndpack
```
Containing:
```
module.yaml  
spells.yaml  
feats.yaml  
creatures.yaml
```
Loader extracts it into the module system.

---

### 11. Runtime Data Flow

Full runtime pipeline:
```
load modules  
      ↓  
resolve dependencies  
      ↓  
load YAML content  
      ↓  
build content registry  
      ↓  
load character from DB  
      ↓  
resolve content keys → IDs  
      ↓  
run rule engine  
      ↓  
produce character sheet
```
---

### 12. Why This Architecture Works

It keeps responsibilities clean:

| Layer     | Responsibility        |
| --------- | --------------------- |
| engine    | rule evaluation       |
| content   | definitions + IDs     |
| loader    | module + YAML loading |
| character | character state       |
| database  | persistence           |
| cli/gui   | interface             |

This avoids **God modules**.

---

### 13. Compile-Time Advantages

Using crates improves:
```
compile speed  
testability  
modularity
```
Example:
```
engine tests don't require database or GUI
```
---

### 14. Scaling to a Web App

You would simply add:
```
crates/server/
```
Structure:
```
server/  
 ├─ api/  
 ├─ auth/  
 └─ routes/
```
Everything else remains unchanged.

---

### 15. Final Architecture Summary
```
Rules data → Loader → Content Registry  
                          ↓  
Character DB → Character Struct  
                          ↓  
                    Rule Engine  
                          ↓  
                Derived Character Sheet  
                          ↓  
                   CLI / GUI / API
```
---

✅ This structure gives you a system that is:

- modular
- moddable
- safe for homebrew
- performant
- maintainable

while keeping the **rule engine itself small and clean**.

---

If you'd like, I can also show **one last optimization used by large tabletop engines that dramatically speeds up rule evaluation for complex characters (reducing rule evaluation from hundreds of rule checks to only a few dozen)**.

# Are there other special cases in the ruleset like the wildshapes that we haven't covered? If so what are they and how might our design need to change to accommodate them?

Yes — there **are several special-case mechanics in 5e** similar to Wild Shape that stress an otherwise clean rules engine. Most of them do **not require architecture changes**, but a few benefit from small extensions to the design we built.

The good news is that your architecture (rules + effects + character state + choices + content registry) already handles most of them. The remaining cases mainly fall into **four categories**:

1. **Stat block replacement**
2. **Limited selectable forms/options**
3. **Conditional feature replacement**
4. **Persistent but configurable entities**

Below are the important ones.

---

### 1. Stat Block Replacement Systems

(Wild Shape is one example)

These mechanics **temporarily replace large parts of the character sheet** with another stat block.

Examples:

- Druid **Wild Shape**
- **Polymorph**
- **True Polymorph**
- **Shapechange**
- **Magic Jar** (partial)
- **Wildshape 2024 with form limits**

##### Why they are tricky

They modify many stats simultaneously:
```
HP  
AC  
movement  
attacks  
skills  
senses  
size  
creature type
```
##### Our current solution

We already introduced:
```
StatOverride  
StatSource
```
Example rule effect:
```YAML
override stat: Strength  
source: creature.strength
```
##### Design extension

Add a **StatSource enum**:
```Rust
pub enum StatSource {  
    Character,  
    Creature(CreatureId),  
}
```
Character context stores:
```Rust
active_form: Option<CreatureId>
```
The rule engine checks the current stat source.

No major architecture change required.

---

### 2. Limited Known Forms / Options

(2024 Wild Shape)

These systems require **a stored list of selectable options**.

Examples:

- **Wild Shape forms (2024)**
- **Warlock invocations**
- **Artificer infusions**
- **Battle Master maneuvers**
- **Metamagic options**

##### Why they are tricky

Players must choose a **subset from a large list**, and the list may change when leveling.

Example:
```
Choose 2 invocations  
Choose 4 maneuvers  
Choose 3 metamagics
```
##### Our design already supports this

Using:
```
ChoiceSet  
character_choices table
```
Example:
```YAML
choice: warlock_invocation  
value: agonizing_blast
```
##### Optional improvement

Add **choice limits**.
```Rust
pub struct ChoiceSet {  
    pub id: ChoiceSetId,  
    pub options: Vec<ContentKey>,  
    pub max_selections: u8,  
}
```
No architectural change needed.

---

### 3. Partial Feature Replacement

(Some features replace earlier ones)

Examples:
```
Fighting Style replacements  
Cantrip replacement on level up  
Spell replacement for known casters  
Tasha's feature replacements  
2024 feature swaps
```
Example:
```
replace spell known  
replace fighting style  
replace class feature
```
##### Why it matters

These are **player edits to previous choices**, not new features.

##### Solution

Treat replacements as **choice mutations**.

Example DB state:
```YAML
choice: spell_known  
value: burning_hands  
  
choice: spell_replace  
old: burning_hands  
new: scorching_ray
```
Or simpler:
```
Remove old choice and insert new one.
```
No rule engine change required.

---

### 4. Persistent Configurable Entities

Some mechanics create **objects with state**.

Examples:
```
Artificer infusions  
Steel Defender  
Homunculus  
Beast Master companion  
Find Familiar  
Summoned creatures
```
These behave like **mini-characters**.

##### Why this matters

They may have:
```
HP  
AC  
actions  
conditions  
equipment
```
##### Simple design solution

Treat them as **creature instances attached to a character**.
```Rust
pub struct Companion {  
    pub creature_id: CreatureId,  
    pub owner: CharacterId,  
}
```
Database:
```
character_companions
```
The engine evaluates them like creatures.

No rule engine change needed.

---

### 5. Feature Scaling Tables

Some features scale by level in **nonlinear tables**.

Examples:
```
Sneak attack dice  
Rage damage  
Martial arts die  
Channel divinity uses  
Wild shape uses
```
##### Current solution

Handled by **rules conditional on level**.

Example rule:
```
if rogue_level >= 5  
sneak_attack = 3d6
```
But this can produce many rules.

##### Optional improvement

Add **LevelProgression tables**.
```Rust
pub struct LevelProgression<T> {  
    pub values: Vec<T>  
}
```
Example:
```
[1d6,1d6,2d6,2d6,3d6...]
```
Cleaner but optional.

---

### 6. Conditional Rule Activation

Some rules activate only under specific contexts.

Examples:
```
Sneak attack conditions  
Pack tactics  
Barbarian rage bonuses  
Flanking rules (optional)
```
Already handled by:
```
RuleCondition
```
Example:
```YAML
condition: target_adjacent_to_ally
```
No architecture change needed.

---

### 7. Temporary Rule Bundles

Some spells apply **large sets of temporary effects**.

Examples:
```
Bless  
Haste  
Slow  
Heroism  
Tenser's Transformation
```
Already handled by:
```
ActiveEffect
```
Example:
```Rust
EffectSource::Spell
```
No change required.

---

### 8. Inventory Transformations

Certain effects temporarily change equipment behavior.

Examples:
```
Hex Warrior weapon  
Shillelagh  
Magic weapon  
Pact weapon
```
These modify **item-derived stats**.

Our design already supports:
```Rust
RuleEffect::ModifyAttack  
RuleEffect::ModifyDamage
```
So no architecture change needed.

---

### 9. Spellcasting Variants

Spellcasting has multiple models:
```
prepared casters  
known casters  
spellbooks  
pact magic  
psionics (optional)
```
Our architecture already models this through:
```
SpellcastingSystem trait  
Spell slots resources  
character_spells table
```
No changes needed.

---

### 10. Shared Resource Pools

Some resources come from **multiple sources**.

Examples:
```
Channel Divinity  
Wild Shape  
Sorcery Points  
Superiority Dice
```
Already solved by:
```Rust
ResourcePool
```
Example:
```
resource_id = channel_divinity
```
Rules modify the maximum.

---

### 11. Things We Already Covered That Solve Most Edge Cases

Your system already handles:
```
inventory containers  
stacking rules  
rule overrides  
homebrew modules  
content registry  
choice trees  
wildshape form lists
```
So most special cases fall naturally into existing systems.

---

### Final Assessment

Only **three mechanics required explicit architectural support**:

| Mechanic                   | Solution           |
| -------------------------- | ------------------ |
| stat block replacement     | StatSource system  |
| limited selectable options | ChoiceSet          |
| persistent companions      | creature instances |

Everything else fits into:
```
rules  
effects  
conditions  
resources  
choices
```
---

### Final Conclusion

Your architecture already handles **~95% of 5e mechanics cleanly**.

The only special systems worth explicitly modeling are:
```
StatSource (forms)  
ChoiceSet (selectable features)  
Companion entities  
LevelProgression tables (optional)
```
Everything else can be expressed through the **data-driven rule system**.

---

If you'd like, I can also show **the 6 worst rule edge cases in 5e that frequently break naive character builders and how your architecture specifically handles each one**.