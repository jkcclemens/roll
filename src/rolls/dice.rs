use std::convert::TryFrom;
use super::super::errors::*;

/// A number of dice, all with the same amount of faces.
#[derive(Debug, PartialEq, Hash)]
pub struct Dice {
  /// The amount of dice in this group.
  pub amount: usize,
  /// The number of faces on each die in this group.
  pub faces: usize
}

impl<T> TryFrom<T> for Dice
  where T: AsRef<str>
{
  type Err = Error;

  /// Tries to convert a string of the format `(num)d[faces]`, where `num` is optional
  /// and `faces` is required, into some `Dice`.
  fn try_from(string: T) -> Result<Dice> {
    let string = string.as_ref().to_lowercase();
    let parts: Vec<_> = string.split('d').collect();
    if parts.len() != 2 {
      return Err("incorrect dice format".into());
    }
    let amount = if parts[0].is_empty() {
      1
    } else {
      parts[0].parse::<usize>().chain_err(|| "could not parse dice amount")?
    };
    let faces = parts[1].parse::<usize>().chain_err(|| "could not parse dice faces")?;
    Ok(Dice {
      amount: amount,
      faces: faces
    })
  }
}
