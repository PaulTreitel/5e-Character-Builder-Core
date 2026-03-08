![[Collections#Spells]]

```
Spell (
	Name: string
	Level: integer
	School: enum (spell schools)
	Casting time: enum (A, BA, Reaction, 1 minutes, 10 minutes, 1 hour, 8 hours, 24 hours)
	Range: enum (self, touch, feet (integer))
	Option Area: (
		size: integer
		shape: enum (square, cube, sphere, cone, cylinder, line)
	)
	Components: (
		3 booleans
		Option string description
	)
	Duration: enum (instantaneous, rounds: (integer), minutes: (integer), hours: (integer), days: (integer), until dispelled, until dispelled or triggered, special)
	Concentration: boolean
	Option Save type: enum (STR, DEX, CON, WIS, INT, CHA)
	Option Attack type: enum (melee spell, ranged spell, melee weapon, ranged weapon)
	Class/Subclass lists: array of strings
	Description: string
	Option upcasting bonus: ???
)
```

Handling the upcasting bonus is difficult because of the different ways it works:
- For cantrips, scaling is based on character level, unlike leveled spells, and increases dice.
- For spells like *spirit guardians* or *call lightning*, it increases dice.
- For spells like *bless* or *hold person*, it increases the number of targets.
- For spells like *ice knife*, it increases secondary dice.
- For spells like *dispel magic* and *counterspell*, it has a special scaling effect.
- For spells like *spiritual weapon* and *elemental weapon*, it scales at specific levels or only increases every couple levels.
- For spells like *fog cloud* and *creation*, the spell area scales.