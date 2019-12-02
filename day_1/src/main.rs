// extern crate rayon;

use std::fs;
// use rayon::prelude::*;

// Calculates fuel use for a single mass
fn calc_single_fuel(mass: f64) -> f64 {
  let mut current_fuel = 0.0;
  let mut current_mass = mass;

  while current_mass != 0.0 {
    let fuel_required = (current_mass / 3.0).floor() - 2.0;
    if fuel_required >= 0.0 {
      current_fuel += fuel_required;
      current_mass = fuel_required
    } else {
      // println!("{:?}", fuel_required);
      current_mass = 0.0
      // break
    }
    // println!("{:?}", current_fuel);
  }
  // println!("{:?}", current_fuel);
  current_fuel
}

// Calculates and sums up total fuel use
fn calc_fuel(sum: f64, element: &&str) -> f64 {
  calc_single_fuel(element.parse::<i64>().unwrap() as f64) + sum
}
fn main() {
  let contents = fs::read_to_string("input.txt").unwrap();
  let masses: Vec<&str> = contents.split("\n").collect();
  let total: f64 = masses.iter().fold(0.0, calc_fuel);
  println!("{:?}", total as i64);
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_calc_single_fuel() {
    assert_eq!(calc_single_fuel(14.0), 2.0);
  }
}
