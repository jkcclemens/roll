pub mod dice;

use rand;
use rand::distributions::{IndependentSample, Range};
use self::dice::Dice;

/// The result of rolling some `Dice`.
#[derive(Debug, PartialEq, Hash)]
pub struct RollResult {
  /// The individual dice rolled, in order.
  pub rolls: Vec<usize>,
  /// The sum of all the rolled dice.
  pub result: usize
}

pub trait Rolls<T> {
  fn roll(&self, rollable: &T) -> RollResult;
}

/// A simple struct with a `roll` method for each type of rollable object.
#[derive(Debug, Default)]
pub struct Roller;

impl Roller {
  /// Creates a new `Roller`.
  pub fn new() -> Self {
    Roller {}
  }
}

impl Rolls<Dice> for Roller {
  /// Rolls some `Dice` and returns the `RollResult`.
  fn roll(&self, dice: &Dice) -> RollResult {
    let range = Range::new(1, dice.faces + 1);
    let mut rng = rand::thread_rng();
    let rolls: Vec<usize> = (0..dice.amount).map(|_| range.ind_sample(&mut rng)).collect();
    let result = rolls.iter().sum();
    RollResult {
      rolls: rolls,
      result: result
    }
  }
}
