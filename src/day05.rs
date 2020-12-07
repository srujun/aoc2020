use crate::problem::Problem;

use std::collections::HashSet;

#[derive(Default)]
pub struct DayFive {}

impl DayFive {
  fn seat_id(seat: &str) -> u16 {
    let binary: String = seat
      .chars()
      .map(|letter| match letter {
        'F' => '0',
        'B' => '1',
        'L' => '0',
        'R' => '1',
        _ => panic!("Unexpected char found!"),
      })
      .collect();

    u16::from_str_radix(&binary, 2).unwrap()
  }
}

impl Problem for DayFive {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(935.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let max = input
      .split('\n')
      .filter(|line| !line.is_empty())
      .map(|seat| DayFive::seat_id(seat))
      .max()
      .unwrap();
    Some(max.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(743.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let seat_ids: HashSet<u16> = input
      .split('\n')
      .filter(|line| !line.is_empty())
      .map(|seat| DayFive::seat_id(seat))
      .collect();

    let my_seat_id = (0..1023)
      .filter(|id| {
        !seat_ids.contains(id) && seat_ids.contains(&(id + 1)) && seat_ids.contains(&(id - 1))
      })
      .next();
    my_seat_id.map(|id| id.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::day05::DayFive;

  #[test]
  fn seat_id() {
    assert_eq!(DayFive::seat_id("BFFFBBFRRR"), 567);
    assert_eq!(DayFive::seat_id("FFFBBBFRRR"), 119);
    assert_eq!(DayFive::seat_id("BBFFBBFRLL"), 820);
  }
}
