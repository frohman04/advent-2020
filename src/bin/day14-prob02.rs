extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

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
    Masks(Vec<Mask>),
}

impl Instruction {
    pub fn from_raw(line: &str) -> Instruction {
        if line.starts_with("mem") {
            Instruction::MemSet(MemSet::from_raw(line))
        } else {
            Instruction::Masks(Mask::from_raw(line))
        }
    }

    pub fn run(instructions: Vec<Instruction>) -> u64 {
        let mut masks = vec![Mask::new(0, std::u64::MAX)];
        let mut mem: HashMap<usize, u64> = HashMap::new();

        for inst in instructions {
            match inst {
                Instruction::Masks(inst_masks) => masks = inst_masks,
                Instruction::MemSet(inst_memset) => {
                    masks.iter().for_each(|mask| {
                        mem.insert(
                            mask.apply(inst_memset.addr as u64) as usize,
                            inst_memset.val,
                        );
                    });
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

#[derive(PartialEq)]
struct Mask {
    set_mask: u64,
    unset_mask: u64,
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mask")
            .field("set_mask", &format_args!("{:b}", self.set_mask))
            .field("unset_mask", &format_args!("{:b}", self.unset_mask))
            .finish()
    }
}

impl Mask {
    pub fn new(set_mask: u64, unset_mask: u64) -> Mask {
        Mask {
            set_mask,
            unset_mask,
        }
    }

    pub fn from_raw(line: &str) -> Vec<Mask> {
        let raw_mask = line.strip_prefix("mask = ").expect(&format!(
            "Unable to find prefix `mask = ` for line {}",
            line
        ));

        let mut set_mask = 0u64;
        let mut flaky_bits: Vec<usize> = Vec::new();

        for (i, val) in raw_mask.chars().rev().enumerate() {
            match val {
                '0' => (),
                '1' => set_mask |= 2u64.pow(i as u32),
                'X' => flaky_bits.push(i),
                a => panic!("Found unexpected char in mask: {}", a),
            }
        }

        let flaky_bits: HashSet<usize> = {
            let mut flaky_bits_set: HashSet<usize> = HashSet::new();
            flaky_bits_set.extend(flaky_bits.into_iter());
            flaky_bits_set
        };
        let mut flaky_masks: Vec<Mask> = Vec::new();
        for len in 0..=flaky_bits.len() {
            for comb in flaky_bits.iter().combinations(len) {
                let mut set_mask_local = set_mask.clone();
                let mut unset_mask_local = std::u64::MAX;

                let to_set = comb.into_iter().map(|val| *val).collect::<HashSet<usize>>();
                flaky_bits
                    .difference(&to_set)
                    .sorted()
                    .for_each(|i| unset_mask_local &= !(2u64.pow(*i as u32)));
                to_set
                    .into_iter()
                    .sorted()
                    .for_each(|i| set_mask_local |= 2u64.pow(i as u32));

                flaky_masks.push(Mask::new(set_mask_local, unset_mask_local));
            }
        }

        flaky_masks
            .into_iter()
            .sorted_by_key(|mask| mask.set_mask)
            .collect()
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
            Instruction::from_raw("mask = 000000000000000000000000000000X1001X"),
            Instruction::Masks(vec![
                Mask::new(0b010010u64, std::u64::MAX & !0b100001u64),
                Mask::new(0b010011u64, std::u64::MAX & !0b100000u64),
                Mask::new(0b110010u64, std::u64::MAX & !0b000001u64),
                Mask::new(0b110011u64, std::u64::MAX & !0b000000u64)
            ])
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
                Instruction::Masks(vec![
                    Mask::new(0b010010u64, std::u64::MAX & !0b100001u64),
                    Mask::new(0b010011u64, std::u64::MAX & !0b100000u64),
                    Mask::new(0b110010u64, std::u64::MAX & !0b000001u64),
                    Mask::new(0b110011u64, std::u64::MAX & !0b000000u64)
                ]),
                Instruction::MemSet(MemSet::new(42, 100)),
                Instruction::from_raw("mem[42] = 100"),
                Instruction::Masks(vec![
                    Mask::new(0b0000, std::u64::MAX & !0b1011),
                    Mask::new(0b0001, std::u64::MAX & !0b1010),
                    Mask::new(0b0010, std::u64::MAX & !0b1001),
                    Mask::new(0b0011, std::u64::MAX & !0b1000),
                    Mask::new(0b1000, std::u64::MAX & !0b0011),
                    Mask::new(0b1001, std::u64::MAX & !0b0010),
                    Mask::new(0b1010, std::u64::MAX & !0b0001),
                    Mask::new(0b1011, std::u64::MAX & !0b0000)
                ]),
                Instruction::from_raw("mask = 00000000000000000000000000000000X0XX"),
                Instruction::MemSet(MemSet::new(26, 1))
            ]),
            208
        )
    }
}
