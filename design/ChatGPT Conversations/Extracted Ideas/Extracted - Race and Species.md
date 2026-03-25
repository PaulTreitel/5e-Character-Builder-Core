# Links

### Part 1
- [[Part 1 Raw#1. Core Principle Treat Rules as Data|a little JSON]]
- [[Part 1 Raw#6. Race Example|Race struct and JSON]]
- side mention in a [[Part 1 Raw#6. Rule Database|Rule Database]]

### Part 2
- side mention in a [[Part 2 Raw#8. Rule Database|Rule Database]]

### Part 5
- [[Part 5 Raw#7. Species (Race / Species System)|short YAML]]
	
# Extracted Data Structures and Representations

### Part 1

```JSON
{
  "name": "Elf",
  "effects": [
    { "type": "stat_bonus", "stat": "dexterity", "value": 2 }
  ]
}
```

```Rust
pub struct Race {
    pub id: RaceId,
    pub name: String,
    pub speed: u32,
    pub size: Size,

    pub ability_bonuses: Vec<AbilityBonus>,
    pub features: Vec<FeatureId>,
}
```

```JSON
{
  "id": "elf",
  "name": "Elf",
  "speed": 30,
  "size": "medium",
  "ability_bonuses": [
    { "ability": "dexterity", "amount": 2 }
  ],
  "features": ["darkvision", "keen_senses"]
}
```

```Rust
pub struct Race {
    pub id: String,
    pub features: Vec<String>,
}
```

### Part 2

```Rust
pub struct Race {  
    pub id: RaceId,  
    pub speed: u32,  
    pub features: Vec<FeatureId>,  
}
```

### Part 5

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

# (Pseudo)Code