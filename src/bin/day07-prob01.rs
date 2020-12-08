#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    let bags = std::fs::read_to_string("src/bin/day07.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| parse_line(val))
                .collect::<HashMap<String, Vec<(u8, String)>>>()
        })
        .expect("Unable to open file");
    println!("{:?}", find_shiny_gold(bags).len());
}

fn parse_line<'a>(line: &str) -> (String, Vec<(u8, String)>) {
    let parts = line.split(" bags contain ").collect::<Vec<&str>>();
    let source_color = parts[0].to_string();
    let contents = parts[1]
        .trim_end_matches(".")
        .split(", ")
        .filter(|p| *p != "no other bags")
        .map(|p| {
            let clean = p
                .trim_end_matches("s")
                .trim_end_matches(" bag")
                .split(" ")
                .collect::<Vec<&str>>();
            let count = clean[0].parse::<u8>().expect(&format!(
                "Unable to parse count from '{}' in '{}' in '{}'",
                clean[0], p, line
            ));
            let color = clean[1..clean.len()].join(" ").to_string();
            (count, color)
        })
        .collect::<Vec<(u8, String)>>();
    (source_color, contents)
}

fn find_shiny_gold(bags: HashMap<String, Vec<(u8, String)>>) -> HashSet<String> {
    let mut queue: VecDeque<LinkedList<String>> = VecDeque::new();
    bags.keys().for_each(|k| {
        let mut ll = LinkedList::new();
        ll.push_front(k.clone());
        queue.push_back(ll);
    });

    let mut out: HashSet<String> = HashSet::new();
    while !queue.is_empty() {
        let to_process_chain = queue.pop_front().unwrap();
        for (_, contained_bag) in bags[to_process_chain.front().unwrap()].iter() {
            if contained_bag == "shiny gold" {
                out.insert(to_process_chain.back().unwrap().clone());
            } else {
                let mut new_chain = to_process_chain.clone();
                new_chain.push_front(contained_bag.clone());
                queue.push_back(new_chain);
            }
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line_no_bags() {
        assert_eq!(
            parse_line("faded blue bags contain no other bags."),
            ("faded blue".to_string(), vec![])
        )
    }

    #[test]
    fn test_parse_line_one_singular_bag() {
        assert_eq!(
            parse_line("bright white bags contain 1 shiny gold bag."),
            (
                "bright white".to_string(),
                vec![(1, "shiny gold".to_string())]
            )
        )
    }

    #[test]
    fn test_parse_line_one_plural_bag() {
        assert_eq!(
            parse_line("bright white bags contain 2 shiny gold bags."),
            (
                "bright white".to_string(),
                vec![(2, "shiny gold".to_string())]
            )
        )
    }

    #[test]
    fn test_parse_line_multiple_bags() {
        assert_eq!(
            parse_line("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            (
                "vibrant plum".to_string(),
                vec![
                    (5, "faded blue".to_string()),
                    (6, "dotted black".to_string())
                ]
            )
        )
    }

    #[test]
    fn test_find_shiny_gold_self() {
        assert_eq!(
            find_shiny_gold(hashmap!("shiny gold".to_string() => vec![])),
            hashset!()
        )
    }

    #[test]
    fn test_find_shiny_gold_one_layer() {
        assert_eq!(
            find_shiny_gold(
                hashmap!("light red".to_string() => vec![(4, "shiny gold".to_string())])
            ),
            hashset!("light red".to_string())
        )
    }

    #[test]
    fn test_find_shiny_gold_multiple_layers() {
        assert_eq!(
            find_shiny_gold(
                hashmap!("light red".to_string() => vec![(2, "muted yellow".to_string())], "muted yellow".to_string() => vec![(4, "shiny gold".to_string())])
            ),
            hashset!("light red".to_string(), "muted yellow".to_string())
        )
    }

    #[test]
    fn test_find_shiny_gold_complex() {
        assert_eq!(
            find_shiny_gold(hashmap!(
                "light red".to_string() => vec![(1, "bright white".to_string()), (2, "muted yellow".to_string())],
                "dark orange".to_string() => vec![(3, "bright white".to_string()), (4, "muted yellow".to_string())],
                "bright white".to_string() => vec![(1, "shiny gold".to_string())],
                "muted yellow".to_string() => vec![(2, "shiny gold".to_string()), (9, "faded blue".to_string())],
                "shiny gold".to_string() => vec![(1, "dark olive".to_string()), (2, "vibrant plum".to_string())],
                "dark olive".to_string() => vec![(3, "faded blue".to_string()), (4, "dotted black".to_string())],
                "vibrant plum".to_string() => vec![(5, "faded blue".to_string()), (6, "dotted black".to_string())],
                "faded blue".to_string() => vec![],
                "dotted black".to_string() => vec![]
            )),
            hashset!(
                "bright white".to_string(),
                "muted yellow".to_string(),
                "dark orange".to_string(),
                "light red".to_string()
            )
        )
    }
}
