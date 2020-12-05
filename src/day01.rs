use std::collections::HashMap;

use crate::problem::Problem;

#[derive(Default)]
pub struct DayOne {}

impl DayOne {
  /// Returns the indices of the two numbers in `nums` that add up to `target`
  fn two_sum(nums: &Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut complements = HashMap::with_capacity(nums.len());
    for (idx, num) in nums.iter().enumerate() {
      let complement = target - num;
      if let Some(c_idx) = complements.get(&complement) {
        return Some((idx, *c_idx));
      }
      complements.insert(num, idx);
    }
    // No result found (should not happen)
    None
  }

  /// Returns the indices of the three numbers in `nums` that add up to `target`
  fn three_sum(nums: &Vec<i32>, target: i32) -> Option<(usize, usize, usize)> {
    // Calculate two_sum around a pivot
    let two_sum_x = |pivot_idx: usize, target_x: i32| -> Option<(usize, usize)> {
      let mut complements = HashMap::with_capacity(nums.len());
      for (idx, num) in nums.iter().enumerate() {
        if idx == pivot_idx {
          // Skip the pivot element
          continue;
        }
        let complement = target_x - num;
        if let Some(c_idx) = complements.get(&complement) {
          return Some((idx, *c_idx));
        }
        complements.insert(num, idx);
      }
      None
    };

    for (pivot_idx, pivot) in nums.iter().enumerate() {
      if let Some(tuple) = two_sum_x(pivot_idx, target - pivot) {
        return Some((pivot_idx, tuple.0, tuple.1));
      }
    }

    // No result found (should not happen)
    None
  }
}

impl Problem for DayOne {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(646779.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let target = 2020;
    let nums: Vec<i32> = input
      .split('\n')
      .map(|s| s.parse())
      .filter_map(Result::ok)
      .collect();

    let indices = Self::two_sum(&nums, target);
    indices
      .map(|(i, j)| nums[i] * nums[j])
      .map(|val| val.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(246191688.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let target = 2020;
    let nums: Vec<i32> = input
      .split('\n')
      .map(|s| s.parse())
      .filter_map(Result::ok)
      .collect();

    let indices = Self::three_sum(&nums, target);
    indices
      .map(|(i, j, k)| nums[i] * nums[j] * nums[k])
      .map(|val| val.to_string())
  }
}

#[cfg(test)]
mod tests {}
