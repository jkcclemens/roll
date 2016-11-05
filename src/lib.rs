#![recursion_limit = "1024"]

#![feature(try_from)]
#![feature(plugin)]

#![plugin(clippy)]

#[macro_use]
extern crate error_chain;
extern crate rand;

pub mod rolls;
pub mod errors;

#[cfg(test)]
mod test;

pub use rolls::Roller;
pub use rolls::dice;
pub use rolls::Rolls;
