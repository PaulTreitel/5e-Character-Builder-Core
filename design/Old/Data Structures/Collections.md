A collection represents all of the possible things that one could add to a character from a new sourcebook or through homebrew. All objects in a collection share that collection as their listed source.

# Classes

Classes **must** have:
* Hit die
* Proficiencies (armor, weapons, 2 saving throws, optional tool(s), choices for 2-4 skills)
* List of pieces of starting equipment, including choices between them
	* XdY gold value if you're skipping starting equipment
* List of features, including scaling features (eg sneak attack dice, artificer infusions known, sorcery points)
* Multiclass prerequisites
* Subclasses, including level at which subclass is chosen

A class **can** have:
- Spell list
- Number of cantrips known
- Spell slot progression (including Pact Magic progression)
- Features that replace base class features (Tasha's revisions)
- ???
## Subclasses

A subclass can have:
- List of features, including scaling features
- Spell list
- Grant spells (either granted prepared or granted to spell list)
- ???

# Spells

A spell has:
* Name
* Level
* School
* Casting time
* Range
* **Optional** Area
* Components (V/S/M)
* Duration
* Concentration (boolean)
* Ritual (boolean)
* **Optional** Save type
* **Optional** Attack type
* Which class/subclass lists it's on
* Description
* **Optional** upcasting bonus

Additional metadata might include
* Tag for type of spell
* Damage type
* Conditions it can inflict
* effects to display on character sheet

# Items

An item **must** have:
* Name
* Description
* Rarity
* Magic (boolean)

An item **can**:
* Be a weapon
	* Have weapon properties, including extra rider damage
* Be armor
	* Have armor properties
* Be equipped
* Be a container
* Have weight
* Have gold cost
* Require attunement
* Increase or set an ability score (which can stack)
* Grant proficiencies
* Grant additional senses
* Grant advantage on ability checks or saving throws
* Grant spells
* Increase passive skill
* Have charges/numbered activations per short rest/long rest/day
* Give bonuses to attacks, AC, or spell save DCs
* Set AC
* Have activations
* Be consumable
* Have prerequisites
* Increase maximum hit points (by constant or non-constant values)
* Grant additional movement types
* Grant resistances and immunities
## Weapons

In addition to the possible things for an item, a weapon has:
* Damage dice
* Damage type
* Category (simple/martial, ranged/melee)
* List of properties, including ranged and thrown values and versatile dice
## Armor

In addition to the possible things for an item, armor has:
* Base AC value
* **Optional** dexterity bonus cap
* **Optional** strength requirement
* Stealth disadvantage (boolean)
* Armor type (light, medium, heavy, unarmored)

# Proficiencies

A proficiency is either a tool, armor, weapon, or language.

# Backgrounds

A background has:
* Name
* Description
* Two skill proficiencies
* Two language or tool proficiencies (any pairing but each proficiency is only language or only tool)
* List of pieces of equipment and money
* d8 suggested personality traits
* d6 suggested ideals
* d6 suggested bonds
* d6 suggested flaws
* **Optional** dX suggested choice (criminal specialty, defining event, etc)

# Species

A species **must** have:
* Name
* Creature type(s)
* Size (including selectable sizes)
* Speed
* Languages

A species **can** have:
* Ability score increases
- Subspecies
	- Subspecies can also have ability score increases
- Proficiencies
- Senses
- Advantage on saving throws or ability checks
- Resistances and immunities
- Increase maximum hit points (level-scaling)
- Text feature descriptions
- Grant innate spells at different levels, with specified or chosen casting ability, including limited activations
	- includes spells with shared activation limits (ie cast X or Y once per day)
	- includes "chosen from a class list" spells
- Grant a feat
- Attacks (eg breath weapon, talon attack)
- Grant additional movement types (and disable them from wearing armor types)
- dX feature table
- Activated abilities, including limited activations
- Grant expertise
- Gain temporary, swappable proficiency
- Change size for effective carry/drag/lift capacities
- Add proficiency bonus to initiative rolls
- Set default AC
- Prohibit wearing armor
- Change weapon reach
- Option to select different effects
- Add damage die to critical hits
- Grant bonus to AC
- Replace unarmed strike die
- Grant half-proficiency

# Feats

A feat **must** have:
- Name
- Description

A feat **can**:
- Increase ability score
- Have prerequisites
- Increase maximum health (level scaling)
- Grant access to a creature
- Grant access to a spell (fixed or chosen), with fixed or chosen spellcasting ability, with or without set # of uses, etc
- Grant any kind of activated ability or class feature
- Grant an attack
- Grant access to a creature
- Grant advantage or disadvantage, half proficiency (round up or down), proficiency, or expertise to any check or save
	- Grant proficiency in a language, weapon, or armor
- Grant senses
- Grant, set, reduce, or remove additional movement types
- Add or ignore a weapon property
- Ignore restrictions (dual wield, armor speed reduction, AC dex bonuses)
- Set carrying capacity size-equivalent
- Grant resistance, immunity, or weakness
- Set size
- Grant flat or dice-based bonus to any numerical value, including stacking ability score bonus
- Increase spell range or damage

# Conditions

A condition has:
* Name
* Description

Additional data for a condition might include:
* effects to display on character sheet

# Monsters?

It may be worth adding a monster system wildshapes, transformations, summons, familiars, etc.

A monster has:
- Name
- Size
- Creature Type
- Alignment
- AC
- Hit points and hit dice
- Speed(s)
- Ability scores
- Senses
- Languages
- Challenge Rating
- Proficiency Bonus
- Actions

A monster **can** have:
- Saving throw proficiencies
- Skill proficiencies
- Damage resistances
- Damage vulnerabilities
- Damage immunities
- Condition immunities
- Features
- Bonus Actions
- Reactions
- Legendary Actions
- Lair Actions
- Mythic Actions