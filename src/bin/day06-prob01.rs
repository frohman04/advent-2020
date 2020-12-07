#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use crate::ParseResult::Divider;
use std::collections::hash_set::Union;
use std::collections::HashSet;

fn main() {
    let forms = std::fs::read_to_string("src/bin/day06.txt")
        .map(|file| {
            file.lines()
                .map(|line| ParseResult::from_raw(line))
                .collect::<Vec<ParseResult>>()
        })
        .expect("Unable to open file");
    let groups = ParseResult::merge(forms);
    println!(
        "{:?}",
        groups
            .iter()
            .map(|cg| cg.merged().questions.len())
            .sum::<usize>()
    );
}

#[derive(Debug, PartialEq)]
struct CustomsGroup {
    forms: Vec<Customs>,
}

impl CustomsGroup {
    pub fn new(forms: Vec<Customs>) -> CustomsGroup {
        CustomsGroup { forms }
    }

    pub fn merged(&self) -> Customs {
        let mut out: HashSet<char> = HashSet::new();
        for form in &self.forms {
            out.extend(&form.questions);
        }
        Customs::new(out)
    }
}

#[derive(Debug, PartialEq)]
enum ParseResult {
    Form(Customs),
    Divider,
}

impl ParseResult {
    pub fn from_raw(line: &str) -> ParseResult {
        if line.is_empty() {
            ParseResult::Divider
        } else {
            ParseResult::Form(Customs::from_raw(line))
        }
    }

    pub fn merge(results: Vec<ParseResult>) -> Vec<CustomsGroup> {
        let mut out: Vec<CustomsGroup> = Vec::new();

        let mut partials: Vec<Customs> = Vec::new();
        for result in results {
            match result {
                ParseResult::Divider => {
                    out.push(CustomsGroup::new(partials));
                    partials = Vec::new();
                }
                ParseResult::Form(customs) => partials.push(customs),
            }
        }
        if !partials.is_empty() {
            out.push(CustomsGroup::new(partials));
        }

        out
    }
}

#[derive(Debug, PartialEq)]
struct Customs {
    questions: HashSet<char>,
}

impl Customs {
    pub fn new(questions: HashSet<char>) -> Customs {
        Customs { questions }
    }

    pub fn from_raw(line: &str) -> Customs {
        let questions = line.trim().chars().collect::<HashSet<char>>();
        Customs::new(questions)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_customs_from_raw() {
        assert_eq!(
            Customs::from_raw("abcx"),
            Customs::new(hashset!('a', 'b', 'c', 'x'))
        )
    }

    #[test]
    fn test_parseresult_from_raw_form() {
        assert_eq!(
            ParseResult::from_raw("abcx"),
            ParseResult::Form(Customs::new(hashset!('a', 'b', 'c', 'x')))
        )
    }

    #[test]
    fn test_parseresult_from_raw_divider() {
        assert_eq!(ParseResult::from_raw(""), ParseResult::Divider)
    }

    #[test]
    fn test_parseresult_merge_single_term_divider() {
        assert_eq!(
            ParseResult::merge(vec![
                ParseResult::Form(Customs::new(hashset!('a'))),
                ParseResult::Form(Customs::new(hashset!('b'))),
                ParseResult::Form(Customs::new(hashset!('c'))),
                ParseResult::Form(Customs::new(hashset!('x'))),
                ParseResult::Divider
            ]),
            vec![CustomsGroup::new(vec![
                Customs::new(hashset!('a')),
                Customs::new(hashset!('b')),
                Customs::new(hashset!('c')),
                Customs::new(hashset!('x')),
            ])]
        )
    }

    #[test]
    fn test_parseresult_merge_single_no_term_divider() {
        assert_eq!(
            ParseResult::merge(vec![
                ParseResult::Form(Customs::new(hashset!('a'))),
                ParseResult::Form(Customs::new(hashset!('b'))),
                ParseResult::Form(Customs::new(hashset!('c'))),
                ParseResult::Form(Customs::new(hashset!('x'))),
            ]),
            vec![CustomsGroup::new(vec![
                Customs::new(hashset!('a')),
                Customs::new(hashset!('b')),
                Customs::new(hashset!('c')),
                Customs::new(hashset!('x')),
            ])]
        )
    }

    #[test]
    fn test_parseresult_merge_multiple() {
        assert_eq!(
            ParseResult::merge(vec![
                ParseResult::Form(Customs::new(hashset!('a'))),
                ParseResult::Form(Customs::new(hashset!('b'))),
                ParseResult::Form(Customs::new(hashset!('c'))),
                ParseResult::Form(Customs::new(hashset!('x'))),
                ParseResult::Divider,
                ParseResult::Form(Customs::new(hashset!('q'))),
                ParseResult::Form(Customs::new(hashset!('w'))),
                ParseResult::Form(Customs::new(hashset!('e'))),
                ParseResult::Form(Customs::new(hashset!('r'))),
                ParseResult::Divider,
            ]),
            vec![
                CustomsGroup::new(vec![
                    Customs::new(hashset!('a')),
                    Customs::new(hashset!('b')),
                    Customs::new(hashset!('c')),
                    Customs::new(hashset!('x')),
                ]),
                CustomsGroup::new(vec![
                    Customs::new(hashset!('q')),
                    Customs::new(hashset!('w')),
                    Customs::new(hashset!('e')),
                    Customs::new(hashset!('r')),
                ])
            ]
        )
    }

    #[test]
    fn test_customsgroup_merge() {
        assert_eq!(
            CustomsGroup::new(vec![
                Customs::new(hashset!('a')),
                Customs::new(hashset!('b')),
                Customs::new(hashset!('c')),
                Customs::new(hashset!('x')),
            ])
            .merged(),
            Customs::new(hashset!['a', 'b', 'c', 'x'])
        )
    }
}
