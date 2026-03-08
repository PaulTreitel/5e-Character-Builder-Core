mod spell;
mod basic;
mod background;
mod proficiencies;
mod monster;
mod inventory;
mod feats;
mod character;
mod class;
mod race;
mod spellcasting;

fn main() {
    println!("Hello, world!");
    feats::test_create_feats::make_feats();
    // let _x = class::Class{};
    // let _y = character::Character{};
}