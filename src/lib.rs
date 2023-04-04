pub mod spell_handling;
pub mod user_handling;
pub mod inventory_handling;

pub use crate::spell_handling::spell_views;
pub use crate::user_handling::user_construction;
pub use crate::user_handling::user_construction::User;
pub use crate::inventory_handling::backpack;
pub use crate::inventory_handling::backpack::Pack;

/* UI / Pages Related */


/* Spell Related */
pub fn spell_workbench() {
    spell_views::print_craftable_spells();
}

/* User Related */
pub fn create_wizard() -> User {
    user_construction::create_user()
}

/* Backpack Related */
pub fn create_backpack(name: String) -> Pack {

    backpack::create_pack(name)
}
