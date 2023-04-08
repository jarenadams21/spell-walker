use std::io;
use app::spell_workbench;
use app::create_wizard;
use app::create_backpack;
use app::create_spellpack;
use app::craft_a_spell;
use app::spell_handling::spell_views::Spell;


fn main() {

   let user = create_wizard();

   /* Last user action */
   let mut selection = String::new();

   /* main backpack */
   let mut backpack = create_backpack("PACKPANTHR".to_string());
   backpack.add("Boots".to_string(), 1);

   /* main spell_bag */
   let mut spell_bag = create_spellpack("SPELLZ".to_string());
   let mut pot : Spell = Spell::create_spell("turbo".to_string(),
    "known for its 'snail-like' ability. this potion is free, but requires a 50 health reduction to craft.".to_string(),
    0, 0);
   let mut z4 : Spell = Spell::create_spell("z4".to_string(),
    "known to cause extreme levels of chill".to_string(), 0, 2);
   spell_bag.add(pot, 1);
   spell_bag.add(z4, 2);


   println!("\nHi {} Welcome to backpacking!\n", user.get_name());
   println!("You have been equipped with a basic set of boots, a turbo potion, and two z4 potions to get started on your journey! Good luck!\n");

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
                craft_a_spell(&mut spell_bag);
                break;
            },
            _ => {
                ();
                continue;
            }
       }

       println!("Main Over!");
       break;
       

   }
}