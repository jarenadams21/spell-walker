/* Represents a traveler's backpack */
pub mod backpack {
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct Pack {

        name: String,
        inventory: HashMap<String,i32>,
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

    pub fn create_pack(name: String) -> Pack {

        Pack {
            name,
            inventory: HashMap::new(),
            capacity: 32,
            item_count: 0,
        }
    }
}