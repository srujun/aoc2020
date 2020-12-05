use std::fs;

use clap::{App, Arg};
use colored::*;

use aoc2020::day01::DayOne;
use aoc2020::day02::DayTwo;
use aoc2020::problem::Problem;

fn main() -> Result<(), String> {
  let matches = App::new("AoC 2020")
    .arg(Arg::with_name("debug").short("d").long("debug"))
    .arg(Arg::with_name("DAY").index(1))
    .get_matches();
  let debug = matches.is_present("debug");

  match matches.value_of("DAY") {
    Some(day) => {
      print_problem(day.parse::<usize>().expect("Invalid day input!"), debug)?;
    }
    None => {
      for day in 1..25 {
        if print_problem(day, debug).is_err() {
          break;
        }
        println!();
      }
    }
  }

  Ok(())
}

fn print_problem(day: usize, debug: bool) -> Result<(), String> {
  let day_str = format!("{:02}", day);

  let problem =
    get_problem(day, debug).ok_or_else(|| format!("Day {} not implemented!", day_str))?;
  let input = fs::read_to_string(format!("inputs/day{}.txt", day_str)).unwrap();

  println!("{}", format!("DAY {}", day_str).blue().bold());
  print_part(1, &problem.soln_one(), &problem.part_one(&input));
  print_part(2, &problem.soln_two(), &problem.part_two(&input));

  Ok(())
}

fn print_part(num: usize, expected: &Option<String>, actual: &Option<String>) {
  println!(
    "Part {}: (expected answer: {})",
    num,
    expected.as_ref().unwrap_or(&"unknown".to_string()).bold()
  );
  println!(
    "Actual: {} {}",
    actual
      .as_ref()
      .unwrap_or(&"unimplemented".to_string())
      .bold(),
    result(&expected, &actual)
  );
}

fn result(expected: &Option<String>, actual: &Option<String>) -> String {
  if expected.is_none() {
    "??".yellow().to_string()
  } else if expected == actual {
    "✓".green().to_string()
  } else {
    "✗".red().to_string()
  }
}

fn get_problem(day: usize, debug: bool) -> Option<Box<dyn Problem>> {
  if debug {
    match day {
      1 => Some(Box::new(DayOne::debug())),
      2 => Some(Box::new(DayTwo::debug())),
      _ => None,
    }
  } else {
    match day {
      1 => Some(Box::new(DayOne::new())),
      2 => Some(Box::new(DayTwo::new())),
      _ => None,
    }
  }
}
