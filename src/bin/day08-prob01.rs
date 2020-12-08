#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::collections::HashSet;

fn main() {
    let program = std::fs::read_to_string("src/bin/day08.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Instruction::from_asm(val))
                .collect::<Vec<Instruction>>()
        })
        .expect("Unable to open file");
    println!("{:?}", run_program(program));
}

fn run_program(program: Vec<Instruction>) -> i32 {
    let mut execed_insts: HashSet<usize> = HashSet::new();
    let mut pc: usize = 0;
    let mut acc: i32 = 0;
    while !execed_insts.contains(&pc) {
        execed_insts.insert(pc);
        // println!("pc:{:?} acc:{:?} {:?}", pc, acc, program[pc]);
        match program[pc] {
            Instruction::Acc(val) => {
                acc += val;
                pc += 1;
            }
            Instruction::Jmp(val) => pc = (val + pc as i32) as usize,
            Instruction::Nop => pc += 1,
        }
    }
    acc
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl Instruction {
    fn parse_i32(raw: &str) -> i32 {
        raw.parse::<i32>()
            .expect(&format!("Unable to parse i32 from {}", raw))
    }

    pub fn from_asm(line: &str) -> Instruction {
        let pieces = line.split(" ").collect::<Vec<&str>>();
        let cmd = pieces[0];
        match cmd {
            "acc" => Instruction::Acc(Instruction::parse_i32(pieces[1])),
            "jmp" => Instruction::Jmp(Instruction::parse_i32(pieces[1])),
            "nop" => Instruction::Nop,
            val => panic!("Unknown instruction: {}", val),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from_asm_acc_positive() {
        assert_eq!(Instruction::from_asm("acc +3"), Instruction::Acc(3))
    }

    #[test]
    fn test_instruction_from_asm_acc_zero() {
        assert_eq!(Instruction::from_asm("acc +0"), Instruction::Acc(0))
    }

    #[test]
    fn test_instruction_from_asm_acc_negative() {
        assert_eq!(Instruction::from_asm("acc -3"), Instruction::Acc(-3))
    }

    #[test]
    fn test_instruction_from_asm_jmp_positive() {
        assert_eq!(Instruction::from_asm("jmp +3"), Instruction::Jmp(3))
    }

    #[test]
    fn test_instruction_from_asm_jmp_zero() {
        assert_eq!(Instruction::from_asm("jmp +0"), Instruction::Jmp(0))
    }

    #[test]
    fn test_instruction_from_asm_jmp_negative() {
        assert_eq!(Instruction::from_asm("jmp -3"), Instruction::Jmp(-3))
    }

    #[test]
    fn test_instruction_from_asm_nop() {
        assert_eq!(Instruction::from_asm("nop +3"), Instruction::Nop)
    }

    #[test]
    fn test_run_program() {
        assert_eq!(
            run_program(vec![
                Instruction::Nop,
                Instruction::Acc(1),
                Instruction::Jmp(4),
                Instruction::Acc(3),
                Instruction::Jmp(-3),
                Instruction::Acc(-99),
                Instruction::Acc(1),
                Instruction::Jmp(-4),
                Instruction::Acc(6)
            ]),
            5
        )
    }
}
