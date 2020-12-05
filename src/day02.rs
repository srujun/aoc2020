use crate::problem::Problem;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
  static ref LINE_PATTERN: Regex =
    Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>[a-z]): (?P<pass>[a-z]+)$").unwrap();
}

#[derive(Default)]
pub struct DayTwo {}

impl DayTwo {
  fn is_valid_password_p1(line: &str) -> bool {
    let caps = LINE_PATTERN.captures(line).unwrap();
    let min: usize = caps["min"].parse().unwrap();
    let max: usize = caps["max"].parse().unwrap();
    let letter: char = caps["letter"].parse().unwrap();
    let password = &caps["pass"];

    let count = password.matches(letter).count();
    count >= min && count <= max
  }

  fn is_valid_password_p2(line: &str) -> bool {
    let caps = LINE_PATTERN.captures(line).unwrap();
    let min: usize = caps["min"].parse().unwrap();
    let max: usize = caps["max"].parse().unwrap();
    let letter: char = caps["letter"].parse().unwrap();
    let password = &caps["pass"];

    (password.chars().nth(min - 1).unwrap() == letter)
      ^ (password.chars().nth(max - 1).unwrap() == letter)
  }
}

impl Problem for DayTwo {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(628.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split('\n').collect();

    let count: usize = lines
      .iter()
      .filter(|line| !line.is_empty())
      .map(|line| DayTwo::is_valid_password_p1(line))
      .filter(|&validity| validity == true)
      .count();
    Some(count.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(705.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split('\n').collect();

    let count: usize = lines
      .iter()
      .filter(|line| !line.is_empty())
      .map(|line| DayTwo::is_valid_password_p2(line))
      .filter(|&validity| validity == true)
      .count();
    Some(count.to_string())
  }
}
