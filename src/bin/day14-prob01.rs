extern crate regex;

use regex::Regex;
use std::collections::HashMap;

fn main() {
    let instructions = std::fs::read_to_string("src/bin/day14.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Instruction::from_raw(val))
                .collect::<Vec<Instruction>>()
        })
        .expect("Unable to open file");
    println!("{:?}", Instruction::run(instructions));
}

#[derive(Debug, PartialEq)]
enum Instruction {
    MemSet(MemSet),
    Mask(Mask),
}

impl Instruction {
    pub fn from_raw(line: &str) -> Instruction {
        if line.starts_with("mem") {
            Instruction::MemSet(MemSet::from_raw(line))
        } else {
            Instruction::Mask(Mask::from_raw(line))
        }
    }

    pub fn run(instructions: Vec<Instruction>) -> u64 {
        let mut mask = Mask::new(0, std::u64::MAX);
        let mut mem: HashMap<usize, u64> = HashMap::new();

        for inst in instructions {
            match inst {
                Instruction::Mask(inst_mask) => mask = inst_mask,
                Instruction::MemSet(inst_memset) => {
                    mem.insert(inst_memset.addr, mask.apply(inst_memset.val));
                    ()
                }
            }
        }

        mem.values().sum()
    }
}

#[derive(Debug, PartialEq)]
struct MemSet {
    addr: usize,
    val: u64,
}

impl MemSet {
    pub fn new(addr: usize, val: u64) -> MemSet {
        MemSet { addr, val }
    }

    pub fn from_raw(line: &str) -> MemSet {
        let pattern = Regex::new(r"mem\[([0-9]+)\] = ([0-9]+)").expect("Invalid regex");
        let captures = pattern
            .captures(line)
            .expect(&format!("Unable to match line '{}'", line));

        let raw_addr = captures
            .get(1)
            .expect(&format!("Unable to match `addr`: {}", line))
            .as_str();
        let addr = raw_addr
            .parse::<usize>()
            .expect(&format!("Unable to parse `addr` as usize: {}", raw_addr));

        let raw_val = captures
            .get(2)
            .expect(&format!("Unable to match `val`: {}", line))
            .as_str();
        let val = raw_val
            .parse::<u64>()
            .expect(&format!("Unable to parse `val` as u64: {}", raw_val));

        MemSet::new(addr, val)
    }
}

#[derive(Debug, PartialEq)]
struct Mask {
    set_mask: u64,
    unset_mask: u64,
}

impl Mask {
    pub fn new(set_mask: u64, unset_mask: u64) -> Mask {
        Mask {
            set_mask,
            unset_mask,
        }
    }

    pub fn from_raw(line: &str) -> Mask {
        let raw_mask = line.strip_prefix("mask = ").expect(&format!(
            "Unable to find prefix `mask = ` for line {}",
            line
        ));

        let mut set_mask = 0u64;
        let mut unset_mask = std::u64::MAX;

        for (i, val) in raw_mask.chars().rev().enumerate() {
            match val {
                '0' => unset_mask &= !(2u64.pow(i as u32)),
                '1' => set_mask |= 2u64.pow(i as u32),
                'X' => (),
                a => panic!("Found unexpected char in mask: {}", a),
            }
        }

        Mask::new(set_mask, unset_mask)
    }

    pub fn apply(&self, val: u64) -> u64 {
        val & self.unset_mask | self.set_mask
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_from_raw_memset() {
        assert_eq!(
            Instruction::from_raw("mem[6] = 11"),
            Instruction::MemSet(MemSet::new(6, 11))
        )
    }

    #[test]
    fn test_instruction_from_raw_mask() {
        assert_eq!(
            Instruction::from_raw("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            Instruction::Mask(Mask::new(0b1000000u64, std::u64::MAX & !0b10u64))
        )
    }

    #[test]
    fn test_mask_apply1() {
        assert_eq!(
            Mask::new(0b1000000u64, std::u64::MAX & !0b10u64).apply(11),
            73
        )
    }

    #[test]
    fn test_mask_apply2() {
        assert_eq!(
            Mask::new(0b1000000u64, std::u64::MAX & !0b10u64).apply(101),
            101
        )
    }

    #[test]
    fn test_instruction_run() {
        assert_eq!(
            Instruction::run(vec![
                Instruction::Mask(Mask::new(0b1000000u64, std::u64::MAX & !0b010u64)),
                Instruction::MemSet(MemSet::new(8, 11)),
                Instruction::MemSet(MemSet::new(7, 101)),
                Instruction::MemSet(MemSet::new(8, 0))
            ]),
            165
        )
    }
}
