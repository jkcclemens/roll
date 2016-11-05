#![feature(try_from)]

extern crate roll;

use roll::{Roller, Rolls};
use roll::dice::Dice;
use std::convert::TryInto;
use std::env::args;

fn main() {
  let roller = Roller::new();
  let args: Vec<_> = args().skip(1).collect();
  for arg in args {
    let dice: Dice = match arg.clone().try_into() {
      Ok(d) => d,
      Err(e) => {
        println!("Can't roll {}! {}", arg, e);
        continue;
      }
    };
    println!("Rolling {}...", arg);
    let res = roller.roll(&dice);
    let rolls_as_strings: Vec<String> = res.rolls.iter().map(|r| r.to_string()).collect();
    let rolls_string = rolls_as_strings.join(", ");
    println!("  {} = {{{}}}", res.result, rolls_string);
  }
}
