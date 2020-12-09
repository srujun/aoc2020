use crate::problem::Problem;

use std::collections::HashSet;

#[derive(Default)]
pub struct DayNine {}

impl DayNine {
  fn find_invalid_naive(nums: &Vec<i64>, preamble: usize) -> Option<&i64> {
    // Checks if prev_nums has any two nums that add up to target.
    let has_complements = |prev_nums: &[i64], target: &i64| -> bool {
      let mut complements: HashSet<i64> = HashSet::with_capacity(preamble);
      for n in prev_nums {
        if complements.contains(n) {
          // Sum exists!
          return true;
        } else {
          complements.insert(target - n);
        }
      }
      // No complement exists...
      false
    };

    for (idx, num) in nums.iter().enumerate().skip(preamble) {
      if !has_complements(&nums[(idx - preamble)..idx], num) {
        return Some(num);
      }
    }
    // No invalid number found...
    None
  }

  /// Returns the start and end indices (inclusive) of a contiguous set of elements that sum up to
  /// target.
  fn find_contiguous(nums: &Vec<i64>, target: &i64) -> (usize, usize) {
    let (mut start, mut end) = (0, 1);
    let mut curr_sum = nums[start] + nums[end];

    while end < nums.len() {
      if start >= end {
        panic!("Invalid state: start={} end={}", start, end);
      }

      if curr_sum == *target {
        return (start, end);
      } else if curr_sum < *target {
        end += 1;
        curr_sum += nums[end];
      } else {
        curr_sum -= nums[start];
        start += 1;
      }
    }

    panic!("No contiguous set found!");
  }
}

impl Problem for DayNine {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(1504371145.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    Self::find_invalid_naive(&nums, 25).map(|num| num.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(183278487.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let nums: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let target: &i64 = Self::find_invalid_naive(&nums, 25).unwrap();

    let (start_idx, end_idx) = Self::find_contiguous(&nums, target);
    let answer = nums[start_idx..end_idx].iter().max().unwrap()
      + nums[start_idx..end_idx].iter().min().unwrap();

    Some(answer.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::day09::DayNine;

  #[test]
  fn find_invalid_naive_given() {
    let nums: Vec<i64> = vec![
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(DayNine::find_invalid_naive(&nums, 5), Some(&127));
  }

  #[test]
  fn find_contiguous_given() {
    let nums: Vec<i64> = vec![
      35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(DayNine::find_contiguous(&nums, &127), (2, 5));
  }
}
