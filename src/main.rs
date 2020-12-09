use std::fs;

use clap::{App, Arg};
use colored::*;

use aoc2020::day01::DayOne;
use aoc2020::day02::DayTwo;
use aoc2020::day03::DayThree;
use aoc2020::day04::DayFour;
use aoc2020::day05::DayFive;
use aoc2020::day06::DaySix;
use aoc2020::day07::DaySeven;
use aoc2020::day08::DayEight;
use aoc2020::day09::DayNine;
// use aoc2020::day10::DayTen;
// use aoc2020::day11::DayEleven;
// use aoc2020::day12::DayTwelve;
// use aoc2020::day13::DayThirteen;
// use aoc2020::day14::DayFourteen;
// use aoc2020::day15::DayFifteen;
// use aoc2020::day16::DaySixteen;
// use aoc2020::day17::DaySeventeen;
// use aoc2020::day18::DayEighteen;
// use aoc2020::day19::DayNineteen;
// use aoc2020::day20::DayTwenty;
// use aoc2020::day21::DayTwentyOne;
// use aoc2020::day22::DayTwentyTwo;
// use aoc2020::day23::DayTwentyThree;
// use aoc2020::day24::DayTwentyFour;
// use aoc2020::day25::DayTwentyFive;
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
      3 => Some(Box::new(DayThree::debug())),
      4 => Some(Box::new(DayFour::debug())),
      5 => Some(Box::new(DayFive::debug())),
      6 => Some(Box::new(DaySix::debug())),
      7 => Some(Box::new(DaySeven::debug())),
      8 => Some(Box::new(DayEight::debug())),
      9 => Some(Box::new(DayNine::debug())),
      // 10 => Some(Box::new(DayTen::debug())),
      // 11 => Some(Box::new(DayEleven::debug())),
      // 12 => Some(Box::new(DayTwelve::debug())),
      // 13 => Some(Box::new(DayThirteen::debug())),
      // 14 => Some(Box::new(DayFourteen::debug())),
      // 15 => Some(Box::new(DayFifteen::debug())),
      // 16 => Some(Box::new(DaySixteen::debug())),
      // 17 => Some(Box::new(DaySeventeen::debug())),
      // 18 => Some(Box::new(DayEighteen::debug())),
      // 19 => Some(Box::new(DayNineteen::debug())),
      // 20 => Some(Box::new(DayTwenty::debug())),
      // 21 => Some(Box::new(DayTwentyOne::debug())),
      // 22 => Some(Box::new(DayTwentyTwo::debug())),
      // 23 => Some(Box::new(DayTwentyThree::debug())),
      // 24 => Some(Box::new(DayTwentyFour::debug())),
      // 25 => Some(Box::new(DayTwentyFive::debug())),
      _ => None,
    }
  } else {
    match day {
      1 => Some(Box::new(DayOne::new())),
      2 => Some(Box::new(DayTwo::new())),
      3 => Some(Box::new(DayThree::new())),
      4 => Some(Box::new(DayFour::new())),
      5 => Some(Box::new(DayFive::new())),
      6 => Some(Box::new(DaySix::new())),
      7 => Some(Box::new(DaySeven::new())),
      8 => Some(Box::new(DayEight::new())),
      9 => Some(Box::new(DayNine::new())),
      // 10 => Some(Box::new(DayTen::new())),
      // 11 => Some(Box::new(DayEleven::new())),
      // 12 => Some(Box::new(DayTwelve::new())),
      // 13 => Some(Box::new(DayThirteen::new())),
      // 14 => Some(Box::new(DayFourteen::new())),
      // 15 => Some(Box::new(DayFifteen::new())),
      // 16 => Some(Box::new(DaySixteen::new())),
      // 17 => Some(Box::new(DaySeventeen::new())),
      // 18 => Some(Box::new(DayEighteen::new())),
      // 19 => Some(Box::new(DayNineteen::new())),
      // 20 => Some(Box::new(DayTwenty::new())),
      // 21 => Some(Box::new(DayTwentyOne::new())),
      // 22 => Some(Box::new(DayTwentyTwo::new())),
      // 23 => Some(Box::new(DayTwentyThree::new())),
      // 24 => Some(Box::new(DayTwentyFour::new())),
      // 25 => Some(Box::new(DayTwentyFive::new())),
      _ => None,
    }
  }
}
