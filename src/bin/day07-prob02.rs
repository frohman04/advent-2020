#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::collections::HashMap;

fn main() {
    let bags = std::fs::read_to_string("src/bin/day07.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| parse_line(val))
                .collect::<HashMap<String, Vec<(u8, String)>>>()
        })
        .expect("Unable to open file");
    println!("{:?}", shiny_gold_contains(bags));
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

fn shiny_gold_contains(bags: HashMap<String, Vec<(u8, String)>>) -> u64 {
    let mut cache: HashMap<String, u64> = HashMap::new();
    while cache.len() != bags.len() {
        let mut additions: HashMap<String, u64> = HashMap::new();
        for bag in bags.keys().filter(|b| !cache.contains_key(*b)) {
            if bags[bag].iter().all(|(_, b)| cache.contains_key(b)) {
                let sum = bags[bag]
                    .iter()
                    .map(|(count, b)| (*count as u64) * (cache[b] + 1))
                    .sum::<u64>();
                additions.insert(bag.clone(), sum);
            }
        }
        cache.extend(additions);
    }
    cache[&"shiny gold".to_string()]
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
    fn test_shiny_gold_contains_single_layer_single_child() {
        assert_eq!(
            shiny_gold_contains(
                hashmap!("shiny gold".to_string() => vec![(2, "light blue".to_string())], "light blue".to_string() => vec![])
            ),
            2
        )
    }

    #[test]
    fn test_shiny_gold_contains_single_layer_two_children() {
        assert_eq!(
            shiny_gold_contains(
                hashmap!("shiny gold".to_string() => vec![(2, "light blue".to_string()), (3, "dark red".to_string())], "light blue".to_string() => vec![], "dark red".to_string() => vec![])
            ),
            5
        )
    }

    #[test]
    fn test_shiny_gold_contains_two_layers_single_child() {
        assert_eq!(
            shiny_gold_contains(
                hashmap!("shiny gold".to_string() => vec![(2, "muted yellow".to_string())], "muted yellow".to_string() => vec![(4, "light blue".to_string())], "light blue".to_string() => vec![])
            ),
            10
        )
    }

    #[test]
    fn test_shiny_gold_contains_two_layers_two_children() {
        assert_eq!(
            shiny_gold_contains(hashmap!(
                "shiny gold".to_string() => vec![(2, "muted yellow".to_string()), (3, "dark red".to_string())],
                "muted yellow".to_string() => vec![(4, "light blue".to_string())],
                "dark red".to_string() => vec![(5, "bright orange".to_string())],
                "light blue".to_string() => vec![],
                "bright orange".to_string() => vec![])),
            28
        )
    }

    #[test]
    fn test_shiny_gold_contains_nested() {
        assert_eq!(
            shiny_gold_contains(hashmap!(
                "shiny gold".to_string() => vec![(2, "dark red".to_string())],
                "dark red".to_string() => vec![(2, "dark orange".to_string())],
                "dark orange".to_string() => vec![(2, "dark yellow".to_string())],
                "dark yellow".to_string() => vec![(2, "dark green".to_string())],
                "dark green".to_string() => vec![(2, "dark blue".to_string())],
                "dark blue".to_string() => vec![(2, "dark violet".to_string())],
                "dark violet".to_string() => vec![]
            )),
            126
        )
    }

    #[test]
    fn test_shiny_gold_contains_complex() {
        assert_eq!(
            shiny_gold_contains(hashmap!(
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
            32
        )
    }
}
