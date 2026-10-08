# Macros

Macros are used to write "functions" which will then be expanded out during the processing of the YAML file. This is to allow for some common operations to be done without writing them out every time over. Unlike normal functions, they have no output; rather they replace macro calls with the expansion code in the macro much like a `#define` in C, using inputs as a template. For example, consider the following:
```YAML
- id: stat_below
  type: macro
  args:
    - stat_input
    - threshold

  expansion:
    expr: if
    args:
      - condition: 
        expr: less_than
        args:
          - id: var: stat_input
          - var: threshold
      - var: stat_input
      - null
```
If we then need a feat which lets us select between increasing STR or DEX by 1 up to a maximum of 20, we could write
```YAML
choice:
  type: ability_score_increase
  count: 1
  options:
    - id: macro.stat_below
      args: [id: score.strength, 20]
    - id: macro.stat_below
      args: [id: score.dexterity, 20]
```
Which will then expand out to
```YAML
choice:
  type: ability_score_increase
  count: 2
  options:
    - expr: if
      args:
        - condition: 
          expr: less_than
          args:
            - id: score.strength
            - 20
        - score.strength
        - null
    - expr: if
      args:
        - condition: 
          expr: less_than
          args:
            - id: score.dexterity
            - 20
        - score.dexterity
        - null
```

## Macro Syntax

```YAML
# defining a macro
- id: macro_name
  type: macro
  args:
    - input1
    - input2
    # ...
expansion:
  # YAML expression structure
  # use var: inputname to get an input
  
# using a macro
- id: macro.macro_name
  args: [my_input1, my_input2]
```
# Full Structures
## Items

### Weapons
#### Basic Weapon
```YAML
- id: str (weapon_id)
  name: str (Weapon Name)
  type: item
  subtype: basic_weapon
  rarity: common
  stackable: false
  equippable: true
  attunement: false
  tags: [weapon, damage, combat, simple | martial, melee | ranged, any other tags like weapon type]
  weight: [int, pound | pounds] | null
  cost: null | {gp: optional int, ep: optional int, pp: optional int, sp: optional int, cp: optional int}

  description: >-
    str description

  rules:
    damage:
      expr: dice
      args: [int, int]
      type: damage_type
    properties:
      - id: property_name
        value: optional property data (versatile die expr, range distances, etc)
```

#### Magic Weapon
```YAML
# has attributes fron a basic weapon, differences noted here
- id: str (weapon_id)
  subtype: magic_weapon
  attunement: bool

  rules:
	# replaces normal damage and properties, which are inherited from
    - base: item.basic_weapon.(item this is based on)

    - additional_damage:
	    (optional structure like normal damage)

	# optional additional rules
    - phase: magic_items
      type: grant_resource
      # whatever other fields required
      
    # optional additional rules
    - phase: actions
      type: grant_action
      action: weapon_id.action.light
      
    # ...

# specific action granted by weapon
- id: str (weapon_id.action.action_id)
  name: str (Action Name)
  type: action
  action: str (action | reaction | bonus_action | 1 minute | 10 minutes | 1 hour | 8 hours | 12 hours | 24 hours)
  description: >-
    str description
```

### Armor

#### Basic Armor
```YAML
- id: str (armor_id)
  name: str (Armor Name)
  type: item
  subtype: basic_armor
  rarity: common
  stackable: false
  equippable: true
  attunement: false
  tags: [armor, armor_size, other_traits]
  weight: [int, pound | pounds] | null
  cost: null | {gp: optional int, ep: optional int, pp: optional int, sp: optional int, cp: optional int}

  description: >-
    str description

  rules:
    - phase: derived_stats
      target: defense.ac.base
      operation: set
      value: int (armor base value)

	# optional for armors that cap dex
    - phase: derived_stats
      target: defense.ac.max_dex
      operation: set
      value: int (dex cap)
      
    # optional for armors with strength requirements
    - phase: derived_stats
      target: speed.walking
      operation: sub
      value:
	    expr: if
	    args:
		  - condition: 
		    expr: less_than
		    args:
			  - score.strength
			  - int (armor str req)
			- 10
			- 0

	# optional for armors with stealth disadvantage
    - phase: feature_modifiers
      target: defense.ac.stealth_penalty
      operation: set
      value: true
```

## Feats
```YAML
# base feat
- id: str (feat_id)
  name: str (Feat Name)
  type: feat
  
  description: str "give description"
  
  rules:
	- phase: str
	  type: str
	  # whatever other fields required
	# ...
	  
# subfeature
- id: str (feat_id.feature_id)
  name: str (Feature Name)
  type: str (whatever type, like feature option)
  
  rules:
	- phase: str
	  type: str
	  # whatever other fields required
	# ...
  
# specific subfeature: action the feat grants
- id: str (feat_id.action.feature_id)
  name: str (Feature Name)
  type: str (whatever type, like feature option)
  
  rules:
	- phase: str
	  type: str
	  # whatever other fields required
	# ...
```


## Spells

```YAML
- id: str (spell_name_id)
  name: str (Spell Name)
  type: spell
  level: int [0-9]
  school: str (school_name)
  tags: [whatever, tags]

  casting_time: str (action | reaction | bonus_action | 1 minute | 10 minutes | 1 hour | 8 hours | 12 hours | 24 hours)
  duration: instantaneous | special | until dispelled | [int, (round | minute | hour | day)]
  range: touch | [int, (feet | mile | miles)]
  components:
    - verbal: bool
    - somatic: bool
    - material: str | null

  description: >-
    str spell description
    
  upcast_description: >-
	optional str (upcast description)
  
  rules:
	area: optional (
		[int, (feet | miles), (sphere | cone | cube | line)]
		| 
		[int, (feet | miles), int (feet | miles), cylinder] # width, height
	)
	defense: optional (defense.ac | defense.save.stat)
	damage: optional {expr: for_the_dice, type: damage_type}
	count: optional (expr, such as for magic missile)
	targets: optional (expr, such as for hold person)
	additional_damage: {same as damage, just secondary effect}
	# any YAML rule expressions
```