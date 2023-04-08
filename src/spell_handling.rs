
pub mod spell_views {

    use crate::SpellPack;
    use std::io;


    #[derive(Eq, Hash, PartialEq, Clone, Debug)]
    pub struct Spell {

        pub name: String,
        pub description: String,
        pub required_level: i32,
        pub required_monster_metal: i32,
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

    pub fn provide_vector_of_existing_spells() -> Vec<Spell> {

        let mut EXISTING_SPELLS : Vec<Spell> = vec![Spell {
            name: "turbo".to_string(),
            description: "known for its 'snail-like' ability.".to_string(),
            required_level: 0,
            required_monster_metal: 0,
        }, Spell {
            name: "z4".to_string(),
            description: "known to cause extreme levels of chill".to_string(),
            required_level: 0,
            required_monster_metal: 2,
        }];

        EXISTING_SPELLS
    }

    // Spells are craftable if the user
    // has the materials to make it, and they
    // have 'learned' or 'unlocked' it
    pub fn print_craftable_spells(pack: &SpellPack) {

        let mut EXISTING_SPELLS : Vec<Spell> = provide_vector_of_existing_spells();

        println!("\nHere are the available spells to craft: \n");

        for spell in &EXISTING_SPELLS {
            println!(" --> {:?}: {:?}", spell.name, spell.description);
        }

        println!("\nYour current spells:");
        pack.print_bag();
    }

    // Prompts a user to craft a spell
    // If the user chooses an already existing spell,
    // their eligiblity to do so is checked and the 
    // appropriate action is taken.
    // If the user opts to craft their own new spell,
    // that is handled and added to the universal list
    // of spells that are now craftable (EXISTING_SPELLS).
    pub fn prompt_user_spell_creation(pack: &mut SpellPack) {

        /* Last user action */
        let mut selection = String::new();

        /* Existing Spells */
        let mut EXISTING_SPELLS : Vec<Spell> = provide_vector_of_existing_spells();
 
        'await_valid_action : loop {

        println!("Type in the name of the spell you'd like to CRAFT (plz be exact): ");
        io::stdin()
        .read_line(&mut selection)
        .expect("failed to parse selection...");  
        
        /* User selects action */
       let selection: String = match selection.trim().parse() {
        Ok(spell_name) => spell_name,
        Err(_) => continue,
        };

        for spell in &EXISTING_SPELLS {
            if (spell.name.eq(&selection)) {
                pack.update_spell_count(spell.clone(), 2);
                println!("Crafted!");
                pack.print_bag();
                break 'await_valid_action;
            }
        }

        println!("non-existent brother try another, or... (Press 1 to find out, Press 0 to try again)");
        break 'await_valid_action;
        


    }

    }
}