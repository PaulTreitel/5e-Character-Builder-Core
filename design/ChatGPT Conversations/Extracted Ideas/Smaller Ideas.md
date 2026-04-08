# Character Builder struct

[[Part 1 Raw#4. Application Layer (Character Builder Workflow)|Link]]

Create a struct specifically for building a `Character` struct using the builder pattern in Rust

```Rust
pub struct CharacterBuilder {
    character: Character,
}

impl CharacterBuilder {

    pub fn new(name: String) -> Self { }

    pub fn set_race(&mut self, race: Race) { }

    pub fn set_class(&mut self, class: Class) { }

    pub fn assign_stats(&mut self, stats: Stats) { }

    pub fn build(self) -> Character { }
}
```

# Recommended Crates

[[Part 1 Raw#11. Recommended Rust Crates|Link]]