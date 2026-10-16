
# File Structure

For smaller rule collections, everything can be in one file. In that case, all things must be segmented:
```YAML
collection:
  # collection metadata
classes:
  # class data
spells:
  # spell data
feats:
  # feat data
# etc
```

Otherwise each section should be in its own file. Things that are not core rule types (macros, progressions, and choice pools) can reside in `collection.yaml` or in the other files; if it's used across files it should be in `collection.yaml`.

# Identifier Structure

All rules use hierarchical namespaced IDs:
```YAML
id: srd2014.spell.fireball            # Fireball spell from the SRD
id: phb2014.feat.great-weapon-master  # Great Weapon Master feat from the 2014 Player's Handbook
id: homebrew.class.mystic   # homebrew Mystic class
```

The collection prefix can be omitted within the collection and the rule type can be omitted in its segment
```YAML
# in srd2014 collection
spells:
  - id: fireball
    type: spell # type field indicates the contained namespace
	  
# elsewhere in srd2014 collection
classes:
  - id: wizard 
	type: class
	# etc
	  
	# in the classes section so the classes prefix can be left off
  - id: wizard.feature.spell-list
	3rd:
		# srd2014 prefix can be left off since it's in that collection
	  - id: spell.fireball
		    
# in phb collection
classes:
  - id: sorcerer.feature.spell-list
    type: class-feature
	3rd:
	  - id: srd.spell.fireball
  # etc
		    
# in the homebrew collection
spells:
  - id: weird-fireball
    type: spell
    # etc
classes:
	- id: mystic
	  type: class
	  # etc
	  progression:
		1:
		  - phb.sorcerer.feature.spell-list # gets the sorcerer spell list
```

# Expression Language

## Add, Multiply, Divide

```YAML
# computes 1 + Charisma modifier
add:
  - 1
  - id: modifier.charisma
    
# computes strength modifier - 1
sub:
  - id: modifier.strength
  - 1
    
# computes 5 * paladin level
mul:
  - 5
  - id: paladin.level
    
# divide cleric level by 3, rounding down
div-floor:
  - id: cleric.level
  - 3
    
# divide proficiency bonus by 2, rounding up
div-ceil:
  - id: character.prof-bonus
  - 2
```

## Conditionals (Ternary)

Structure:
```YAML
if:
  - condition
  - value if true
  - value if false
```

Example
```YAML
# Returns 19 if the Strength score is less than 19 and the strength score if it isn't
if:
  - less-than: [id: strength.score, 19]
  - 19
  - id: strength.score
```

### Less Than, Greater Than, Equals, Etc

```YAML
# Returns strength score < 19
less-than:
  - id: strength.score
  - 19
    
# Returns strength score > 19
greater-than:
  - id: strength.score
  - 19
    
# Returns strength score == 19
equal:
  - id: strength.score
  - 19
    
# Returns strength score <= 19
less-equal:
  - id: strength.score
  - 19
    
# Returns strength score >= 19
greater-equal:
  - id: strength.score
  - 19
```

### And, Or, Not

```
and:
  - condition 1
  - condition 2
    
or:
  - condition 1
  - condition 2
    
not: condition
```


TODO what else do we need?

## Dice

```YAML
# structure
dice:
  - die count
  - die size
    
# example: 6d8
dice: [6, 8]
```

## Damage

```YAML
# Example: 8d6 fire + 6d6 radiant damage
damage:
  - dice: [8, 6]
    type: fire
  - dice: [6, 6]
    type: radiant
# Example: 4d4 + 4 thunder
damage:
  - add:
      - dice: [4, 4]
      - 4
    type: thunder
```

## Choice

Example: Direct option list
```YAML
choice:
  # how many you can select
  count: 2
  # (optional) can an option be selected multiple times
  repeat: true
  options:
    - id: score.strength
    - id: score.dexterity
    - id: score.constitution
    - id: score.intelligence
    - id: score.wisdom
    - id: score.charisma
```

Example: Using a choice pool
```YAML
choice:
  count: 2
  repeat: true
  choice-pool: single-ability-score-increase
```

Example: identifier representing a category
```YAML
# choose any feat
choice:
  count: 1
  options:
    - id: feats.all
```

## Sheet Effect
Indicates that an effect should be available as an option for displaying the changes on the character sheet

```YAML
# Adds a toggleable effect to add the character's shield defense bonus to their dexterity saving throws.
feats:
  - id: shield-master
    # ...
    rules:
      - # ...
        target: defense.save.dexterity
        operation: add
        value: {id: defense.ac.shield}
        sheet-effect: toggle
        sheet-title: Shield Master Defense
```


## Resources

# Macros

Macros are shortcuts that expand out into chunks of YAML script. This makes repetitive expressions easier.
Structure:
```YAML
macros:
  - id: macro-identifier
    type: macro
    args:
      - input1
      - input2
      # etc
    expansion:
	  # code that the macro usage will expand into
	  # use var: input1 to use input1
```

Example:
```YAML
macros:
  - id: stat-below
    type: macro
    args:
      - stat-input
      - threshold
    expansion:
      if:
        - less-than: [var: stat-input, var: threshold]
        - var: stat-input
        - null
          
# Usage
- id: macros.stat-below
  args: [id: score.strength, 20]
# expands to
- if:
    - less-than: [id: score.strength, 20]
    - id: score.strength
    - null
```

# Choice Pools

Choice pools are predefined sets of options which may be used in other places

Structure:
```YAML
choice-pools:
  - id: my-choice-pool
    type: choice-pool
    options:
      - option1
      - option2
      # etc
```

Example:
```YAML
choice-pools:
  - id: single-ability-score-increase
    type: choice-pool
    options:
      - id: score.strength
      - id: score.dexterity
      - id: score.constitution
      - id: score.intelligence
      - id: score.wisdom
      - id: score.charisma
        
# Usage
choice:
  count: 2
  choice-pool: single-ability-score-increase
```

# Metadata: collection.yaml

Structure:
```YAML
collection:
  id: homebrew-package-id
  name: Homebrew Collection (2014)
  version "1.0"
  author: My Name
  edition: "2014"
  url: "https://example.com" # optional
  published: Jamuary 1, 2026 # optional
  # TODO any more?
```

# Spells

### Areas

```YAML
# Example: 20ft sphere
area: {type: sphere, radius: 20, unit: feet}

# Example: 10 ft wide, 40ft tall cylinder
area:
  type: cylinder
  height: 40
  width: 10
  unit: feet
```

### Example: Fireball
```
spells:
  - id: fireball
    name: Fireball
    type: spell
    level: 3
    school: evocation
    tags: [damage, fire]

    casting-time: action
    duration: instantaneous
    range: [150, feet]
    components: [v, s, m]
    material: bat guano and sulfur

    description: >-
      ...

    upcast-description: >-
      ...

    rules:
      area: {type: sphere, radius: 20, unit: feet}
      defense: defense.save.dexterity
      damage:
        - dice:
            - add: [5, id: spell.slot-level]
            - 6
          type: fire
```

### Example: Mage Armor

# Feats

# Races/Species

2014 Example:
```YAML
races:
  - id: elf
    name: Elf

    description: >-
      ...

    size:
      description: >-
        Elves range from under 5 to over 6 feet tall and have slender builds.
        Your size is Medium.
      rules:
        - phase: base-stats
          target: size
          operation: set
          value: Medium

    ability-score-increase:
      description: "..."
      rules:
        - phase: base-stats
          type: modifier
          target: score.dexterity
          operation: add
          value: 2

    speed:
      description: "..."
      rules:
        - phase: base-stats
          type: modifier
          target: speed.walking
          operation: set
          value: 30

    languages:
      description: "..."
      rules:
        - phase: feature-modifiers
          type: grant-proficiency
          value: [id: languages.common, id: languages.elven]

    features:
      - name: Age
        description: >-
          ...

      - name: Alignment
        description: >-
          ...

      - name: Keen Senses
        description: You have proficiency in the Perception skill.
        rules:
          - phase: proficiency
            type: grant-proficiency
            value: [id: skill.perception]

      - name: Darkvision
        description: >-
          ...
        rules:
          - phase: feature-modifiers
            type: grant-sense
            value:
              sense: darkvision
              range: 60

      - name: Trance
        description: >-
          ...

      - name: Fey Ancestry
        description: >-
          ...
        rules:
          - phase: feature-modifiers
            type: grant-advantage
            operation: set
            target: defense.save
            value: against being charmed

          - phase: feature-modifiers
            type: grant-immunity
            operation: set
            target: defense.immunities
            value: magical sleep
```
# Background

2014 Example:
```YAML
backgrounds:
  - id: acolyte
    name: Acolyte
    description: "..."

    rules:
      proficiency:
        skill: [id: skill.insight, id: skill.religion]
        language:
          - choice:
              count: 2
              options:
                - id: languages.all

      equipment:
        - id: item.gear.holy-symbol
        - id: item.gear.prayer-book
        - {id: item.gear.incense-stick, count: 5}
        - id: item.gear.vestments
        - id: item.gear.common-clothes
        - id: item.gear.pouch
        - money:
          - gp: 15

    feature:
      - id: shelter-of-the-faithful
        name: Shelter of the Faithful
        description: "..."

    personality-traits:
      - "I idolize a particular hero of my faith, and constantly refer to that \
        person's deeds and example."
      # ...

    ideals:
      - "Tradition. The ancient traditions of worship and sacrifice must be \
        preserved and upheld. (Lawful)"
      # ...

    bonds:
      - "I would die to recover an ancient relic of my faith that was lost long \
        ago."
      # ...

    flaws:
      - "I judge others harshly, and myself even more severely."
      # ...
```

# Classes

# Subclasses

# Feats

# Creatures

# Items

## Item Variants

## Gear

### Containers

## Weapons

### Magic Weapons

## Armor

### Magic Armor

## Magic Items

