#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;

fn main() {
    let (grammar, messages) = std::fs::read_to_string("src/bin/day19.txt")
        .map(|file| {
            let mut grammar: Vec<String> = Vec::new();
            let mut messages: Vec<String> = Vec::new();
            let mut in_messages = false;
            for line in file.lines() {
                if line.is_empty() {
                    in_messages = true;
                    continue;
                }
                if in_messages {
                    messages.push(line.to_string());
                } else {
                    grammar.push(line.to_string());
                }
            }
            (grammar, messages)
        })
        .expect("Unable to open file");
    println!(
        "{:?}",
        validate_messages(Symbol::parse_grammar(grammar), messages)
    );
}

#[derive(Debug, PartialEq)]
enum Symbol {
    Nonterm(Vec<Vec<usize>>),
    Term(char),
}

impl Symbol {
    fn parse_grammar(lines: Vec<String>) -> HashMap<usize, Symbol> {
        lines
            .into_iter()
            .map(|line| Symbol::from_str(line.as_str()))
            .collect()
    }

    fn from_str(line: &str) -> (usize, Symbol) {
        let pieces = line.split(": ").collect::<Vec<&str>>();
        let name = pieces[0]
            .parse::<usize>()
            .expect(&format!("Unable to parse usize from {}", pieces[0]));
        let sym = if pieces[1].starts_with('"') {
            Symbol::Term(pieces[1].chars().nth(1).unwrap())
        } else {
            Symbol::Nonterm(
                pieces[1]
                    .split(" | ")
                    .map(|chunk| {
                        chunk
                            .split(" ")
                            .map(|pname| {
                                pname
                                    .parse()
                                    .expect(&format!("Unable to parse usize from {}", pname))
                            })
                            .collect()
                    })
                    .collect(),
            )
        };
        (name, sym)
    }
}

fn validate_messages(grammar: HashMap<usize, Symbol>, messages: Vec<String>) -> usize {
    messages
        .into_iter()
        .filter(|message| validate_message(&grammar, message))
        // .map(|m| {
        //     println!("{}", m);
        //     m
        // })
        .count()
}

fn validate_message(grammar: &HashMap<usize, Symbol>, message: &String) -> bool {
    match validate_message_int(&grammar, message, &0, 0) {
        Ok(i) => {
            if i == message.len() {
                true
            } else {
                // println!("{}", message);
                // println!("Unable to match due to unmatched characters after completing parsing (unparsed starting at {})", i);
                false
            }
        }
        Err(_reason) => {
            // println!("{}", message);
            // println!("{}", reason);
            false
        }
    }
}

fn validate_message_int(
    grammar: &HashMap<usize, Symbol>,
    message: &String,
    sym_i: &usize,
    mess_i: usize,
) -> Result<usize, String> {
    let sym = &grammar[sym_i];
    match sym {
        Symbol::Term(c) => {
            if mess_i >= message.len() {
                Err(format!(
                    "{} => {:?}: Indexed off end of message while looking for '{}' in {}",
                    sym_i, sym, c, message
                ))
            } else if *c
                == message
                    .chars()
                    .nth(mess_i)
                    .expect(&format!("No {}th character in '{}'", mess_i, message))
            {
                Ok(mess_i + 1)
            } else {
                Err(format!(
                    "{} => {:?}: Expected '{}' at {} in message {}",
                    sym_i, sym, c, mess_i, message
                ))
            }
        }
        Symbol::Nonterm(prods) => {
            let mut errors: Vec<(Vec<usize>, String)> = Vec::new();
            for prod in prods {
                let mut mess_i = mess_i;
                let mut is_match = true;
                for next_sym in prod {
                    match validate_message_int(grammar, message, next_sym, mess_i) {
                        Ok(next_mess_i) => mess_i = next_mess_i,
                        Err(reason) => {
                            errors.push((prod.clone(), reason));
                            is_match = false;
                            break;
                        }
                    }
                }
                if is_match {
                    return Ok(mess_i);
                }
            }
            Err(format!(
                "{} => {:?}: Unable to match any productions of rule:\n{}",
                sym_i,
                sym,
                errors
                    .into_iter()
                    .map(|(_prod, error)| error
                        .lines()
                        .map(|line| format!("  {}", line))
                        .join("\n"))
                    .join("\n")
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_symbol_from_str_term() {
        assert_eq!(Symbol::from_str("0: \"a\""), (0, Symbol::Term('a')))
    }

    #[test]
    fn test_symbol_from_str_nonterm_single() {
        assert_eq!(
            Symbol::from_str("0: 4 1 5"),
            (0, Symbol::Nonterm(vec![vec![4, 1, 5]]))
        )
    }

    #[test]
    fn test_symbol_from_str_nonterm_digits() {
        assert_eq!(
            Symbol::from_str("0: 42 37"),
            (0, Symbol::Nonterm(vec![vec![42, 37]]))
        )
    }

    #[test]
    fn test_symbol_from_str_nonterm_multiple() {
        assert_eq!(
            Symbol::from_str("1: 2 3 | 3 2"),
            (1, Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]))
        )
    }

    #[test]
    fn test_symbol_parse_grammar() {
        assert_eq!(
            Symbol::parse_grammar(vec![
                "0: 1 2".to_string(),
                "1: \"a\"".to_string(),
                "2: 1 3 | 3 1".to_string(),
                "3: \"b\"".to_string(),
            ]),
            hashmap!(
                0 => Symbol::Nonterm(vec![vec![1, 2]]),
                1 => Symbol::Term('a'),
                2 => Symbol::Nonterm(vec![vec![1, 3], vec![3, 1]]),
                3 => Symbol::Term('b')
            )
        )
    }

    #[test]
    fn test_validate_message_static() {
        assert_eq!(
            validate_message(&hashmap!(0 => Symbol::Term('a')), &"a".to_string()),
            true
        )
    }

    #[test]
    fn test_validate_message_static_fail() {
        assert_eq!(
            validate_message(&hashmap!(0 => Symbol::Term('a')), &"b".to_string()),
            false
        )
    }

    #[test]
    fn test_validate_message_multi_rule() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1]]),
                    1 => Symbol::Nonterm(vec![vec![2]]),
                    2 => Symbol::Nonterm(vec![vec![3]]),
                    3 => Symbol::Nonterm(vec![vec![4]]),
                    4 => Symbol::Nonterm(vec![vec![5]]),
                    5 => Symbol::Term('a')
                ),
                &"a".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message_multi_rule_false() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1]]),
                    1 => Symbol::Nonterm(vec![vec![2]]),
                    2 => Symbol::Nonterm(vec![vec![3]]),
                    3 => Symbol::Nonterm(vec![vec![4]]),
                    4 => Symbol::Nonterm(vec![vec![5]]),
                    5 => Symbol::Term('a')
                ),
                &"b".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message_multi_term() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1, 1, 1, 1]]),
                    1 => Symbol::Term('a')
                ),
                &"aaaaa".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message_multi_term_fail() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1, 1, 1, 1]]),
                    1 => Symbol::Term('a')
                ),
                &"aabaa".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message_multi_prod1() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1, 1], vec![2, 2, 2]]),
                    1 => Symbol::Term('a'),
                    2 => Symbol::Term('b')
                ),
                &"aaa".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message_multi_prod2() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1, 1], vec![2, 2, 2]]),
                    1 => Symbol::Term('a'),
                    2 => Symbol::Term('b')
                ),
                &"bbb".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message_multi_prod_backtrack() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1, 1], vec![1, 2, 1]]),
                    1 => Symbol::Term('a'),
                    2 => Symbol::Term('b')
                ),
                &"aba".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message_multi_prod_fail() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1, 1], vec![2, 2, 2]]),
                    1 => Symbol::Term('a'),
                    2 => Symbol::Term('b')
                ),
                &"aab".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message_too_few_chars() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1]]),
                    1 => Symbol::Term('a')
                ),
                &"a".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message_leftover_chars() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 1]]),
                    1 => Symbol::Term('a')
                ),
                &"aaa".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message1() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![4, 1, 5]]),
                    1 => Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]),
                    2 => Symbol::Nonterm(vec![vec![4, 4], vec![5, 5]]),
                    3 => Symbol::Nonterm(vec![vec![4, 5], vec![5, 4]]),
                    4 => Symbol::Term('a'),
                    5 => Symbol::Term('b')
                ),
                &"ababbb".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message2() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![4, 1, 5]]),
                    1 => Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]),
                    2 => Symbol::Nonterm(vec![vec![4, 4], vec![5, 5]]),
                    3 => Symbol::Nonterm(vec![vec![4, 5], vec![5, 4]]),
                    4 => Symbol::Term('a'),
                    5 => Symbol::Term('b')
                ),
                &"bababa".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message3() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![4, 1, 5]]),
                    1 => Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]),
                    2 => Symbol::Nonterm(vec![vec![4, 4], vec![5, 5]]),
                    3 => Symbol::Nonterm(vec![vec![4, 5], vec![5, 4]]),
                    4 => Symbol::Term('a'),
                    5 => Symbol::Term('b')
                ),
                &"abbbab".to_string()
            ),
            true
        )
    }

    #[test]
    fn test_validate_message4() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![4, 1, 5]]),
                    1 => Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]),
                    2 => Symbol::Nonterm(vec![vec![4, 4], vec![5, 5]]),
                    3 => Symbol::Nonterm(vec![vec![4, 5], vec![5, 4]]),
                    4 => Symbol::Term('a'),
                    5 => Symbol::Term('b')
                ),
                &"aaabbb".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_message5() {
        assert_eq!(
            validate_message(
                &hashmap!(
                    0 => Symbol::Nonterm(vec![vec![4, 1, 5]]),
                    1 => Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]),
                    2 => Symbol::Nonterm(vec![vec![4, 4], vec![5, 5]]),
                    3 => Symbol::Nonterm(vec![vec![4, 5], vec![5, 4]]),
                    4 => Symbol::Term('a'),
                    5 => Symbol::Term('b')
                ),
                &"aaaabbb".to_string()
            ),
            false
        )
    }

    #[test]
    fn test_validate_messages1() {
        assert_eq!(
            validate_messages(
                hashmap!(
                    0 => Symbol::Nonterm(vec![vec![4, 1, 5]]),
                    1 => Symbol::Nonterm(vec![vec![2, 3], vec![3, 2]]),
                    2 => Symbol::Nonterm(vec![vec![4, 4], vec![5, 5]]),
                    3 => Symbol::Nonterm(vec![vec![4, 5], vec![5, 4]]),
                    4 => Symbol::Term('a'),
                    5 => Symbol::Term('b')
                ),
                vec![
                    "ababbb".to_string(),
                    "bababa".to_string(),
                    "abbbab".to_string(),
                    "aaabbb".to_string(),
                    "aaaabbb".to_string()
                ]
            ),
            2
        )
    }

    #[test]
    fn test_validate_messages2() {
        assert_eq!(
            validate_messages(
                hashmap!(
                    0 => Symbol::Nonterm(vec![vec![1, 2]]),
                    1 => Symbol::Term('a'),
                    2 => Symbol::Nonterm(vec![vec![1, 3], vec![3, 1]]),
                    3 => Symbol::Term('b')
                ),
                vec![
                    "aab".to_string(),
                    "aba".to_string(),
                    "a".to_string(),
                    "ab".to_string(),
                    "aa".to_string()
                ]
            ),
            2
        )
    }
}
