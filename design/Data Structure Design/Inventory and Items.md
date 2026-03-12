
Last updated 2026-03-11

# General Ideas

### Inventory Tree Structure

Rather than creating the tree structure of an inventory using nested data structures, have the inventory be a flat 1D `Vec` (may also be a `HashMap` mapping item IDs to their descriptions or instances) and instead use an `ItemLocation` field to indicate where in the hierarchy it is. Using a 2D `Vec` or similar makes operations more difficult, loses container information, and may require further recursive nesting/logic to determine nesting. 

Finding items within a container is an iteration over the 1D `Vec` and filtering based on matching `ItemLocation` fields. Alternatively, store the items themselves in the 1D `Vec` or map and add a `HashMap<ItemId, Vec<ItemId or ItemInstance>>` that maps container items to lists of their contained items to avoid iterating over the full list to find child/parent items.

### Money

There are two options: represent money as an independent struct containing fields for each currency (cp, sp, ep, gp, pp) or treat them as just stacked items. The latter will probably require special treatment in the UI layer and would probably add an `is_currency` flag to the items themselves and require special logical treatment like `get_currency(& Inventory)` or `get_currency(&Inventory, ContainerId)` . The advantage of the first implementation is obvious simplicity. The latter implementation trades off complexity for the possibility of homebrewing currencies.

### Item Representations

Items require division between the rules definition `ItemDefinition` and the inventory representation `ItemInstance`. The rules definition contains the immutable book information like name, description, rule effects, and any subtype information (armor AC values, requiring attunement, stackability(?), etc). The inventory representation needs to contain the active, specific information like quantity, location, if it's equipped, etc).

For statuses like attuned and equipped, that could also be stored withing the `Inventory` or `Character` structs as separate fields instead of keeping that within the individual`ItemInstance` representation. Either way there will have to be some logic enforcing stack vs attuned/equipped state. Since the item list is likely to be quite small (we will use some kind of stacking mechanism), iterating over the list to find attuned/equipped items will not be terribly expensive.

### Item Stacking

Some items should obviously stack, like arrows or potions. Some items obviously can't stack, like magic items that require attunement. Some items lie in a gray zone; you could make it so that regular weapons stack (so 2x dagger instead of dagger, dagger) or not. For items that do stack, any customizations (e.g. display name, +1 to attack) could force them to unstack or could apply to the whole stack. Forcing them to unstack is probably a UI anti-pattern, in which case gray area items should not stack; adding a display name or custom tag to a stack of potions makes sense but adding +1 to attack to a stack of longswords does not.

For the data structure, one option is to make a unique `ItemStack` struct. This would force separate treatment of unstacked and stacked items in the code because they are different types, which I think is a bad idea. A simpler version is to give the `ItemInstance` struct a field `stack_size: ItemStack`  and make `ItemStack` an enum between `One/Unstackable` and `Stack(u32)`. This also eliminates any `StackId` tracking.

### Item Weight

To avoid floating point errors, store weight values as integers measured in thousandths of a pound. The weirdest value I saw checking the PHB is 20 bolts for 1.5 lbs which gives 0.075 lbs/bolt. Switch to float only at end of calculation or when displaying to the user.

# Structure Specifics

### Inventory

The inventory will need to have
- A vector or hashmap representation of the item instances
	- vector of the instance structs themselves or a hashmap of instance IDs to instance structs
- maybe a separate money struct
- maybe lists for attuned/equipped items

### Item Definition (Rules)

The item definition needs to track:
- an item definition ID, either an integer assigned dynamically on data load or an identifying string like `phb.studded_leather`
- display name
- display description
- item weight (optional)
- what type of item (weapon, gear, armor, etc) and any additional subtype data
- item tags (for search/categorization)
- flags for stackability, attunement, is a container, maybe more
	- this may also be mixed into the item subtype data
- some list of its rule effects/modifiers
	- includes data about items with charges (max charges) and their property reset mechanics (long rest, short rest, dawn, dusk, etc)

### Item Instance (Inventory)

The item instance needs to track:
- specific instance identifier and corresponding item rules identifier
- item location (carried, equipped, in a container), probably enum `Carried, Equipped, Container(SomeIdType)`
	- if we allow any item ID then we allow invalid states of items being inside non-container items
- flag for if it's attuned
	- probably needs to be an enum `Unattunable, Unattuned, Attuned` or similar. The aim is to make it so that an unattunable item like a rope cannot have the attunement flag set (well you can just change the enum value but code logic may have to cover that).
- item stack size (see [[Inventory and Items#Item Stacking|Item Stacking]])
- optional representation of item state in some form or another
	- items with charges/uses (current vs max), durability values, etc

### Item Subtype Data

- containers
	- weight capacity
	- possibly allowed item limitations (e.g. only bolts in a crossbow bolt case)
	- weight overrides (e.g., bag of holding)---perhaps this should be handled as a `RuleEffect` or something similar
- armor
	- base AC value
	- dex bonus (full, limited, none)
	- applies stealth disadvantage
- weapons
	- damage dice
	- damage die size
	- damage type
	- optional 2-handed damage die size
	- weapon properties
	- (2024) weapon masteries
- magic items
	- attunable?
- gear
	- ???

Weapon damage data should allow for multiple, i.e., a list of different damages (for example a magic sword that does 1d8 slashing and 1d6 fire).

The shield is an interesting edge case since it's not quite armor but not quite gear. It should most likely be classed as an armor with 0 base AC, no dex bonus, but a `RuleEffect` or similar of an AC bonus +2.

In fact, much of the subtype data could potentially reduce down to `RuleEffect`s for the item.

### Money

I still haven't decided if I want the simple but inflexible route or the complex but homebrewable route.

# Data Structures

external referred types (defined somewhere else)
- `RuleEffect`
internal referred types (I need to figure out their definitions)
- `ItemDefinitionId`
- `ItemInstanceId`
could be either
- `WeaponProperty`
- `WeaponMastery`

TODO
- what should be public and what should be private??
- define above types
- `ItemType`
	- does `ItemType::Gear` need a value?
	- should `ItemType::MagicItem` be split into different types?
- define data structure for the armor dex bonus (full, +X, none)
	- this should probably be expressed in a `RuleEffect` or something
- fill in `ItemState` (or even should it be a set enum?)
- eliminate potential "stackable and attunable" state in `ItemDefinition`
	- `ItemInstance` has a similar problem but spread across the `stack: ItemStack` and `attuned: Attunement` fields. An `EquippedAttuned` enum seems very clunky but might eliminate it
- containers
	- `ContainerId` vs `ItemInstanceId` or whatever for `ItemLocation` 
	- enforce item limitations like bolts for a case of bolts?
	- how to handle overrides (allowing more weight, ignoring item limitations, ignoring contained weight like bag of holding, etc)
- for `WeaponData`, figure out what the best way of expressing properties and masteries is, both for item data and for use in the broader engine logic.

```Rust
struct Inventory {
	money: Money,
	// TODO
}

struct ItemDefinition {
	id: ItemDefinitionId,
	name: String,
	description: String,
	weight: Option<u32>,
	item_type: ItemType,
	tags: Vec<Tag>,
	stackable: bool,
	attunable: bool,
	effects: Vec<RuleEffect>,
}

struct ItemInstance {
	id: ItemInstanceId,
	definition_id: ItemDefinitionId,
	location: ItemLocation,
	attuned: Attunement,
	stack: ItemStack,
	state: ItemState,
}

enum Attunement {
	Unattunable,
	Unattuned,
	Attuned,
}

enum ItemStack {
	Unstackable,
	Stackable(u32),
}

enum ItemType {
	Armor(ArmorData),
	Weapon(WeaponData),
	Gear(GearData),
	MagicItem(MagicItemData),
	Container(ContainerData),
}

enum ItemLocation {
	Carried,
	Equipped,
	InContainer(ContainerId),
}

enum ItemState {
	None,
	// ???
}

struct Money {  
    cp: u32,  
    sp: u32,  
    ep: u32,  
    gp: u32,  
    pp: u32,  
}

struct ArmorData {
	base_ac: u8,
	dex_bonus: IDontKnow,
	stealth_disadvantage: bool,
}

struct WeaponData {
	damage: Vec<Damage>,
	two_haned: Option<Vec<Damage>>
	properties: Vec<WeaponProperty>,
	mastery: WeaponMastery,
}

struct GearData {
	// ???
}

struct MagicItemData {
	// ???
}

struct ContainerData {
	max_weight: u32,
	// ???
}
```
