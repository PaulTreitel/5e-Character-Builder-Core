use known::SpellcastingKnown;
use pact_magic::PactMagic;
use prepared::SpellcastingPrepared;

pub mod known;
pub mod prepared;
pub mod pact_magic;

pub enum Spellcasting {
    Known(SpellcastingKnown),
    Prepared(SpellcastingPrepared),
    Pact(PactMagic),
}

mod spellcasting {

}