use crate::problem::Problem;

use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Operation {
  ACC,
  JMP,
  NOP,
}

#[derive(Clone, Copy, Debug)]
struct Instruction(Operation, i32);

#[derive(Default)]
pub struct DayEight {}

impl DayEight {
  fn parse_instruction(line: &str) -> Instruction {
    let mut parts = line.split(' ');
    let operation = match parts.next().unwrap() {
      "acc" => Operation::ACC,
      "jmp" => Operation::JMP,
      "nop" => Operation::NOP,
      _ => panic!("Cannot parse {}", line),
    };
    let argument = parts
      .next()
      .unwrap()
      .parse()
      .expect(&format!("No argument in {}", line));
    Instruction(operation, argument)
  }

  fn execute(instruction: &Instruction, ipr: &mut usize, acc: &mut i32) {
    match instruction.0 {
      Operation::ACC => {
        *acc += instruction.1;
        *ipr += 1;
      }
      Operation::JMP => {
        *ipr = ipr.wrapping_add(instruction.1 as usize);
      }
      Operation::NOP => {
        *ipr += 1;
      }
    }
  }

  fn acc_before_duplicate(program: &Vec<Instruction>) -> i32 {
    let mut iprs_seen: HashSet<usize> = HashSet::new();
    let mut ipr: usize = 0;
    let mut acc: i32 = 0;

    while !iprs_seen.contains(&ipr) {
      let instruction = &program[ipr];
      iprs_seen.insert(ipr);
      Self::execute(instruction, &mut ipr, &mut acc);
    }

    acc
  }

  fn acc_after_flip(program: &Vec<Instruction>) -> i32 {
    let mut program_copy = program.to_vec();
    let mut iprs_seen: HashSet<usize> = HashSet::new();
    let mut ipr: usize = 0;
    let mut acc: i32 = 0;

    while !iprs_seen.contains(&ipr) {
      let instruction: Instruction = program_copy[ipr];

      // Try flipping and running if possible.
      match instruction.0 {
        Operation::ACC => {
          // No flip, continue as normal.
        }
        Operation::JMP => {
          program_copy[ipr] = Instruction(Operation::NOP, instruction.1);
          if let Some(final_acc) = Self::does_terminate(&program_copy, iprs_seen.clone(), ipr, acc)
          {
            return final_acc;
          }
          // Didn't work, let's flip back.
          program_copy[ipr] = instruction;
        }
        Operation::NOP => {
          program_copy[ipr] = Instruction(Operation::JMP, instruction.1);
          if let Some(final_acc) = Self::does_terminate(&program_copy, iprs_seen.clone(), ipr, acc)
          {
            return final_acc;
          }
          // Didn't work, let's flip back.
          program_copy[ipr] = instruction;
        }
      };

      // Execute instruction as normal.
      iprs_seen.insert(ipr);
      Self::execute(&instruction, &mut ipr, &mut acc);
    }

    panic!("Could not find a flip to terminate the program!");
  }

  /// Some(acc) if program terminates, None if it doesn't
  /// Call this after flipping NOP <-> JMP in program
  fn does_terminate(
    program: &Vec<Instruction>,
    mut iprs_seen: HashSet<usize>,
    mut ipr: usize,
    mut acc: i32,
  ) -> Option<i32> {
    while !iprs_seen.contains(&ipr) {
      if ipr == program.len() {
        // Attempting to run instruction after the last one, we TERMINATED!
        return Some(acc);
      }

      let instruction = &program[ipr];
      iprs_seen.insert(ipr);
      Self::execute(instruction, &mut ipr, &mut acc);
    }

    // We found a duplicate instruction :(
    None
  }
}

impl Problem for DayEight {
  fn new() -> Self {
    Self {}
  }

  fn soln_one(&self) -> Option<String> {
    Some(1548.to_string())
  }

  fn part_one(&self, input: &str) -> Option<String> {
    let instructions: Vec<Instruction> = input
      .lines()
      .map(|line| DayEight::parse_instruction(line))
      .collect();

    let acc = DayEight::acc_before_duplicate(&instructions);
    Some(acc.to_string())
  }

  fn soln_two(&self) -> Option<String> {
    Some(1375.to_string())
  }

  fn part_two(&self, input: &str) -> Option<String> {
    let instructions: Vec<Instruction> = input
      .lines()
      .map(|line| DayEight::parse_instruction(line))
      .collect();

    let acc = DayEight::acc_after_flip(&instructions);
    Some(acc.to_string())
  }
}

#[cfg(test)]
mod tests {
  use crate::day08::DayEight;
  use crate::problem::Problem;

  #[test]
  fn part_one_given() {
    let problem = DayEight::new();
    let input = "nop +0\n\
                 acc +1\n\
                 jmp +4\n\
                 acc +3\n\
                 jmp -3\n\
                 acc -99\n\
                 acc +1\n\
                 jmp -4\n\
                 acc +6\n";
    assert_eq!(problem.part_one(input).unwrap(), "5".to_string());
  }

  #[test]
  fn part_two_given() {
    let problem = DayEight::new();
    let input = "nop +0\n\
                 acc +1\n\
                 jmp +4\n\
                 acc +3\n\
                 jmp -3\n\
                 acc -99\n\
                 acc +1\n\
                 jmp -4\n\
                 acc +6\n";
    assert_eq!(problem.part_two(input).unwrap(), "8".to_string());
  }
}
