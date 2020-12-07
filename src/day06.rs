use crate::problem::Problem;

use std::collections::HashSet;

#[derive(Default)]
pub struct DaySix {}

impl DaySix {
  fn get_uniq_count(lines: &str) -> usize {
    let set: HashSet<char> = lines.chars().filter(|c| *c != '\n').collect();
    set.len()
  }

  fn get_common_count(lines: &str) -> usize {
    fn intersection(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
      a.into_iter().filter(|e| b.contains(e)).collect()
    }

    let mut map = lines
      .split("\n")
      .filter(|line| !line.is_empty())
      .map(|line| line.chars().collect());
    let set: HashSet<char> = map.next().unwrap();
    let set = map.fold(set, |a, b| intersection(a, b));

    set.len()
  }
}

impl Problem for DaySix {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(6878.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let total: usize = input
      .split("\n\n")
      .map(|group| Self::get_uniq_count(group))
      .sum();
    Some(total.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(3464.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let total: usize = input
      .split("\n\n")
      .map(|group| Self::get_common_count(group))
      .sum();
    Some(total.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::day06::DaySix;

  #[test]
  fn get_uniq_count() {
    assert_eq!(DaySix::get_uniq_count("abc"), 3);
    assert_eq!(DaySix::get_uniq_count("a\nb\nc\n"), 3);
    assert_eq!(DaySix::get_uniq_count("ab\nac"), 3);
    assert_eq!(DaySix::get_uniq_count("a\na\na\na\n"), 1);
    assert_eq!(DaySix::get_uniq_count("b\n"), 1);
  }
}
