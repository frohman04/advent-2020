#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn main() {
    let tiles = std::fs::read_to_string("src/bin/day20.txt")
        .map(|file| {
            let mut tiles: Vec<Rc<Tile>> = Vec::new();
            let mut curr_tile: Vec<String> = Vec::new();
            for line in file.lines() {
                if line.is_empty() && !curr_tile.is_empty() {
                    tiles.push(Rc::new(Tile::from_lines(curr_tile.clone())));
                    curr_tile.clear();
                } else {
                    curr_tile.push(line.to_string());
                }
            }
            tiles
        })
        .expect("Unable to open file");
    println!("{:?}", find_corners(tiles).iter().product::<usize>());
}

#[derive(Debug, PartialEq)]
struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
    edge_counts: HashSet<usize>,
    forward_edges: Vec<u16>,
    backward_edges: Vec<u16>,
}

impl Tile {
    pub fn new(
        id: usize,
        pixels: Vec<Vec<bool>>,
        edge_counts: HashSet<usize>,
        forward_edges: Vec<u16>,
        backward_edges: Vec<u16>,
    ) -> Tile {
        Tile {
            id,
            pixels,
            edge_counts,
            forward_edges,
            backward_edges,
        }
    }

    pub fn from_lines(lines: Vec<String>) -> Tile {
        let id_raw = lines[0]
            .strip_prefix("Tile ")
            .expect("id line did not start with 'Tile '")
            .strip_suffix(":")
            .expect("id line did not end with ':'");
        let id = id_raw
            .parse::<usize>()
            .expect(&format!("Unable to parse usize from '{}'", id_raw));

        let pixels = lines
            .into_iter()
            .skip(1)
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect::<Vec<Vec<bool>>>();
        let edges = {
            let top = pixels[0].clone();
            let bottom = pixels[pixels.len() - 1].clone();
            let left = pixels
                .iter()
                .map(|row| row[0].clone())
                .collect::<Vec<bool>>();
            let right = pixels
                .iter()
                .map(|row| row[row.len() - 1].clone())
                .collect::<Vec<bool>>();
            vec![top, right, bottom, left]
        };
        let forward_edges = edges
            .iter()
            .map(|edge| {
                edge.iter().fold(0u16, |acc, state| {
                    acc.rotate_left(1) + if *state { 1 } else { 0 }
                })
            })
            .collect::<Vec<u16>>();
        let backward_edges = edges
            .iter()
            .map(|edge| {
                edge.iter().rev().fold(0u16, |acc, state| {
                    acc.rotate_left(1) + if *state { 1 } else { 0 }
                })
            })
            .collect::<Vec<u16>>();
        let edge_counts = edges
            .iter()
            .map(|edge| edge.iter().filter(|p| **p).count())
            .collect::<HashSet<usize>>();

        Tile::new(id, pixels, edge_counts, forward_edges, backward_edges)
    }
}

fn find_corners(tiles: Vec<Rc<Tile>>) -> HashSet<usize> {
    let cache = tiles
        .iter()
        .flat_map(|tile| {
            tile.forward_edges
                .iter()
                .map(move |edge| (edge.clone(), tile.id.clone()))
                .chain(
                    tile.backward_edges
                        .iter()
                        .map(move |edge| (edge.clone(), tile.id.clone())),
                )
        })
        .fold(HashMap::new(), |mut map, (edge, tile)| {
            map.entry(edge)
                .or_insert_with(|| HashSet::new())
                .insert(tile.clone());
            map
        });
    // println!("{:?}", cache);
    tiles
        .into_iter()
        .filter_map(|tile| {
            // println!("tile: {:?}", tile.id);
            if (0usize..4)
                .filter(|i| {
                    // println!("  forward: {:?}, backward: {:?}", tile.forward_edges[*i], tile.backward_edges[*i]);
                    !(cache
                        .get(&tile.forward_edges[*i])
                        .map_or(false, |matches| matches.len() == 1)
                        | cache
                            .get(&tile.backward_edges[*i])
                            .map_or(false, |matches| matches.len() == 1))
                })
                .count()
                == 2
            {
                Some(tile.id)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tile_from_lines() {
        assert_eq!(
            Tile::from_lines(vec![
                "Tile 3079:".to_string(),
                "#.#.#####.".to_string(),
                ".#..######".to_string(),
                "..#.......".to_string(),
                "######....".to_string(),
                "####.#..#.".to_string(),
                ".#...#.##.".to_string(),
                "#.#####.##".to_string(),
                "..#.###...".to_string(),
                "..#.......".to_string(),
                "..#.###...".to_string()
            ]),
            Tile::new(
                3079,
                vec![
                    vec![true, false, true, false, true, true, true, true, true, false],
                    vec![false, true, false, false, true, true, true, true, true, true],
                    vec![false, false, true, false, false, false, false, false, false, false],
                    vec![true, true, true, true, true, true, false, false, false, false],
                    vec![true, true, true, true, false, true, false, false, true, false],
                    vec![false, true, false, false, false, true, false, true, true, false],
                    vec![true, false, true, true, true, true, true, false, true, true],
                    vec![false, false, true, false, true, true, true, false, false, false],
                    vec![false, false, true, false, false, false, false, false, false, false],
                    vec![false, false, true, false, true, true, true, false, false, false]
                ],
                hashset!(7, 2, 4, 4),
                vec!(0b1010111110, 0b0100001000, 0b0010111000, 0b1001101000),
                vec!(0b0111110101, 0b0001000010, 0b0001110100, 0b0001011001)
            )
        )
    }

    #[test]
    fn test_find_corners() {
        assert_eq!(
            find_corners(vec![
                Rc::new(Tile::from_lines(vec![
                    "Tile 2311:".to_string(),
                    "..##.#..#.".to_string(),
                    "##..#.....".to_string(),
                    "#...##..#.".to_string(),
                    "####.#...#".to_string(),
                    "##.##.###.".to_string(),
                    "##...#.###".to_string(),
                    ".#.#.#..##".to_string(),
                    "..#....#..".to_string(),
                    "###...#.#.".to_string(),
                    "..###..###".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 1951:".to_string(),
                    "#.##...##.".to_string(),
                    "#.####...#".to_string(),
                    ".....#..##".to_string(),
                    "#...######".to_string(),
                    ".##.#....#".to_string(),
                    ".###.#####".to_string(),
                    "###.##.##.".to_string(),
                    ".###....#.".to_string(),
                    "..#.#..#.#".to_string(),
                    "#...##.#..".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 1171:".to_string(),
                    "####...##.".to_string(),
                    "#..##.#..#".to_string(),
                    "##.#..#.#.".to_string(),
                    ".###.####.".to_string(),
                    "..###.####".to_string(),
                    ".##....##.".to_string(),
                    ".#...####.".to_string(),
                    "#.##.####.".to_string(),
                    "####..#...".to_string(),
                    ".....##...".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 1427:".to_string(),
                    "###.##.#..".to_string(),
                    ".#..#.##..".to_string(),
                    ".#.##.#..#".to_string(),
                    "#.#.#.##.#".to_string(),
                    "....#...##".to_string(),
                    "...##..##.".to_string(),
                    "...#.#####".to_string(),
                    ".#.####.#.".to_string(),
                    "..#..###.#".to_string(),
                    "..##.#..#.".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 1489:".to_string(),
                    "##.#.#....".to_string(),
                    "..##...#..".to_string(),
                    ".##..##...".to_string(),
                    "..#...#...".to_string(),
                    "#####...#.".to_string(),
                    "#..#.#.#.#".to_string(),
                    "...#.#.#..".to_string(),
                    "##.#...##.".to_string(),
                    "..##.##.##".to_string(),
                    "###.##.#..".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 2473:".to_string(),
                    "#....####.".to_string(),
                    "#..#.##...".to_string(),
                    "#.##..#...".to_string(),
                    "######.#.#".to_string(),
                    ".#...#.#.#".to_string(),
                    ".#########".to_string(),
                    ".###.#..#.".to_string(),
                    "########.#".to_string(),
                    "##...##.#.".to_string(),
                    "..###.#.#.".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 2971:".to_string(),
                    "..#.#....#".to_string(),
                    "#...###...".to_string(),
                    "#.#.###...".to_string(),
                    "##.##..#..".to_string(),
                    ".#####..##".to_string(),
                    ".#..####.#".to_string(),
                    "#..#.#..#.".to_string(),
                    "..####.###".to_string(),
                    "..#.#.###.".to_string(),
                    "...#.#.#.#".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 2729:".to_string(),
                    "...#.#.#.#".to_string(),
                    "####.#....".to_string(),
                    "..#.#.....".to_string(),
                    "....#..#.#".to_string(),
                    ".##..##.#.".to_string(),
                    ".#.####...".to_string(),
                    "####.#.#..".to_string(),
                    "##.####...".to_string(),
                    "##..#.##..".to_string(),
                    "#.##...##.".to_string(),
                ])),
                Rc::new(Tile::from_lines(vec![
                    "Tile 3079:".to_string(),
                    "#.#.#####.".to_string(),
                    ".#..######".to_string(),
                    "..#.......".to_string(),
                    "######....".to_string(),
                    "####.#..#.".to_string(),
                    ".#...#.##.".to_string(),
                    "#.#####.##".to_string(),
                    "..#.###...".to_string(),
                    "..#.......".to_string(),
                    "..#.###...".to_string(),
                ]))
            ]),
            hashset!(1951, 3079, 1171, 2971)
        )
    }
}
