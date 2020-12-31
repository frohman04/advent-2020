#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use itertools::Itertools;
use std::cell::RefCell;
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
    println!("{:?}", assemble_tiles(tiles));
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

    pub fn from_pixels(id: usize, pixels: Vec<Vec<bool>>) -> Tile {
        let edges = {
            let top = pixels[0].clone();
            let bottom = pixels[pixels.len() - 1].clone().into_iter().rev().collect::<Vec<bool>>();
            let left = pixels
                .iter()
                .map(|row| row[0].clone())
                .rev()
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

        Tile::from_pixels(id, pixels)
    }

    /// Create a copy of this tile with the provided translations applied.  flip_horizontal flips
    /// the tile over on its y-axis.  rotate rotates the tile; must be a multiple of 90deg.
    pub fn translate(&self, flip_horizontal: bool, rotate: u16) -> Tile {
        let rotate = rotate % 360;

        let int_pixels = match flip_horizontal {
            true => {
                let mut new_pixels: Vec<Vec<bool>> = Vec::new();
                for y in 0..self.pixels.len() {
                    let mut new_row: Vec<bool> = Vec::new();
                    for x in (0..self.pixels[y].len()).rev() {
                        new_row.push(self.pixels[y][x]);
                    }
                    new_pixels.push(new_row);
                }
                new_pixels
            }
            false => self.pixels.clone(),
        };

        let mut new_pixels: Vec<Vec<bool>> = Vec::new();
        match rotate {
            0 => new_pixels = int_pixels,
            90 => {
                for y in 0..int_pixels.len() {
                    let mut new_row: Vec<bool> = Vec::new();
                    for x in (0..int_pixels[y].len()).rev() {
                        new_row.push(int_pixels[x][y]);
                    }
                    new_pixels.push(new_row);
                }
            }
            180 => {
                for y in (0..int_pixels.len()).rev() {
                    let mut new_row: Vec<bool> = Vec::new();
                    for x in (0..int_pixels[y].len()).rev() {
                        new_row.push(int_pixels[y][x]);
                    }
                    new_pixels.push(new_row);
                }
            }
            270 => {
                for y in (0..int_pixels.len()).rev() {
                    let mut new_row: Vec<bool> = Vec::new();
                    for x in 0..int_pixels[y].len() {
                        new_row.push(int_pixels[x][y]);
                    }
                    new_pixels.push(new_row);
                }
            }
            r => panic!("Invalid rotation requested: {}", r),
        }

        Tile::from_pixels(self.id, new_pixels)
    }
}

fn assemble_tiles(tiles: Vec<Rc<Tile>>) -> Vec<Vec<Rc<Tile>>> {
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

    let corners = get_tiles_by_matching_edges(&tiles, &cache, 2);
    let mut tiles_by_id = tiles
        .into_iter()
        .map(|tile| (tile.id, tile.clone()))
        .collect::<HashMap<usize, Rc<Tile>>>();

    let mut out: Vec<Vec<Rc<Tile>>> = Vec::new();

    while !tiles_by_id.is_empty() {
        let first_tile = if out.is_empty() {
            // select a corner tile to the the top-left tile and rotate it to fit that position
            let corner = tiles_by_id
                .get(corners.iter().sorted().next().unwrap())
                .unwrap();
            match (
                is_unmatched_edge(corner, &0, &cache),
                is_unmatched_edge(corner, &1, &cache),
                is_unmatched_edge(corner, &2, &cache),
                is_unmatched_edge(corner, &3, &cache),
            ) {
                (true, true, false, false) => Rc::new(corner.translate(false, 270)),
                (false, true, true, false) => Rc::new(corner.translate(false, 180)),
                (false, false, true, true) => Rc::new(corner.translate(false, 90)),
                (true, false, false, true) => corner.clone(),
                _ => unreachable!(),
            }
        } else {
            find_next_tile(&out.last().unwrap()[0], 2, &cache, &mut tiles_by_id)
        };

        println!("------------------------");
        println!(" !! Starting new row !!");
        println!("------------------------");

        let mut current: Vec<Rc<Tile>> = Vec::new();
        current.push(first_tile.clone());

        while has_match(&current.last().unwrap().forward_edges, &1, &cache) ||
            has_match(&current.last().unwrap().backward_edges, &1, &cache)
        {
            let next_tile = find_next_tile(&current.last().unwrap(), 1, &cache, &mut tiles_by_id);
            current.push(next_tile);
        }

        out.push(current);
    }

    out
}

fn find_next_tile(
    curr_tile: &Rc<Tile>,
    edge_id: usize,
    cache: &HashMap<u16, HashSet<usize>>,
    tiles_by_id: &mut HashMap<usize, Rc<Tile>>,
) -> Rc<Tile> {
    println!("Next tile for {:?} edge {}", curr_tile, edge_id);

    let flip = has_match(&curr_tile.backward_edges, &edge_id, cache);
    println!("  Should flip? {}", flip);

    let next_tile = {
        let next_tile_id = if !flip {
            cache
                .get(&curr_tile.forward_edges[edge_id])
                .unwrap()
                .iter()
                .filter(|e| **e != curr_tile.id as usize)
                .next()
                .unwrap()
                .clone()
        } else {
            cache
                .get(&curr_tile.backward_edges[edge_id])
                .unwrap()
                .iter()
                .filter(|e| **e != curr_tile.id as usize)
                .next()
                .unwrap()
                .clone()
        };
        println!("  ID of next tile: {}", next_tile_id);
        tiles_by_id.remove(&next_tile_id).unwrap().clone()
    };

    let rotate = if !flip {
        next_tile
            .forward_edges
            .iter()
            .enumerate()
            .filter(|(_, e)| **e != curr_tile.forward_edges[edge_id])
            .map(|(i, _)| i)
            .next()
            .unwrap()
    } else {
        next_tile
            .backward_edges
            .iter()
            .enumerate()
            .filter(|(_, e)| **e != curr_tile.backward_edges[edge_id])
            .map(|(i, _)| i)
            .next()
            .unwrap()
    };
    let rotate = ((rotate as i16 - 2) % 4).abs() as u16;

    Rc::new(next_tile.translate(flip, rotate * 90))
}

fn has_match(
    edge: &Vec<u16>,
    edge_id: &usize,
    cache: &HashMap<u16, HashSet<usize>>
) -> bool {
    cache.get(&edge[*edge_id]).map_or(false, |ids| ids.len() > 1)
}

fn is_unmatched_edge(
    tile: &Rc<Tile>,
    edge_i: &usize,
    cache: &HashMap<u16, HashSet<usize>>,
) -> bool {
    !has_match(&tile.forward_edges, edge_i, cache) && !has_match(&tile.backward_edges, edge_i, cache)
}

fn get_tiles_by_matching_edges(
    tiles: &Vec<Rc<Tile>>,
    cache: &HashMap<u16, HashSet<usize>>,
    matched_edges: usize,
) -> HashSet<usize> {
    tiles
        .iter()
        .filter_map(|tile| {
            if (0usize..4)
                .filter(|i| is_unmatched_edge(tile, i, cache))
                .count()
                == matched_edges
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
                vec!(0b1010111110, 0b0100001000, 0b0001110100, 0b0001011001),
                vec!(0b0111110101, 0b0001000010, 0b0010111000, 0b1001101000)
            )
        )
    }

    #[test]
    fn test_tile_translate_noop() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(false, 0),
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_rot90() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(false, 90),
            Tile::from_pixels(
                42,
                vec![
                    vec![false, true, true],
                    vec![false, false, false],
                    vec![true, false, true]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_rot180() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(false, 180),
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, false],
                    vec![false, false, true],
                    vec![true, false, true]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_rot270() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(false, 270),
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![false, false, false],
                    vec![true, true, false]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_flip() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(true, 0),
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![false, false, true],
                    vec![true, false, false]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_flip_rot90() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(true, 90),
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![false, false, false],
                    vec![false, true, true]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_flip_rot180() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(true, 180),
            Tile::from_pixels(
                42,
                vec![
                    vec![false, false, true],
                    vec![true, false, false],
                    vec![true, false, true]
                ]
            )
        )
    }

    #[test]
    fn test_tile_translate_flip_rot270() {
        assert_eq!(
            Tile::from_pixels(
                42,
                vec![
                    vec![true, false, true],
                    vec![true, false, false],
                    vec![false, false, true]
                ]
            )
            .translate(true, 270),
            Tile::from_pixels(
                42,
                vec![
                    vec![true, true, false],
                    vec![false, false, false],
                    vec![true, false, true]
                ]
            )
        )
    }

    #[test]
    fn test_assemble_tiles() {
        assert_eq!(
            assemble_tiles(vec![
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
            vec![
                vec![
                    Rc::new(Tile::from_lines(vec![
                        "Tile 1171:".to_string(),
                        ".....##...".to_string(),
                        "####..#...".to_string(),
                        "#.##.####.".to_string(),
                        ".#...####.".to_string(),
                        ".##....##.".to_string(),
                        "..###.####".to_string(),
                        ".###.####.".to_string(),
                        "##.#..#.#.".to_string(),
                        "#..##.#..#".to_string(),
                        "####...##.".to_string(),
                    ])),
                    Rc::new(Tile::from_lines(vec![
                        "Tile 1489:".to_string(),
                        "....#.#.##".to_string(),
                        "..#...##..".to_string(),
                        "...##..##.".to_string(),
                        "...#...#..".to_string(),
                        ".#...#####".to_string(),
                        "#.#.#.#..#".to_string(),
                        "..#.#.#...".to_string(),
                        ".##...#.##".to_string(),
                        "##.##.##..".to_string(),
                        "..#.##.###".to_string(),
                    ])),
                    Rc::new(Tile::from_lines(vec![
                        "Tile 2971:".to_string(),
                        "#....#.#..".to_string(),
                        "...###...#".to_string(),
                        "...###.#.#".to_string(),
                        "..#..##.##".to_string(),
                        "##..#####.".to_string(),
                        "#.####..#.".to_string(),
                        ".#..#.#..#".to_string(),
                        "###.####..".to_string(),
                        ".###.#.#..".to_string(),
                        "#.#.#.#...".to_string()
                    ]))
                ],
                vec![
                    Rc::new(Tile::from_lines(vec![
                        "Tile 2473:".to_string(),
                        "####...##.".to_string(),
                        "...######.".to_string(),
                        "..##.###.#".to_string(),
                        ".###.###.#".to_string(),
                        "...#.#.#.#".to_string(),
                        "##.######.".to_string(),
                        "###..#.###".to_string(),
                        "#..###.#..".to_string(),
                        "#....##.##".to_string(),
                        "...###.#..".to_string(),
                    ])),
                    Rc::new(Tile::from_lines(vec![
                        "Tile 1427:".to_string(),
                        "..#.##.###".to_string(),
                        "..##.#..#.".to_string(),
                        "#..#.##.#.".to_string(),
                        "#.##.#.#.#".to_string(),
                        "##...#....".to_string(),
                        ".##..##...".to_string(),
                        "#####.#...".to_string(),
                        ".#.####.#.".to_string(),
                        "#.###..#..".to_string(),
                        ".#..#.##..".to_string(),
                    ])),
                    Rc::new(Tile::from_lines(vec![
                        "Tile 2729:".to_string(),
                        "#.#.#.#...".to_string(),
                        "....#.####".to_string(),
                        ".....#.#..".to_string(),
                        "#.#..#....".to_string(),
                        ".#.##..##.".to_string(),
                        "...####.#.".to_string(),
                        "..#.#.####".to_string(),
                        "...####.##".to_string(),
                        "..##.#..##".to_string(),
                        ".##...##.#".to_string(),
                    ]))
                ],
                vec![
                    Rc::new(Tile::from_lines(vec![
                        "Tile 3079:".to_string(),
                        "...###.#..".to_string(),
                        ".......#..".to_string(),
                        "...###.#..".to_string(),
                        "##.#####.#".to_string(),
                        ".##.#...#.".to_string(),
                        ".#..#.####".to_string(),
                        "....######".to_string(),
                        ".......#..".to_string(),
                        "######..#.".to_string(),
                        ".#####.#.#".to_string()
                    ])),
                    Rc::new(Tile::from_lines(vec![
                        "Tile 2311:".to_string(),
                        ".#..#.##..".to_string(),
                        ".....#..##".to_string(),
                        ".#..##...#".to_string(),
                        "#...#.####".to_string(),
                        ".###.##.##".to_string(),
                        "###.#...##".to_string(),
                        "##..#.#.#.".to_string(),
                        "..#....#..".to_string(),
                        ".#.#...###".to_string(),
                        "###..###..".to_string(),
                    ])),
                    Rc::new(Tile::from_lines(vec![
                        "Tile 1951:".to_string(),
                        ".##...##.#".to_string(),
                        "#...####.#".to_string(),
                        "##..#.....".to_string(),
                        "######...#".to_string(),
                        "#....#.##.".to_string(),
                        "#####.###.".to_string(),
                        ".##.##.###".to_string(),
                        ".#....###.".to_string(),
                        "#.#..#.#..".to_string(),
                        "..#.##...#".to_string()
                    ]))
                ]
            ]
        )
    }
}
