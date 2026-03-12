
# Links

### General Discussion

#### Part 2
* [[Part 2 Raw#***returning to the original text file I gave you, how would the design change for adding an inventory system. This must account for money, adventuring gear (including container items like backpacks), weapons, armor, and magic items which can have many features, effects, and bonuses.***|Original Question]]
* [[Part 2 Raw#***Returning to the inventory, for containers, how does representing `ItemLocation` create a tree structure? Wouldn't it make more sense to store containers as `Vec<Vec<ItemInstance>>` or for names/ids as `HashMap<String, Vec<ItemInstance>>`?***|ItemLocation to create a tree structure]]

#### Part 3
- [[Part 3 Raw#You previously mentioned the inventory architecture used by large RPG systems for stackable items, containers, equipment slots, and unique vs stackable objects. What elements do they have that we have not already discussed?|Inventory architecture from large RPGs]]
- [[Part 3 Raw#Put this together with the inventory systems we previously designed, keeping only the parts relevant to D&D (so for example no need for multiplayer elements, event hooks, item ownership, or equipment slots).|Combining it with previous design]]
- [[Part 3 Raw#Please do [Show an ECS implementation]|ECS implementation]]
- [[Part 3 Raw#This ECS system seems a little bit overkill in terms of how abstract it becomes. Are there important things I'm missing?|Redesigning without ECS?]]
- [[Part 3 Raw#Please stop suggesting ECS systems and similar levels of overkill abstraction. What is the container query and item traversal problem?|Container Query and Item Traversal Problem]]

### Data Structure/Architecture

#### Part 2
- See [[Part 2 Raw#***returning to the original text file I gave you, how would the design change for adding an inventory system. This must account for money, adventuring gear (including container items like backpacks), weapons, armor, and magic items which can have many features, effects, and bonuses.***|Original Question]] 
- [[Part 2 Raw#9. Best Rust-Idiomatic Inventory Model|ItemLocation -> Idiomatic Inventory Model]]

#### Part 3
- See [[Part 3 Raw#You previously mentioned the inventory architecture used by large RPG systems for stackable items, containers, equipment slots, and unique vs stackable objects. What elements do they have that we have not already discussed?|large RPG architecture]] 
- See [[Part 3 Raw#Put this together with the inventory systems we previously designed, keeping only the parts relevant to D&D (so for example no need for multiplayer elements, event hooks, item ownership, or equipment slots).|Combining it with previous design]] above
- See [[Part 3 Raw#Please do [Show an ECS implementation]|ECS implementation]] above (but don't use it) as well as the [[Part 3 Raw#This ECS system seems a little bit overkill in terms of how abstract it becomes. Are there important things I'm missing?|redesign without ECS]] above

#### Part 4

- Code architecture and structs: [[Part 4 Raw#3. Inventory System|Inventory System]] and [[Part 4 Raw#4. Item Definitions|Item Definitions]]

#### Part 5

- YAML data representation: [[Part 5 Raw#11. Items|Items]] and [[Part 5 Raw#12. Weapons|Weapons]] and [[Part 5 Raw#13. Armor|Armor]] and [[Part 5 Raw#14. Containers|Containers]]

#### Part 6

- YAML rule representation: [[Part 6 Raw#1. YAML Rules (Data Layer)|Chain Mail and Ring of Protection Example]]
- Part 6 architecture [[Part 6 Raw#15. Items|Item Data Structures]]

#### Part 7

- SQL tables: [[Part 7 Raw#11. Inventory Items|Items]] and [[Part 7 Raw#12. Container Tree|Containers]] and [[Part 7 Raw#13. Equipped Items|Equipped Items]]
- [[Part 7 Raw#2. Content Crate|Content Crate]] architecture

#### Unassigned

- [[Part 5 Raw#11. Interaction With Inventory|Item Suppression by shapechanging]]

# Common Data Structure Elements

### struct Inventory
- Store items as:
	1. `Vec<ItemInstance>`
	2. `HashMap<ItemId, ItemInstance>`
	3. `HashMap<ItemInstanceId, ItemInstance>` plus `HashMap<ItemStackId, ItemStack>`
	4. `HashMap<ItemInstanceId, ItemInstance>`
- Could store money as a separate struct
- Items option 3 treats an `ItemStack` as a separate but parallel data structure in the `Inventory` (i.e., stacks of items are not treated the same as non-stacking items)
- store lists for attuned and equipped items?

### enum ItemLocation
* records if an item is carried, in a container, or (sometimes) equipped
	1. `Carried, Equipped, InContainer(ItemId),`
	2. `Root, InContainer(ItemId)`
	3. `Carried, Container(ItemInstanceId)`
### struct ItemInstance
- `ItemId`
	- optional parent id (for containers)
- `ItemLocation`
- bools for equipped and attuned
- (optional) integer for quantity
- separate `ItemDefId` and `ItemInstanceId`
- an (optional) item state (charges, durability, spells contained, etc)
- optional container rules

### struct ItemDefinition
- `ItemId`
- name
- weight
- `Vec<ItemProperty>`
- `ItemType` (weapon, armor, gear, etc)
- `Vec<Tag>`
- bools for stackable and requires attunement
- `Vec<ItemEffect>` or `Vec<RuleEffect>` (modifiers)

### structs ContainerData, ArmorData, WeaponData, GearData, MagicItemData
- containers
	- weight capacity, ~~item capacity,~~ allowed item tags
	- maybe make capacities optional
	- perhaps add something like `contents_weight_override` for things like bags of holding
- armor
	- base ac
	- dex bonus
	- stealth disadvantage
- weapons
	- `Damage` (presumably struct of die size, die count, damage type)
	- `WeaponProperty` (eg finesse, heavy)
	- Add a `WeaponMastery` for 2024??
- magic items
	- requires attunement?
- gear
	- ???

#### struct Damage
- properly belongs to other parts of design
- just needs to store weapon damage info

#### struct/enum WeaponProperty
- enum for enumerating list of 5e base weapon properties
- struct if we want to be able to homebrew more (like `phb.heavy` and `my_stuff.arm_covering`)

### struct Tag
- wrapper for a string (possibly `'static`)

### struct ItemEffect
-  `Vec` of modifiers/rule effects

### enum ItemState
- variants: no state, charges, durability, spells contained

### struct ItemStack
- `StackId` and `ItemDefId`
- unsigned integer for quantity
- `ItemLocation`

### enum ItemType
- variants: weapon, armor, gear, container, magic item
- each variant can have it's own subtype data struct (e.g., `WeaponData`)

### struct Id (all of them)
- see \<TODO Identifiers extract> for more
- either dynamically generated integers (probably indexes into more fundamental arrays) or strings (like `phb.potion_of_healing` or `tcoe.eldritch_claw_tattoo`)

### struct Money
- unsigned integers for 5e currencies

# Extracted Data Structures and Representations

### Part 2

#### Original Question

```Rust
pub struct Inventory {  
    pub coins: Coins,  
    pub items: Vec<ItemInstance>,  
}

pub struct Coins {  
    pub cp: u32,  
    pub sp: u32,  
    pub ep: u32,  
    pub gp: u32,  
    pub pp: u32,  
}

pub struct ItemInstance {  
    pub id: ItemId,  
    pub quantity: u32,  
    pub equipped: bool,  
    pub attuned: bool,  
    pub location: ItemLocation,  
}

pub struct ItemDefinition {  
    pub id: ItemId,  
    pub name: String,  
    pub kind: ItemKind,  
    pub weight: f32,  
    pub properties: Vec<ItemProperty>,  
    pub rules: Vec<RuleEffect>,  
}

pub enum ItemKind {  
    Weapon(WeaponData),  
    Armor(ArmorData),  
    Gear(GearData),  
    Container(ContainerData),  
    MagicItem(MagicItemData),  
}

pub struct WeaponData {  
    pub damage: Damage,  
    pub properties: Vec<WeaponProperty>,  
}

pub struct ArmorData {  
    pub base_ac: u8,  
    pub dex_bonus: DexBonusRule,  
    pub stealth_disadvantage: bool,  
}

pub struct ContainerData {  
    pub capacity_weight: f32,  
}

pub struct MagicItemData {  
    pub requires_attunement: bool,  
}

pub enum ItemLocation {  
    Carried,  
    Equipped,  
    InContainer(ItemId),  
}
```

#### ItemLocation

```Rust
struct ItemInstance {  
    id: ItemId,  
    parent: Option<ItemId>,  
}

pub struct Inventory {  
    items: HashMap<ItemId, ItemInstance>,  
    children: HashMap<ItemId, Vec<ItemId>>,  
}

// Better recommended

pub struct Inventory {  
    items: HashMap<ItemId, ItemInstance>,  
}  

pub struct ItemInstance {  
    pub id: ItemId,  
    pub quantity: u32,  
    pub location: ItemLocation,  
}

pub enum ItemLocation {  
    Carried,  
    Equipped,  
    InContainer(ItemId),  
}
```

### Part 3

#### Large RPG Architecture

```Rust
struct ItemInstanceId(Uuid);

struct ItemStack {  
    id: StackId,  
    item_def: ItemDefId,  
    quantity: u32,  
}

struct ItemInstance {  
    id: ItemInstanceId,  
    def: ItemDefId,  
  
    state: ItemState,  
}
// Example
enum ItemState {  
    None,  
  
    Charges {  
        current: u32,  
        max: u32,  
    },  
  
    Durability {  
        current: u32,  
        max: u32,  
    },  
  
    SpellContainer {  
        spells: Vec<SpellId>,  
    }  
}

struct ItemDefinition {  
    id: ItemDefId,  
    name: String,  
    tags: Vec<Tag>,  
}

struct ContainerRules {  
    max_items: Option<u32>,  
    max_weight: Option<f32>,  
    allowed_tags: Option<Vec<Tag>>,  
}

struct Attunement {  
    required: bool,  
}

// Character tracks
struct AttunementState {  
    attuned_items: Vec<ItemInstanceId>,  
}

struct ItemLocation {  
    owner: OwnerId,  
    container: Option<ItemInstanceId>,  
}

// items emitting rule modifiers
struct ItemEffect {  
    modifiers: Vec<Modifier>,  
}
```

#### Combining it With Previous Stuff

```Rust
struct Inventory {  
    items: HashMap<ItemInstanceId, ItemInstance>,  
    stacks: HashMap<ItemStackId, ItemStack>,  
}

struct ItemDefinition {  
    id: ItemDefId,  
    name: String,  
  
    tags: Vec<Tag>,  
  
    weight: f32,  
  
    stackable: bool,  
  
    container: Option<ContainerRules>,  
  
    effects: Vec<ItemEffect>,  
  
    attunement_required: bool,  
}

struct Tag(&'static str);

struct ItemInstance {  
    id: ItemInstanceId,  
  
    def: ItemDefId,  
  
    location: ItemLocation,  
  
    state: ItemState,  
  
    attuned: bool,  
}

enum ItemState {  
    None,  
  
    Charges {  
        current: u32,  
        max: u32,  
    },  
  
    Durability {  
        current: u32,  
        max: u32,  
    },  
}

struct ItemStack {  
    id: ItemStackId,  
  
    def: ItemDefId,  
  
    quantity: u32,  
  
    location: ItemLocation,  
}

enum ItemLocation {  
    Root,  
    InContainer(ItemInstanceId),  
}

struct ContainerRules {  
    max_weight: Option<f32>,  
    max_items: Option<u32>,  
}

// Currency = stackable items

// Character tracks
struct Attunement {  
    items: Vec<ItemInstanceId>,  
}

struct ItemEffect {  
    modifier: Modifier,  
}
```

#### ECS Implementation

Excluded because I don't intend to use it

#### Post-ECS Implementation

```Rust
struct Inventory {  
    items: HashMap<ItemId, ItemInstance>  
}

struct ItemInstance {  
    id: ItemId,  
    def: ItemDefId,  
  
    location: ItemLocation,  
  
    stack: Option<u32>,  
    container: Option<ContainerRules>,  
    state: Option<ItemState>,  
  
    attuned: bool,  
}

enum ItemLocation {  
    Root,  
    InContainer(ItemId),  
}

struct ItemDefinition {  
    id: ItemDefId,  
    name: String,  
  
    tags: Vec<Tag>,  
  
    weight: f32,  
  
    stackable: bool,  
  
    effects: Vec<ItemEffect>,  
  
    container: Option<ContainerRules>,  
  
    attunement_required: bool,  
}

struct ContainerRules {  
    max_weight: Option<f32>,  
    max_items: Option<u32>,  
}

enum ItemState {  
    Charges { current: u32, max: u32 },  
}
```

### Part 4

#### Inventory System

```Rust
pub struct Inventory {  
    pub items: HashMap<ItemInstanceId, ItemInstance>,  
}

pub struct ItemInstance {  
    pub id: ItemInstanceId,  
    pub definition: ItemId,  
  
    pub quantity: u32,  
  
    pub location: ItemLocation,  
}

pub enum ItemLocation {  
    Carried,  
    Container(ItemInstanceId),  
}

pub struct Money {  
    pub cp: u32,  
    pub sp: u32,  
    pub gp: u32,  
    pub pp: u32,  
}
```

#### Item Definitions

```Rust
pub struct ItemDefinition {  
    pub id: ItemId,  
    pub name: String,  
  
    pub item_type: ItemType,  
  
    pub weight: f32,  
  
    pub rules: Vec<RuleEffect>,  
}

pub enum ItemType {  
    Weapon(WeaponData),  
    Armor(ArmorData),  
    Gear,  
    Container(ContainerData),  
    MagicItem,  
}

pub struct ContainerData {  
    pub capacity_weight: f32,  
}
```

### Part 5: YAML Examples

#### Items

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

#### Weapons

```YAML
id: longsword  
type: weapon  
  
damage: 1d8  
damage_type: slashing  
  
properties:  
  - versatile  
  
rules: []
```

#### Armor

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

#### Containers

```YAML
id: backpack  
type: container  
  
capacity:  
  weight: 30
```

### Part 6: YAML Examples

#### Armor

```YAML
id: chain_mail  
type: armor  
  
rules:  
  - phase: base  
    target: armor_class  
    operation: set  
    value: 16
```

#### Magic Item

```YAML
id: ring_of_protection  
  
rules:  
  - phase: modifier  
    target: armor_class  
    operation: add  
    value: 1  
    category: item
```

### Part 7: SQL Tables

#### Items

```SQL
CREATE TABLE item_instances (  
    id INTEGER PRIMARY KEY,  
    character_id INTEGER,  
    item_id INTEGER,  
    quantity INTEGER  
);
```

#### Container Tree

```SQL
CREATE TABLE item_locations (  
    item_instance_id INTEGER,  
    parent_container INTEGER  
);
```

#### Equipped Items

```SQL
CREATE TABLE equipped_items (  
    character_id INTEGER,  
    item_instance_id INTEGER  
);
```

# (Pseudo)Code

### Part 2

#### ItemLocation

```Rust
fn children_of(container: ItemId, inv: &Inventory) -> Vec<&ItemInstance> {  
    inv.items  
        .iter()  
        .filter(|i| matches!(i.location, ItemLocation::InContainer(id) if id == container))  
        .collect()  
}
```

```Rust
fn container_weight(id: ItemId, inv: &Inventory, db: &ItemDb) -> f32 {  
    let mut total = 0.0;  
  
    for item in inv.items.iter().filter(|i| i.location == ItemLocation::InContainer(id)) {  
        let def = db.get(item.id);  
        total += def.weight * item.quantity as f32;  
  
        if def.is_container() {  
            total += container_weight(item.id, inv, db);  
        }  
    }  
  
    total  
}
```

```Rust
impl Inventory {  
    pub fn rule_sources(&self, items: &ItemDatabase) -> Vec<RuleSource> {  
        self.items  
            .iter()  
            .filter(|item| item.equipped || item.attuned)  
            .map(|instance| items.get(instance.id))  
            .flat_map(|def| def.rules.clone())  
            .collect()  
    }  
}
```

### Part 3

#### Large RPG Architecture

```Rust
fn split_stack(stack: &mut ItemStack, amount: u32) -> ItemStack
```

#### Post-ECS Implementation

```Rust
fn move_item(  
    inv: &mut Inventory,  
    item: ItemId,  
    location: ItemLocation,  
)

fn split_stack(  
    inv: &mut Inventory,  
    item: ItemId,  
    amount: u32  
)

fn merge_stacks(  
    inv: &mut Inventory,  
    a: ItemId,  
    b: ItemId  
)

// for traversing containers
fn children_of(inv: &Inventory, container: ItemId) -> Vec<ItemId> {  
    inv.items  
        .iter()  
        .filter(|(_, item)| item.location == ItemLocation::InContainer(container))  
        .map(|(id, _)| *id)  
        .collect()  
}

fn ancestors_of(inv: &Inventory, item: ItemId) -> Vec<ItemId> {  
    let mut result = Vec::new();  
    let mut current = item;  
  
    while let Some(parent) = inv.parent_of(current) {  
        result.push(parent);  
        current = parent;  
    }  
  
    result  
}

fn descendants_of(inv: &Inventory, container: ItemId) -> Vec<ItemId> {  
    let mut result = Vec::new();  
    let mut stack = vec![container];  
  
    while let Some(current) = stack.pop() {  
        for child in children_of(inv, current) {  
            result.push(child);  
            stack.push(child);  
        }  
    }  
  
    result  
}

// Example
fn inside_bag_of_holding(inv: &Inventory, item: ItemId) -> bool {  
    for ancestor in ancestors_of(inv, item) {  
        if inv.definition(ancestor).name == "Bag of Holding" {  
            return true;  
        }  
    }  
    false  
}

// Example
fn coin_total(inv: &Inventory) -> u32 {  
    inv.items  
        .values()  
        .filter(|item| item.is_coin())  
        .map(|item| item.stack.unwrap_or(1))  
        .sum()  
}
```