use crate::spell::Spell;



pub struct PactMagic {
    cantrips: Vec<Spell>,
    spell_slots: u8,
    slot_level: u8,
    spells_known_ct: u8,
}