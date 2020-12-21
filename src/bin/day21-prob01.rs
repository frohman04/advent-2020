#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use itertools::all;
use std::collections::{HashMap, HashSet};
use std::fmt::Formatter;

fn main() {
    let foods = std::fs::read_to_string("src/bin/day21.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Food::from_line(val))
                .collect::<Vec<Food>>()
        })
        .expect("Unable to open file");
    println!("{:?}", problem1(foods));
}

#[derive(Debug, PartialEq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl Food {
    pub fn new(ingredients: HashSet<String>, allergens: HashSet<String>) -> Food {
        Food {
            ingredients,
            allergens,
        }
    }

    pub fn from_line(line: &str) -> Food {
        let pieces = line
            .strip_suffix(")")
            .unwrap()
            .split(" (contains ")
            .collect::<Vec<&str>>();
        let ingredients = pieces[0].split(" ").map(|s| s.to_string()).collect();
        let allergens = pieces[1]
            .replace(",", "")
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        Food::new(ingredients, allergens)
    }
}

fn problem1(foods: Vec<Food>) -> usize {
    let safe_ingredients = find_safe_ingredients(&foods);
    foods
        .iter()
        .flat_map(|food| food.ingredients.intersection(&safe_ingredients))
        .count()
}

fn find_safe_ingredients(foods: &Vec<Food>) -> HashSet<String> {
    let allergen_ingredients = find_allergen_ingredients(foods)
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<HashSet<String>>();
    let all_ingredients = foods
        .into_iter()
        .flat_map(|food| food.ingredients.clone())
        .collect::<HashSet<String>>();
    all_ingredients
        .difference(&allergen_ingredients)
        .map(|i| i.clone())
        .collect()
}

fn find_allergen_ingredients(foods: &Vec<Food>) -> HashMap<String, String> {
    // build map of allergen -> possible ingredients (only ingredients present in all foods that
    // contain the allergen)
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    for food in foods {
        for allergen in food.allergens.iter() {
            let ingredients = allergens
                .get(allergen)
                .map(|ingredients| {
                    ingredients
                        .intersection(&food.ingredients)
                        .map(|ing| ing.clone())
                        .collect()
                })
                .unwrap_or(food.ingredients.clone());
            allergens.insert(allergen.clone(), ingredients);
        }
    }

    // find all allergens that have only one possible ingredient, then remove that ingredient from
    // all other allergens still left to solve, until no allergens are left
    let mut allergen_ingredient: HashMap<String, String> = HashMap::new();
    while !allergens.is_empty() {
        let pinned = allergens
            .iter()
            .filter_map(|(allergen, ingredients)| {
                if ingredients.len() == 1 {
                    Some((allergen.clone(), ingredients.iter().next().unwrap().clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, String)>>();

        for (allergen, ingredient) in pinned {
            allergens.remove(&allergen);
            allergens.iter_mut().for_each(|(_, ingredients)| {
                ingredients.remove(&ingredient);
            });
            allergen_ingredient.insert(allergen, ingredient);
        }
    }

    allergen_ingredient
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_food_from_line() {
        assert_eq!(
            Food::from_line("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            Food::new(
                hashset!(
                    "mxmxvkd".to_string(),
                    "kfcds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string()
                ),
                hashset!("dairy".to_string(), "fish".to_string())
            )
        )
    }

    #[test]
    fn test_find_allergen_ingredients() {
        assert_eq!(
            find_allergen_ingredients(&vec![
                Food::new(
                    hashset!(
                        "mxmxvkd".to_string(),
                        "kfcds".to_string(),
                        "sqjhc".to_string(),
                        "nhms".to_string()
                    ),
                    hashset!("dairy".to_string(), "fish".to_string())
                ),
                Food::new(
                    hashset!(
                        "trh".to_string(),
                        "fvjkl".to_string(),
                        "sbzzf".to_string(),
                        "mxmxvkd".to_string()
                    ),
                    hashset!("dairy".to_string())
                ),
                Food::new(
                    hashset!("sqjhc".to_string(), "fvjkl".to_string()),
                    hashset!("soy".to_string())
                ),
                Food::new(
                    hashset!(
                        "sqjhc".to_string(),
                        "mxmxvkd".to_string(),
                        "sbzzf".to_string()
                    ),
                    hashset!("fish".to_string())
                )
            ]),
            hashmap!(
                "dairy".to_string() => "mxmxvkd".to_string(),
                "fish".to_string() => "sqjhc".to_string(),
                "soy".to_string() => "fvjkl".to_string()
            )
        )
    }

    #[test]
    fn test_find_safe_ingredients() {
        assert_eq!(
            find_safe_ingredients(&vec![
                Food::new(
                    hashset!(
                        "mxmxvkd".to_string(),
                        "kfcds".to_string(),
                        "sqjhc".to_string(),
                        "nhms".to_string()
                    ),
                    hashset!("dairy".to_string(), "fish".to_string())
                ),
                Food::new(
                    hashset!(
                        "trh".to_string(),
                        "fvjkl".to_string(),
                        "sbzzf".to_string(),
                        "mxmxvkd".to_string()
                    ),
                    hashset!("dairy".to_string())
                ),
                Food::new(
                    hashset!("sqjhc".to_string(), "fvjkl".to_string()),
                    hashset!("soy".to_string())
                ),
                Food::new(
                    hashset!(
                        "sqjhc".to_string(),
                        "mxmxvkd".to_string(),
                        "sbzzf".to_string()
                    ),
                    hashset!("fish".to_string())
                )
            ]),
            hashset!(
                "kfcds".to_string(),
                "nhms".to_string(),
                "sbzzf".to_string(),
                "trh".to_string()
            )
        )
    }

    #[test]
    fn test_problem1() {
        assert_eq!(
            problem1(vec![
                Food::new(
                    hashset!(
                        "mxmxvkd".to_string(),
                        "kfcds".to_string(),
                        "sqjhc".to_string(),
                        "nhms".to_string()
                    ),
                    hashset!("dairy".to_string(), "fish".to_string())
                ),
                Food::new(
                    hashset!(
                        "trh".to_string(),
                        "fvjkl".to_string(),
                        "sbzzf".to_string(),
                        "mxmxvkd".to_string()
                    ),
                    hashset!("dairy".to_string())
                ),
                Food::new(
                    hashset!("sqjhc".to_string(), "fvjkl".to_string()),
                    hashset!("soy".to_string())
                ),
                Food::new(
                    hashset!(
                        "sqjhc".to_string(),
                        "mxmxvkd".to_string(),
                        "sbzzf".to_string()
                    ),
                    hashset!("fish".to_string())
                )
            ]),
            5
        )
    }
}
