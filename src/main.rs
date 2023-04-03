use std::collections::HashMap;
use std::io;
use app::spell_workbench;
use app::create_wizard;

fn main() {

   let user = create_wizard();

   /* Last user action */
   let mut selection = String::new();

   println!("\nHi {} Welcome to backpacking!\n", user.get_name());

   /* Program */
   'main_program: loop {

        /* Landing Page UI */
       println!("What would you like to do?");
       println!("Current coins: {}\n", user.get_coins());
       println!("1. Check inventory");
       println!("2. Craft a spell");
       println!("3. Cast a spell");
       println!("4. View equipment");
       println!("5. Craft equipment");
       println!("6. Marketplace");
       println!("7. Check metrics");
       println!("8. Sleep");
       println!("9. Monster Park (scary, dangerous even)");
       println!("Selection: ");
       io::stdin()
       .read_line(&mut selection)
       .expect("failed to parse selection...");

       /* User selects action */
       let selection: u32 = match selection.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
       };

       match selection {
           
            2 => {

                spell_workbench();
            },
            _ => {
                ();
            }
       }

       break;

   }
}