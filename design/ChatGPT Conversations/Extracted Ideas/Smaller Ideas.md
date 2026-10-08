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

# Rests

[[Part 1 Raw#10. Long Rest / Short Rest System|Link]]

```Rust
fn short_rest(state: &mut CharacterState) {
    state.spent_hit_dice.clear();
}

fn long_rest(sheet: &CharacterSheet, state: &mut CharacterState) {
    state.current_hp = sheet.max_hp;
    state.spell_slots_used.clear();
    state.spent_hit_dice.clear();
}
```

# General Rules Repository

[[Part 1 Raw#6. Rule Database|Link]]

```Rust
pub struct Rules {
    pub races: HashMap<String, Race>,
    pub classes: HashMap<String, Class>,
    pub features: HashMap<String, Feature>,
    pub feats: HashMap<String, Feat>,
}
```