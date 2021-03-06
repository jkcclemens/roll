use rolls::{Roller, Rolls};
use rolls::dice::Dice;
use std::convert::TryInto;

/// Unsafely unwraps the `Result<Dice>` generated by `string`. Ensure input is valid before using.
fn get_dice(string: &str) -> Dice {
  string.try_into().unwrap()
}

#[test]
fn test_1d6() {
  // Get a six-sided die.
  let a_1d6 = get_dice("1d6");
  // Roll it 100 times.
  for _ in 0..100 {
    let result = Roller.roll(&a_1d6);
    // Each roll should only have one roll in the result.
    assert_eq!(1, result.rolls.len());
    // The single roll should be between [1, 6] every time.
    assert!(result.result >= 1 && result.result <= 6);
  }
}

#[test]
fn test_3d10() {
  // Get three ten-sided dice.
  let dice = get_dice("3d10");
  // Roll them 100 times.
  for _ in 0..100 {
    let result = Roller.roll(&dice);
    // Each roll should have three dice rolls in it.
    assert_eq!(3, result.rolls.len());
    // The sum of the rolls should equal the result field.
    assert_eq!(result.rolls.iter().sum::<usize>(), result.result);
    // The result should be between [3, 30] every time.
    assert!(result.result >= 3 && result.result <= 30);
  }
}
