use rolls::dice::Dice;
use std::convert::{TryFrom, TryInto};

#[test]
fn test_parse() {
  // We expect "2d4" to result in 2 dice with 4 faces each.
  let expect = Dice {
    amount: 2,
    faces: 4
  };
  assert_eq!(expect, Dice::try_from("2d4").unwrap());
}

#[test]
#[should_panic]
fn test_invalid_parse() {
  // "1d2d4" is an invalid dice string, so this should panic.
  Dice::try_from("1d2d4").unwrap();
}

#[test]
fn test_d_first_parse() {
  // Omitting the amount of dice should assume one, so expect one six-sided die.
  let expect = Dice {
    amount: 1,
    faces: 6
  };
  assert_eq!(expect, Dice::try_from("d6").unwrap());
}

#[test]
#[should_panic]
fn test_invalid_order_parse() {
  // "6d" is missing the faces part of the dice string, which is invalid, so this should panic.
  Dice::try_from("6d").unwrap();
}

#[test]
fn test_uppercase() {
  // This is a valid dice string, but it uses an uppercase D.
  let dice_string = "4D10";
  // Ensure we're testing a string with an uppercase letter (stupid-proofing).
  assert!(dice_string.chars().any(|c| c.is_uppercase()));
  // We should find four ten-sided dice.
  let expect = Dice {
    amount: 4,
    faces: 10
  };
  assert_eq!(expect, dice_string.try_into().unwrap());
}

#[test]
#[should_panic]
fn test_zero_faces() {
  Dice::try_from("1d0").unwrap();
}
