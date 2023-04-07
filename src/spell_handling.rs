
pub mod spell_views {

    use crate::SpellPack;

    #[derive(Eq, Hash, PartialEq, Clone, Debug)]
    pub struct Spell {

        name: String,
        description: String,
        required_level: i32,
        required_monster_metal: i32,
    }

    impl Spell {

        pub fn create_spell(name: String, description: String, required_level: i32, required_monster_metal: i32) -> Spell {

            Spell {
                name,
                description,
                required_level,
                required_monster_metal,
            }
        }

        pub fn print_spell(&self) {
            dbg!(self);
        }
    }

    pub fn print_all_spells() {

        ();
    }

    // Spells are craftable if the user
    // has the materials to make it, and they
    // have 'learned' or 'unlocked' it
    pub fn print_craftable_spells(pack: &SpellPack) {

     let mut EXISTING_SPELLS : Vec<Spell> = vec![Spell {
        name: "test".to_string(),
        description: "test".to_string(),
        required_level: 0,
        required_monster_metal: 0,
    }];


        println!("Here are the available spells to craft: \n");
        dbg!(EXISTING_SPELLS);

        println!("Your current spells:\n");
        pack.print_bag();
    }

    // Prompts a user to craft a spell
    // If the user chooses an already existing spell,
    // their eligiblity to do so is checked and the 
    // appropriate action is taken.
    // If the user opts to craft their own new spell,
    // that is handled and added to the universal list
    // of spells that are now craftable (EXISTING_SPELLS).
    pub fn prompt_user_spell_creation() {

    }
}