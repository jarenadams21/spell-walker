
// User structure formation capabilities
pub mod user_construction {

    use std::io;

    #[derive(Debug, Clone)]
    pub struct User {

    pub name: String,
    pub coins: i32, 
    }

    impl User {

        pub fn get_name(&self) -> &str {
            
            &self.name
        }
    
        // Return the traveler's coins
        pub fn get_coins(&self) -> i32 {
    
            self.coins
        }
    }

    // Prompts a user to enter their information
    // to make a User struct for their travels
    pub fn create_user() -> User {

        /* Who is walking in? */
         println!("Hello, what is your name?");
        let mut traveler = String::new();

        // read_line returns a "Result" enum 
        // with variants 'Ok' and 'Err'
        io::stdin()
        .read_line(&mut traveler)
        .expect("failed to parse name...");

         /* Player structure */
        let player = User {
        name: traveler,
        coins: 0,
        };

        player
    }

}