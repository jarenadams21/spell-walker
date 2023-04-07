use std::io;
use app::spell_workbench;
use app::create_wizard;
use app::create_backpack;
use app::create_spellpack;
use app::spell_handling::spell_views::Spell;


fn main() {

   let user = create_wizard();

   /* Last user action */
   let mut selection = String::new();

   /* main backpack */
   let mut backpack = create_backpack("PACKPANTHR".to_string());
   backpack.add("Boots".to_string(), 1);
   backpack.print_bag();

   /* main spell_bag */
   let mut spell_bag = create_spellpack("SPELLZ".to_string());
   let mut pot : Spell = Spell::create_spell("known for its 'snail-like' ability".to_string());
   spell_bag.add("turbo".to_string(), pot);


   println!("\nHi {} Welcome to backpacking!\n", user.get_name());
   println!("You have been equipped with a basic set of boots to get started on your journey! Good luck!\n");

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

                spell_workbench(&spell_bag);
            },
            _ => {
                ();
            }
       }

       break;

   }
}