use crate::problem::Problem;

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

const TARGET_BAG: &'static str = "shiny gold";

lazy_static! {
  static ref FULL: Regex = Regex::new(r"^(\w+ \w+) bags? contain (.+)\.$").unwrap();
  static ref PAIR: Regex = Regex::new(r"^(\d+) (\w+ \w+) bags?$").unwrap();
  static ref TERMINAL: Regex = Regex::new(r"^no other bags$").unwrap();
}

struct Traverser<'a> {
  // bag -> (child_bag, count of child_bag)
  graph: HashMap<&'a str, HashMap<&'a str, usize>>,
  // bag -> whether bag contains TARGET_BAG in descendants
  contains_target: HashMap<&'a str, bool>,
  // bag -> how many total descendent bags contained in bag
  contained_bags: HashMap<&'a str, usize>,
}

impl<'a> Traverser<'a> {
  fn new(lines: Vec<&'a str>) -> Self {
    Self {
      graph: lines.iter().map(|line| Self::parse_line(line)).collect(),
      contains_target: HashMap::new(),
      contained_bags: HashMap::new(),
    }
  }

  fn parse_line(line: &str) -> (&str, HashMap<&str, usize>) {
    let caps = FULL.captures(line).unwrap();
    let key = caps.get(1).unwrap().as_str();
    let bags = caps.get(2).unwrap().as_str();

    let mut map = HashMap::new();
    if TERMINAL.is_match(bags) {
      return (key, map);
    }

    for pair in bags.split(", ") {
      let pair_caps = PAIR
        .captures(pair)
        .expect(&format!("Failed match on {}", pair));
      let num = pair_caps.get(1).unwrap().as_str().parse().unwrap();
      let bag = pair_caps.get(2).unwrap().as_str();
      map.insert(bag, num);
    }
    (key, map)
  }

  fn count_carriers(&mut self) -> usize {
    let mut count = 0;
    let all_bags: Vec<&str> = self.graph.keys().copied().collect();
    for bag in all_bags {
      if self.is_carrier(TARGET_BAG, bag) {
        count += 1;
      }
    }
    count
  }

  fn is_carrier(&mut self, target_bag: &str, bag: &'a str) -> bool {
    match self.contains_target.get(bag) {
      Some(does_contain) => {
        // We have already traversed this bag's children.
        *does_contain
      }
      None => {
        let mut result = false;
        let children: Vec<&str> = self.graph[bag].keys().copied().collect();
        for child in children {
          result |= (child == target_bag) || self.is_carrier(target_bag, child);
        }
        self.contains_target.insert(bag, result);
        result
      }
    }
  }

  fn count_contained_bags(&mut self, bag: &'a str) -> usize {
    match self.contained_bags.get(bag) {
      Some(count) => *count,
      None => {
        let mut total = 1;
        let children: Vec<(&str, usize)> = self.graph[bag].iter().map(|(k, v)| (*k, *v)).collect();
        for (child, num) in children {
          total += num * self.count_contained_bags(child);
        }
        self.contained_bags.insert(bag, total);
        total
      }
    }
  }
}

#[derive(Default)]
pub struct DaySeven {}

impl DaySeven {}

impl Problem for DaySeven {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(126.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();
    let mut traverser = Traverser::new(lines);

    let num_carriers = traverser.count_carriers();

    Some(num_carriers.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(220149.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let lines: Vec<&str> = input.split("\n").filter(|line| !line.is_empty()).collect();
    let mut traverser = Traverser::new(lines);

    // -1 because we don't want to count the target bag itself.
    let total_contained_bags = traverser.count_contained_bags(TARGET_BAG) - 1;

    Some(total_contained_bags.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::day07::DaySeven;
  use crate::problem::Problem;

  #[test]
  fn part_two_given() {
    let problem = DaySeven::new();
    let input = "shiny gold bags contain 2 dark red bags.\n\
                 dark red bags contain 2 dark orange bags.\n\
                 dark orange bags contain 2 dark yellow bags.\n\
                 dark yellow bags contain 2 dark green bags.\n\
                 dark green bags contain 2 dark blue bags.\n\
                 dark blue bags contain 2 dark violet bags.\n\
                 dark violet bags contain no other bags.\n";
    assert_eq!(problem.part_two(input).unwrap(), "126".to_string());
  }
}
