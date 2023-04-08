pub mod spell_handling;
pub mod user_handling;
pub mod inventory_handling;

pub use crate::spell_handling::spell_views;
pub use crate::user_handling::user_construction;
pub use crate::user_handling::user_construction::User;
pub use crate::inventory_handling::backpack;
pub use crate::inventory_handling::backpack::Pack;
pub use crate::inventory_handling::backpack::SpellPack;
pub use crate::spell_handling::spell_views::Spell;

/* UI / Pages Related */


/* Spell Related */
pub fn spell_workbench(pack: &SpellPack) {
    spell_views::print_craftable_spells(pack);
}

pub fn craft_a_spell(pack: &mut SpellPack) {
    spell_views::prompt_user_spell_creation(pack);
}

/* User Related */
pub fn create_wizard() -> User {
    user_construction::create_user()
}

/* Backpack Related */
pub fn create_backpack(name: String) -> Pack {

    backpack::create_pack(name)
}

pub fn create_spellpack(name: String) -> SpellPack {

    backpack::create_spellpack(name)
}
