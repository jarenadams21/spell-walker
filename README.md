# spell-walker
## We love casting spells
  Chapter 8 benchmark project for Rust documentation learning. Want to test my grasp on collections, structs, module organization among others

## Todo
  1. Inventory / Backpack transactions
    
      FIRST: Code Organization, 
      SECOND: FIX SpellPack Hashmap and Spell struct to store names in Spells and map Spells to i32 values, 
      THEN: Fix the code that does not work
    
        [ X ] Insertions
        [ ] Deletions
        [ ] Updates/increasing count  
        [ ] Conveniency functionality (getters,setters,etc.)
        [ ] Finish Craft a Spell view
        [ ] Finish Monster Park (with some help from liberation::main...
        [ ] Design Spell framework
        [ ] Add timers to unicode screens like loading instead of spitting out all progress bars at once
    
  
  2. Combine metrics, view eqiupment, etc. into one "view yourself"

## Crates & Sources Used
  1. Loading unicode generation: https://changaco.oy.lc/unicode-progress-bars/

## Instructions
  i. cargo build
  ii. cargo run

### Tracking Progress

* Craft a Spell View
    - (User-definable rules, what has changed, cleaner inventory showcase)
    - User should define how many they would like to make, and internal proccesses will validate the request. If the user can't make as much as request, the next closest possible amount to craft is presented as an option
    - Clearer UI for update inventory (what has changed, maybe an original vs now hashmap view)
    - Actually process costs/health deductions
* Spell Framework
    - All spells should hold an internal effects struct, vec, enum etc that calls on a Spell impl function that proccesses spell action for a given user (and handle the associated updates to inventory, other entities, etc)
