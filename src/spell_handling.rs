pub mod spell_views {

    use crate::SpellPack;

    #[derive(Clone, Debug)]
    pub struct Spell {

        description: String,
    }

    impl Spell {

        pub fn create_spell(description: String) -> Spell {

            Spell {
                description,
            }
        }

        pub fn print_spell(&self) {
            dbg!(self);
        }
    }

    // Spells are craftable if the user
    // has the materials to make it, and they
    // have 'learned' or 'unlocked' it
    pub fn print_craftable_spells(pack: &SpellPack) {

        println!("Here are the available spells to craft: \n");
        pack.print_bag();
    }
}