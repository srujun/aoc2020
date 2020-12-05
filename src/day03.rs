use crate::problem::Problem;

#[derive(Default)]
pub struct DayThree {
  debug: bool,
}

impl DayThree {
  fn count_trees(&self, map: &Vec<Vec<char>>, slope: (usize, usize)) -> u32 {
    const TREE: char = '#';
    let width = map[0].len();
    let height = map.len();

    let mut num_trees = 0;
    let mut curr_x = slope.0;
    let mut curr_y = slope.1;

    while curr_y < height {
      // analyze obj at position
      let obj = map[curr_y][curr_x];
      if obj == TREE {
        if self.debug {
          println!("Found tree at ({}, {})", curr_x, curr_y);
        }
        num_trees += 1;
      }

      // move
      curr_x += slope.0;
      curr_y += slope.1;

      // correct curr_x
      curr_x %= width;
    }

    num_trees
  }
}

impl Problem for DayThree {
  fn new() -> Self {
    Self { debug: false }
  }

  fn debug() -> Self {
    Self { debug: true }
  }

  fn soln_one(&self) -> Option<String> {
    Some(268.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let map: Vec<Vec<char>> = input
      .split('\n')
      .filter(|line| !line.is_empty())
      .map(|line| line.chars().collect())
      .collect();

    const SLOPE: (usize, usize) = (3, 1);
    Some(self.count_trees(&map, SLOPE).to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(3093068400_u32.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let map: Vec<Vec<char>> = input
      .split('\n')
      .filter(|line| !line.is_empty())
      .map(|line| line.chars().collect())
      .collect();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result = slopes
      .iter()
      .map(|&slope| self.count_trees(&map, slope))
      .fold(1, |a, b| a * b);

    Some(result.to_string())
  }
}
