# roll

Just a quick dice-rolling library in Rust.

```rust
#![feature(try_from)]
extern crate roll;

use roll::{Roller, Rolls};
use roll::dice::Dice;

fn main() {
  // Create a new roller
  let roller = Roller::new();
  // Get some dice to roll
  let some_dice: Dice = "2d6".try_into().unwrap();
  // Alternatively, we can create dice without parsing a string
  let some_dice = Dice {
    amount: 2,
    faces: 6
  };
  // Roll the dice
  let result = roller.roll(&some_dice);
  // Print out each individual roll
  for roll in result.rolls {
    println!("Rolled a {}", roll);
  }
  // Print out the sum of all the rolls
  println!("All for a total of {}", result.result);
  // Roll again
  let second_result = roller.roll(&some_dice);
  // Check if the rolls were the same
  if result == second_result {
    println!("Wow, we rolled the same thing twice.");
  } else {
    println!("Better luck next time.");
  }
}
```
