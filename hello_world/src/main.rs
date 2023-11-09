///associate greetings module with this crate
mod greetings;
mod how_you_hold_data_for_operations;

//extern crate hello_world_lib;

use greetings::{ spanish, french, english};

// use how_you_hold_data_for_operations::primitives::compound;

//use how_you_hold_data_for_operations::primitives::compound;


fn main() {
    let number = [1,2,3,4,5];

    let mut greeting = "Hello there";
    greeting = "Hwfr";
    println!("Hello, world!");
    //println!("{}", default_greeting());
    println!("{}", spanish::default_greeting());
    println!("{}", french::default_greeting());
    println!("{}", english::default_greeting());
    //println!("{}",hello_world_lib::greeting_from_lib());
  
  //calling the main & analyze from the compound.rs
   how_you_hold_data_for_operations::primitives::compound::analyze_slice(&number);
   how_you_hold_data_for_operations::primitives::compound::main();

   //calling the main from the scalar.rs
   how_you_hold_data_for_operations::primitives::scalar::main();
}
