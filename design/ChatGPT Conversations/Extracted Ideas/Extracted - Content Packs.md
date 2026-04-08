# Links

### Part 1
1. [[Part 1 Raw#3. Content Packs| Content pack structure]]
2. [[Part 1 Raw#3. Content Pack Tables|Content pack SQL]]
3. [[Part 1 Raw#11. Versioning System|Versioning]]

### Part 4
1. [[Part 4 Raw#Please do [Show Capability Flags instead of Edition Checks|Capability Flags]] (long)

### Part 5
1. [[Part 5 Raw#Part 2 — Portable Homebrew Format|Pack File Formatting, Invidividual vs ZIP]] (long)

### Part 7
1. [[Part 7 Raw#Please do [Show Content Registry Loader for Module Consistency].|Content Registry Loader]] (long)
2. [[Part 7 Raw#Please do [show rule module dependency graphs].|Module Dependency Graph]] (long)
3. [[Part 7 Raw#Please do [Show unified Content Table for loading rules].|Unified Content Table]] (long)

# Extracted Data Structures and Representations

### Part 1

#### #1
```
content/
  dnd5e/
    races.json
    classes.json
    feats.json
  grimdark_homebrew/
    races.json
    classes.json
    magic.json
```

#### #2
```SQL
content_packs
-------------
id TEXT PRIMARY KEY
name TEXT
version TEXT
description TEXT
author TEXT
enabled BOOLEAN
```

#### #3
```SQL
content_versions
-------------
pack_id
version
installed_at
```

### Part 4

#### #1
```Rust
pub enum RuleCapability {  
    WeaponMastery,  
    OriginFeats,  
    EpicBoons,  
    PartialShortRestRecovery,  
    BackgroundAbilityScores,  
    NewExhaustionRules,  
}
  
pub struct Engine {  
    pub capabilities: HashSet<RuleCapability>,  
    pub rules: RulesDatabase,  
}

pub enum Edition {  
    Dnd5e2014,  
    Dnd5e2024,  
}

pub struct SpellDefinition {  
    pub id: SpellId,  
    pub rules: Vec<RuleEffect>,  
    pub required_capability: Option<RuleCapability>,  
}
```

### Part 7

#### #1
```Rust
pub struct ContentKey {  
    pub module: ModuleId,  
    pub local_id: String,  
}

// example
ContentKey {  
  module: "phb",  
  local_id: "fireball"  
}

pub struct ModuleRegistry {  
    pub modules: HashMap<String, ModuleInfo>,  
}

pub struct ModuleInfo {  
    pub id: String,  
    pub version: String,  
    pub source: String,  
}

pub struct ContentRegistry {  
    pub spells: HashMap<ContentKey, SpellId>,  
    pub feats: HashMap<ContentKey, FeatId>,  
    pub items: HashMap<ContentKey, ItemId>,  
}

// for missng modules
enum ResolvedSpell {  
    Found(SpellId),  
    Missing(ContentKey),  
}
```

#### #2
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

#### #3
```Rust
pub struct ContentKey {  
    pub module: ModuleId,  
    pub id: String,  
}

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

pub struct ContentId(u32);

pub struct ContentRegistry {  
    pub key_to_id: HashMap<(ContentType, ContentKey), ContentId>,  
    pub spells: Vec<Spell>,  
    pub feats: Vec<Feat>,  
    pub items: Vec<Item>,  
    pub classes: Vec<Class>,  
    pub subclasses: Vec<Subclass>,  
    pub creatures: Vec<Creature>,  
}

// rule engine IDs
pub struct SpellId(pub ContentId);  
pub struct FeatId(pub ContentId);  
pub struct ItemId(pub ContentId);

pub struct ContentEntryMeta {  
    pub module: ModuleId,  
    pub content_type: ContentType,  
}
```

# (Pseudo)Code

### Part 4

#### #1
```Rust
impl Engine {  
    pub fn new(edition: Edition, rules: RulesDatabase) -> Self {  
        let capabilities = match edition {  
            Edition::Dnd5e2014 => {  
                HashSet::from([])  
            }  
  
            Edition::Dnd5e2024 => {  
                HashSet::from([  
                    RuleCapability::WeaponMastery,  
                    RuleCapability::OriginFeats,  
                    RuleCapability::EpicBoons,  
                    RuleCapability::PartialShortRestRecovery,  
                    RuleCapability::BackgroundAbilityScores,  
                ])  
            }  
        };  
  
        Self { capabilities, rules }  
    }  
}
```

```Rust
if engine.capabilities.contains(&RuleCapability::WeaponMastery) {  
    apply_weapon_mastery(weapon);  
}
```

```Rust
fn load_spell(  
    spell: SpellDefinition,  
    engine: &Engine  
) -> Option<SpellDefinition> {  
  
    if let Some(cap) = spell.required_capability {  
        if !engine.capabilities.contains(&cap) {  
            return None;  
        }  
    }  
  
    Some(spell)  
}
```

```Rust
if feat.category == FeatCategory::Origin {  
    require_capability(RuleCapability::OriginFeats)?;  
}

if engine.capabilities.contains(  
    &RuleCapability::PartialShortRestRecovery  
) {  
    recover_partial_resources(character);  
} else {  
    recover_all_short_rest_resources(character);  
}
```

### Part 7

#### #1
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

#### #3
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