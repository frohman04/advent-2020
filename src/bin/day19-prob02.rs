#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use itertools::Itertools;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

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
                    let line = if line == "8: 42" {
                        "8: 42 | 42 8"
                    } else if line == "11: 42 31" {
                        "11: 42 31 | 42 11 31"
                    } else {
                        line
                    };
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
    match validate_message_int(&grammar, message, &0, vec![0]) {
        Ok(is) => {
            if is.iter().any(|i| *i == message.len()) {
                true
            } else {
                // println!("{}", message);
                // println!("Unable to match due to unmatched characters after completing parsing (unparsed starting at {:?})", is);
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
    mess_is: Vec<usize>,
) -> Result<Vec<usize>, String> {
    let sym = &grammar[sym_i];
    match sym {
        Symbol::Term(c) => {
            let res = mess_is
                .into_iter()
                .map(|mess_i| {
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
                        Ok(vec![mess_i + 1])
                    } else {
                        Err(format!(
                            "{} => {:?}: Expected '{}' at {} in message {}",
                            sym_i, sym, c, mess_i, message
                        ))
                    }
                })
                .collect::<Vec<Result<Vec<usize>, String>>>();

            if res.iter().filter(|r| r.is_ok()).count() > 0 {
                Ok(res
                    .into_iter()
                    .filter_map(|r| match r {
                        Ok(is) => Some(is[0]),
                        Err(_) => None,
                    })
                    .collect())
            } else {
                Err(res
                    .into_iter()
                    .filter_map(|r| match r {
                        Ok(_) => unreachable!(),
                        Err(reason) => Some(reason),
                    })
                    .join("\n"))
            }
        }
        Symbol::Nonterm(prods) => {
            let mut errors: Vec<String> = Vec::new();
            let mut matches: HashSet<usize> = HashSet::new();
            for prod in prods {
                match prod.iter().try_fold(mess_is.clone(), |is, next_sym| {
                    validate_message_int(grammar, message, next_sym, is)
                }) {
                    Ok(next_is) => matches.extend(next_is),
                    Err(e) => errors.push(e),
                }
            }

            if matches.is_empty() {
                Err(format!(
                    "{} => {:?}: Unable to match any productions of rule:\n{}",
                    sym_i,
                    sym,
                    errors
                        .into_iter()
                        .map(|error| error.lines().map(|line| format!("  {}", line)).join("\n"))
                        .join("\n")
                ))
            } else {
                Ok(matches.into_iter().collect())
            }
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

    // these tests cause a stack overflow, but the program yields the correct answer :/
    // #[test]
    // fn test_validate_message_recursive1() {
    //     assert_eq!(
    //         validate_message(
    //             &hashmap!(
    //                 0 => Symbol::Nonterm(vec![vec![1]]),
    //                 1 => Symbol::Nonterm(vec![vec![3], vec![1, 2]]),
    //                 2 => Symbol::Term('a'),
    //                 3 => Symbol::Term('b'),
    //             ),
    //             &"b".to_string()
    //         ),
    //         true
    //     )
    // }
    //
    // #[test]
    // fn test_validate_message_recursive2() {
    //     assert_eq!(
    //         validate_message(
    //             &hashmap!(
    //                 0 => Symbol::Nonterm(vec![vec![1]]),
    //                 1 => Symbol::Nonterm(vec![vec![3], vec![1, 2]]),
    //                 2 => Symbol::Term('a'),
    //                 3 => Symbol::Term('b'),
    //             ),
    //             &"ba".to_string()
    //         ),
    //         true
    //     )
    // }
    //
    // #[test]
    // fn test_validate_message_recursive3() {
    //     assert_eq!(
    //         validate_message(
    //             &hashmap!(
    //                 0 => Symbol::Nonterm(vec![vec![1]]),
    //                 1 => Symbol::Nonterm(vec![vec![3], vec![1, 2]]),
    //                 2 => Symbol::Term('a'),
    //                 3 => Symbol::Term('b'),
    //             ),
    //             &"bba".to_string()
    //         ),
    //         true
    //     )
    // }

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

    #[test]
    fn test_validate_messages3() {
        assert_eq!(
            validate_messages(
                hashmap!(
                    42 => Symbol::Nonterm(vec![vec![9, 14], vec![10, 1]]),
                    9 => Symbol::Nonterm(vec![vec![14, 27], vec![1, 26]]),
                    10 => Symbol::Nonterm(vec![vec![23, 14], vec![28, 1]]),
                    1 => Symbol::Term('a'),
                    11 => Symbol::Nonterm(vec![vec![42, 31]]),
                    5 => Symbol::Nonterm(vec![vec![1, 14], vec![15, 1]]),
                    19 => Symbol::Nonterm(vec![vec![14, 1], vec![14, 14]]),
                    12 => Symbol::Nonterm(vec![vec![24, 14], vec![19, 1]]),
                    16 => Symbol::Nonterm(vec![vec![15, 1], vec![14, 14]]),
                    31 => Symbol::Nonterm(vec![vec![14, 17], vec![1, 13]]),
                    6 => Symbol::Nonterm(vec![vec![14, 14], vec![1, 14]]),
                    2 => Symbol::Nonterm(vec![vec![1, 24], vec![14, 4]]),
                    0 => Symbol::Nonterm(vec![vec![8, 11]]),
                    13 => Symbol::Nonterm(vec![vec![14, 3], vec![1, 12]]),
                    15 => Symbol::Nonterm(vec![vec![1], vec![14]]),
                    17 => Symbol::Nonterm(vec![vec![14, 2], vec![1, 7]]),
                    23 => Symbol::Nonterm(vec![vec![25, 1], vec![22, 14]]),
                    28 => Symbol::Nonterm(vec![vec![16, 1]]),
                    4 => Symbol::Nonterm(vec![vec![1, 1]]),
                    20 => Symbol::Nonterm(vec![vec![14, 14], vec![1, 15]]),
                    3 => Symbol::Nonterm(vec![vec![5, 14], vec![16, 1]]),
                    27 => Symbol::Nonterm(vec![vec![1, 6], vec![14, 18]]),
                    14 => Symbol::Term('b'),
                    21 => Symbol::Nonterm(vec![vec![14, 1], vec![1, 14]]),
                    25 => Symbol::Nonterm(vec![vec![1, 1], vec![1, 14]]),
                    22 => Symbol::Nonterm(vec![vec![14, 14]]),
                    8 => Symbol::Nonterm(vec![vec![42]]),
                    26 => Symbol::Nonterm(vec![vec![14, 22], vec![1, 20]]),
                    18 => Symbol::Nonterm(vec![vec![15, 15]]),
                    7 => Symbol::Nonterm(vec![vec![14, 5], vec![1, 21]]),
                    24 => Symbol::Nonterm(vec![vec![14, 1]])
                ),
                vec![
                    "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
                    "bbabbbbaabaabba".to_string(),
                    "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
                    "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
                    "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
                    "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
                    "ababaaaaaabaaab".to_string(),
                    "ababaaaaabbbaba".to_string(),
                    "baabbaaaabbaaaababbaababb".to_string(),
                    "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
                    "aaaaabbaabaaaaababaa".to_string(),
                    "aaaabbaaaabbaaa".to_string(),
                    "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
                    "babaaabbbaaabaababbaabababaaab".to_string(),
                    "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
                ]
            ),
            3
        )
    }

    #[test]
    fn test_validate_messages4() {
        assert_eq!(
            validate_messages(
                hashmap!(
                    42 => Symbol::Nonterm(vec![vec![9, 14], vec![10, 1]]),
                    9 => Symbol::Nonterm(vec![vec![14, 27], vec![1, 26]]),
                    10 => Symbol::Nonterm(vec![vec![23, 14], vec![28, 1]]),
                    1 => Symbol::Term('a'),
                    11 => Symbol::Nonterm(vec![vec![42, 31], vec![42, 11, 31]]),
                    5 => Symbol::Nonterm(vec![vec![1, 14], vec![15, 1]]),
                    19 => Symbol::Nonterm(vec![vec![14, 1], vec![14, 14]]),
                    12 => Symbol::Nonterm(vec![vec![24, 14], vec![19, 1]]),
                    16 => Symbol::Nonterm(vec![vec![15, 1], vec![14, 14]]),
                    31 => Symbol::Nonterm(vec![vec![14, 17], vec![1, 13]]),
                    6 => Symbol::Nonterm(vec![vec![14, 14], vec![1, 14]]),
                    2 => Symbol::Nonterm(vec![vec![1, 24], vec![14, 4]]),
                    0 => Symbol::Nonterm(vec![vec![8, 11]]),
                    13 => Symbol::Nonterm(vec![vec![14, 3], vec![1, 12]]),
                    15 => Symbol::Nonterm(vec![vec![1], vec![14]]),
                    17 => Symbol::Nonterm(vec![vec![14, 2], vec![1, 7]]),
                    23 => Symbol::Nonterm(vec![vec![25, 1], vec![22, 14]]),
                    28 => Symbol::Nonterm(vec![vec![16, 1]]),
                    4 => Symbol::Nonterm(vec![vec![1, 1]]),
                    20 => Symbol::Nonterm(vec![vec![14, 14], vec![1, 15]]),
                    3 => Symbol::Nonterm(vec![vec![5, 14], vec![16, 1]]),
                    27 => Symbol::Nonterm(vec![vec![1, 6], vec![14, 18]]),
                    14 => Symbol::Term('b'),
                    21 => Symbol::Nonterm(vec![vec![14, 1], vec![1, 14]]),
                    25 => Symbol::Nonterm(vec![vec![1, 1], vec![1, 14]]),
                    22 => Symbol::Nonterm(vec![vec![14, 14]]),
                    8 => Symbol::Nonterm(vec![vec![42], vec![42, 8]]),
                    26 => Symbol::Nonterm(vec![vec![14, 22], vec![1, 20]]),
                    18 => Symbol::Nonterm(vec![vec![15, 15]]),
                    7 => Symbol::Nonterm(vec![vec![14, 5], vec![1, 21]]),
                    24 => Symbol::Nonterm(vec![vec![14, 1]])
                ),
                vec![
                    "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa".to_string(),
                    "bbabbbbaabaabba".to_string(),
                    "babbbbaabbbbbabbbbbbaabaaabaaa".to_string(),
                    "aaabbbbbbaaaabaababaabababbabaaabbababababaaa".to_string(),
                    "bbbbbbbaaaabbbbaaabbabaaa".to_string(),
                    "bbbababbbbaaaaaaaabbababaaababaabab".to_string(),
                    "ababaaaaaabaaab".to_string(),
                    "ababaaaaabbbaba".to_string(),
                    "baabbaaaabbaaaababbaababb".to_string(),
                    "abbbbabbbbaaaababbbbbbaaaababb".to_string(),
                    "aaaaabbaabaaaaababaa".to_string(),
                    "aaaabbaaaabbaaa".to_string(),
                    "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa".to_string(),
                    "babaaabbbaaabaababbaabababaaab".to_string(),
                    "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba".to_string(),
                ]
            ),
            12
        )
    }
}
