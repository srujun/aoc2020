use crate::problem::Problem;

use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

const KNOWN_FIELDS: usize = 8;

lazy_static! {
  static ref HEIGHT: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>(in|cm)?)$").unwrap();
  static ref HAIR_COLOR: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
  static ref PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
  static ref EYE_COLORS: HashSet<&'static str> = {
    let mut set = HashSet::new();
    set.insert("amb");
    set.insert("blu");
    set.insert("brn");
    set.insert("gry");
    set.insert("grn");
    set.insert("hzl");
    set.insert("oth");
    set
  };
}

#[derive(Default)]
pub struct DayFour {}

impl DayFour {
  fn is_valid_p1(&self, lines: &str) -> bool {
    let mut fields: HashSet<&str> = HashSet::new();

    for line in lines.split('\n') {
      for detail in line.split(' ') {
        let field = detail.split(':').nth(0).unwrap();
        let new_val = fields.insert(field);
        if !new_val {
          // Two entries for the same field!?
          return false;
        }
      }
    }

    fields.len() == KNOWN_FIELDS || (fields.len() == KNOWN_FIELDS - 1 && !fields.contains("cid"))
  }

  fn is_valid_p2(&self, lines: &str) -> bool {
    let mut fields: HashSet<&str> = HashSet::new();

    for line in lines.split('\n') {
      if line.is_empty() {
        continue;
      }
      for detail in line.split(' ') {
        let mut pairs = detail.split(':');
        let field = pairs.next().expect(&format!("No field for {}", detail));
        let value = pairs.next().expect(&format!("No value for {}", detail));

        if !Self::is_valid_pair(field, value) {
          return false;
        }

        let new_val = fields.insert(field);
        if !new_val {
          // Two entries for the same field!?
          return false;
        }
      }
    }

    fields.len() == KNOWN_FIELDS || (fields.len() == KNOWN_FIELDS - 1 && !fields.contains("cid"))
  }

  fn is_valid_pair(key: &str, val: &str) -> bool {
    match key {
      "byr" => {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let year: u32 = val.parse().unwrap();
        return year >= 1920 && year <= 2002;
      }
      "iyr" => {
        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let year: u32 = val.parse().unwrap();
        return year >= 2010 && year <= 2020;
      }
      "eyr" => {
        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let year: u32 = val.parse().unwrap();
        return year >= 2020 && year <= 2030;
      }
      "hgt" => {
        // hgt (Height) - a number followed by either cm or in:
        //  - If cm, the number must be at least 150 and at most 193.
        //  - If in, the number must be at least 59 and at most 76.
        let caps = HEIGHT
          .captures(val)
          .expect(&format!("No height match for {}", val));
        let num: i32 = caps["num"].parse().unwrap();
        match &caps["unit"] {
          "cm" => return num >= 150 && num <= 193,
          "in" => return num >= 59 && num <= 76,
          _ => return false,
        }
      }
      "hcl" => {
        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        return HAIR_COLOR.is_match(val);
      }
      "ecl" => {
        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        return EYE_COLORS.contains(val);
      }
      "pid" => {
        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        return PASSPORT_ID.is_match(val);
      }
      "cid" => {
        // cid (Country ID) - ignored, missing or not.
        return true;
      }
      _ => return false,
    }
  }
}

impl Problem for DayFour {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(264.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let all_details = input.split("\n\n").filter(|line| !line.is_empty());

    let count: usize = all_details
      .map(|details| self.is_valid_p1(details))
      .filter(|&validity| validity == true)
      .count();
    Some(count.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(224.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let all_details = input.split("\n\n").filter(|line| !line.is_empty());

    let count: usize = all_details
      .map(|details| self.is_valid_p2(details))
      .filter(|&validity| validity == true)
      .count();
    Some(count.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::day04::DayFour;

  #[test]
  fn byr() {
    assert_eq!(DayFour::is_valid_pair("byr", "2002"), true);
    assert_eq!(DayFour::is_valid_pair("byr", "2003"), false);
  }

  #[test]
  fn hgt() {
    assert_eq!(DayFour::is_valid_pair("hgt", "60in"), true);
    assert_eq!(DayFour::is_valid_pair("hgt", "190cm"), true);
    assert_eq!(DayFour::is_valid_pair("hgt", "190in"), false);
    assert_eq!(DayFour::is_valid_pair("hgt", "190"), false);
  }

  #[test]
  fn hcl() {
    assert_eq!(DayFour::is_valid_pair("hcl", "#123abc"), true);
    assert_eq!(DayFour::is_valid_pair("hcl", "#123abz"), false);
    assert_eq!(DayFour::is_valid_pair("hcl", "123abc"), false);
  }

  #[test]
  fn ecl() {
    assert_eq!(DayFour::is_valid_pair("ecl", "brn"), true);
    assert_eq!(DayFour::is_valid_pair("ecl", "wat"), false);
  }

  #[test]
  fn pid() {
    assert_eq!(DayFour::is_valid_pair("pid", "000000001"), true);
    assert_eq!(DayFour::is_valid_pair("pid", "0123456789"), false);
  }
}
