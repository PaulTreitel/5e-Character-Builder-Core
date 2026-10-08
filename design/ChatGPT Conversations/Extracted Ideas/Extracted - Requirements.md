# Links

### Part 1
1. [[Part 1 Raw#6. Requirement System||Rust model]]
2. [[Part 1 Raw#7. Requirement Storage|Requirement storage]]

# Extracted Data Structures and Representations

### Part 1

#### #1
```rust
enum Requirement {
    Class(String),
    Level(u8),
    StatAbove { stat: Stat, value: u8 },
}
```

#### #2
```JSON
{
  "type": "level_at_least",
  "value": 3
}

{
  "type": "class",
  "value": "wizard"
}
```

```Rust
enum Requirement {
    LevelAtLeast(u8),
    Class(String),
    StatAbove { stat: Stat, value: i32 },
}
```

# (Pseudo)Code

### Part 1

#### #1
```Rust
fn check_requirement(character: &Character, req: &Requirement) -> bool
```

