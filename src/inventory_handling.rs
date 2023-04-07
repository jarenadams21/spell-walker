/* Represents a traveler's backpack */
pub mod backpack {

    use std::collections::HashMap;
    pub use crate::Spell;

    #[derive(Clone, Debug)]
    pub struct Pack {

        name: String,
        inventory: HashMap<String,i32>,
        capacity: i32,
        item_count: i32,
    }

    #[derive(Clone, Debug)]
    pub struct SpellPack {

        name: String,
        inventory: HashMap<Spell, i32>,
        capacity: i32,
        item_count: i32,
    }

    impl Pack {

        pub fn add(&mut self, key: String, amount: i32) {

            self.inventory.insert(key, amount);
        }

        pub fn print_bag(&self) {
            println!("{:?}", self.inventory);
        }
    }

    impl SpellPack {

        pub fn add(&mut self, key: Spell, val: i32) {

            self.inventory.insert(key, val);
        }

        pub fn print_bag(&self) {
            println!("{:?}", self.inventory);
        }

    }


    pub fn create_pack(name: String) -> Pack {

        Pack {
            name,
            inventory: HashMap::new(),
            capacity: 32,
            item_count: 0,
        }
    }

    pub fn create_spellpack(name: String) -> SpellPack {

        SpellPack {
            name,
            inventory: HashMap::new(),
            capacity: 32,
            item_count: 0,
        }
    }
}