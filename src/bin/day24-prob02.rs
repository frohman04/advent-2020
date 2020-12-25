#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let directions = std::fs::read_to_string("src/bin/day24.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| parse_directions(val))
                .collect::<Vec<Vec<Direction>>>()
        })
        .expect("Unable to open file");
    println!("{:?}", run_days(directions, 100));
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

fn parse_directions(raw: &str) -> Vec<Direction> {
    let chars = raw.chars().collect::<Vec<char>>();
    let mut directions: Vec<Direction> = Vec::new();
    let mut in_north = false;
    let mut in_south = false;
    for char in chars {
        if !in_north && !in_south {
            match char {
                'e' => directions.push(Direction::East),
                'w' => directions.push(Direction::West),
                'n' => in_north = true,
                's' => in_south = true,
                x => unreachable!("Unknown direction '{}'", x),
            }
        } else if in_north {
            match char {
                'e' => directions.push(Direction::NorthEast),
                'w' => directions.push(Direction::NorthWest),
                x => unreachable!("Unknown direction 'n{}'", x),
            }
            in_north = false;
        } else if in_south {
            match char {
                'e' => directions.push(Direction::SouthEast),
                'w' => directions.push(Direction::SouthWest),
                x => unreachable!("Unknown direction 's{}'", x),
            }
            in_south = false;
        } else {
            unreachable!("How???")
        }
    }
    directions
}

fn run_days(directions: Vec<Vec<Direction>>, num_days: usize) -> usize {
    // println!(
    //     "{:?}",
    //     calc_initial_state(directions.clone())
    //         .into_iter()
    //         .sorted()
    //         .collect::<Vec<(i32, i32)>>()
    // );
    (0..num_days)
        .fold(calc_initial_state(directions), |tiles, _| {
            // println!(
            //     "{:?}",
            //     tiles
            //         .clone()
            //         .into_iter()
            //         .sorted()
            //         .collect::<Vec<(i32, i32)>>()
            // );
            calc_next_day(tiles)
        })
        .iter()
        .count()
}

fn calc_initial_state(directions: Vec<Vec<Direction>>) -> HashSet<(i32, i32)> {
    let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();
    for direction in directions {
        let val = tiles.entry(calc_target(direction)).or_default();
        *val = !*val;
    }
    tiles
        .into_iter()
        .filter_map(|(coord, val)| if val { Some(coord) } else { None })
        .collect()
}

fn calc_next_day(tiles: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut out: HashSet<(i32, i32)> = HashSet::new();

    for point in tiles.iter() {
        // println!("Checking black tile {:?}", point.clone());
        if next_state(&tiles, point) {
            // println!("  turning on");
            out.insert(point.clone());
        }
        for npoint in get_neighbors(point) {
            // println!("Checking white neighbor {:?}", npoint.clone());
            if !tiles.contains(&npoint) && next_state(&tiles, &npoint) {
                // println!("  turning on");
                out.insert(npoint.clone());
            }
        }
    }

    out
}

fn next_state(tiles: &HashSet<(i32, i32)>, point: &(i32, i32)) -> bool {
    let count = get_neighbors(point)
        .into_iter()
        .filter(|npoint| tiles.contains(npoint))
        .count();
    // println!("  count: {}", count);
    if tiles.contains(point) {
        count == 1 || count == 2
    } else {
        count == 2
    }
}

fn get_neighbors(point: &(i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = point;

    let mut offsets = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
    if (point.1 % 2i32).abs() == 0 {
        offsets.push((-1, 1));
        offsets.push((-1, -1));
    } else {
        offsets.push((1, 1));
        offsets.push((1, -1));
    }

    offsets
        .into_iter()
        .map(|(x_off, y_off)| (*x + x_off, *y + y_off))
        .collect()
}

fn calc_target(direction: Vec<Direction>) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    for d in direction {
        match d {
            Direction::East => x += 1,
            Direction::SouthEast => {
                y += 1;
                if (y % 2i32).abs() == 0 {
                    x += 1;
                }
            }
            Direction::SouthWest => {
                y += 1;
                if (y % 2i32).abs() == 1 {
                    x -= 1;
                }
            }
            Direction::West => x -= 1,
            Direction::NorthWest => {
                y -= 1;
                if (y % 2i32).abs() == 1 {
                    x -= 1;
                }
            }
            Direction::NorthEast => {
                y -= 1;
                if (y % 2i32).abs() == 0 {
                    x += 1;
                }
            }
        }
    }

    (x, y)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_directions() {
        assert_eq!(
            parse_directions("eseswwnwne"),
            vec![
                Direction::East,
                Direction::SouthEast,
                Direction::SouthWest,
                Direction::West,
                Direction::NorthWest,
                Direction::NorthEast
            ]
        )
    }

    #[test]
    fn test_calc_target1() {
        assert_eq!(
            calc_target(vec![Direction::East, Direction::SouthEast, Direction::West]),
            (0, 1)
        )
    }

    #[test]
    fn test_calc_target2() {
        assert_eq!(
            calc_target(vec![
                Direction::NorthWest,
                Direction::West,
                Direction::SouthWest,
                Direction::East,
                Direction::East
            ]),
            (0, 0)
        )
    }

    #[test]
    fn test_calc_target_e() {
        assert_eq!(calc_target(vec![Direction::East]), (1, 0))
    }

    #[test]
    fn test_calc_target_w() {
        assert_eq!(calc_target(vec![Direction::West]), (-1, 0))
    }

    #[test]
    fn test_calc_target_se() {
        assert_eq!(calc_target(vec![Direction::SouthEast]), (0, 1))
    }

    #[test]
    fn test_calc_target_sw() {
        assert_eq!(calc_target(vec![Direction::SouthWest]), (-1, 1))
    }

    #[test]
    fn test_calc_target_ne() {
        assert_eq!(calc_target(vec![Direction::NorthEast]), (0, -1))
    }

    #[test]
    fn test_calc_target_nw() {
        assert_eq!(calc_target(vec![Direction::NorthWest]), (-1, -1))
    }

    #[test]
    fn test_calc_target_e_e() {
        assert_eq!(calc_target(vec![Direction::East, Direction::East]), (2, 0))
    }

    #[test]
    fn test_calc_target_w_w() {
        assert_eq!(calc_target(vec![Direction::West, Direction::West]), (-2, 0))
    }

    #[test]
    fn test_calc_target_se_se() {
        assert_eq!(
            calc_target(vec![Direction::SouthEast, Direction::SouthEast]),
            (1, 2)
        )
    }

    #[test]
    fn test_calc_target_sw_sw() {
        assert_eq!(
            calc_target(vec![Direction::SouthWest, Direction::SouthWest]),
            (-1, 2)
        )
    }

    #[test]
    fn test_calc_target_ne_ne() {
        assert_eq!(
            calc_target(vec![Direction::NorthEast, Direction::NorthEast]),
            (1, -2)
        )
    }

    #[test]
    fn test_calc_target_nw_nw() {
        assert_eq!(
            calc_target(vec![Direction::NorthWest, Direction::NorthWest]),
            (-1, -2)
        )
    }

    #[test]
    fn test_get_neighbors_even() {
        assert_eq!(
            get_neighbors(&(3, 2))
                .into_iter()
                .collect::<HashSet<(i32, i32)>>(),
            vec![(4, 2), (3, 3), (2, 3), (2, 2), (2, 1), (3, 1)]
                .into_iter()
                .collect::<HashSet<(i32, i32)>>()
        )
    }

    #[test]
    fn test_get_neighbors_odd() {
        assert_eq!(
            get_neighbors(&(3, 3))
                .into_iter()
                .collect::<HashSet<(i32, i32)>>(),
            vec![(4, 3), (4, 4), (3, 4), (2, 3), (3, 2), (4, 2)]
                .into_iter()
                .collect::<HashSet<(i32, i32)>>()
        )
    }

    #[test]
    fn test_calc_next_day() {
        assert_eq!(
            calc_next_day(hashset!(
                (-2, -1),
                (-2, 0),
                (-2, 1),
                (-2, 2),
                (-2, 3),
                (-1, -1),
                (0, 0),
                (1, -3),
                (1, 2),
                (2, 0)
            )),
            hashset!(
                (-3, 3),
                (-2, 3),
                (-2, 2),
                (-2, 1),
                (-1, 1),
                (0, 1),
                (1, 1),
                (-2, 0),
                (0, 0),
                (1, 0),
                (-3, -1),
                (-2, -1),
                (-1, -1),
                (0, -1),
                (-1, -2)
            )
        )
    }

    #[test]
    fn test_run_days_1() {
        assert_eq!(
            run_days(
                vec![
                    parse_directions("sesenwnenenewseeswwswswwnenewsewsw"),
                    parse_directions("neeenesenwnwwswnenewnwwsewnenwseswesw"),
                    parse_directions("seswneswswsenwwnwse"),
                    parse_directions("nwnwneseeswswnenewneswwnewseswneseene"),
                    parse_directions("swweswneswnenwsewnwneneseenw"),
                    parse_directions("eesenwseswswnenwswnwnwsewwnwsene"),
                    parse_directions("sewnenenenesenwsewnenwwwse"),
                    parse_directions("wenwwweseeeweswwwnwwe"),
                    parse_directions("wsweesenenewnwwnwsenewsenwwsesesenwne"),
                    parse_directions("neeswseenwwswnwswswnw"),
                    parse_directions("nenwswwsewswnenenewsenwsenwnesesenew"),
                    parse_directions("enewnwewneswsewnwswenweswnenwsenwsw"),
                    parse_directions("sweneswneswneneenwnewenewwneswswnese"),
                    parse_directions("swwesenesewenwneswnwwneseswwne"),
                    parse_directions("enesenwswwswneneswsenwnewswseenwsese"),
                    parse_directions("wnwnesenesenenwwnenwsewesewsesesew"),
                    parse_directions("nenewswnwewswnenesenwnesewesw"),
                    parse_directions("eneswnwswnwsenenwnwnwwseeswneewsenese"),
                    parse_directions("neswnwewnwnwseenwseesewsenwsweewe"),
                    parse_directions("wseweeenwnesenwwwswnew")
                ],
                1
            ),
            15
        )
    }

    #[test]
    fn test_run_days_2() {
        assert_eq!(
            run_days(
                vec![
                    parse_directions("sesenwnenenewseeswwswswwnenewsewsw"),
                    parse_directions("neeenesenwnwwswnenewnwwsewnenwseswesw"),
                    parse_directions("seswneswswsenwwnwse"),
                    parse_directions("nwnwneseeswswnenewneswwnewseswneseene"),
                    parse_directions("swweswneswnenwsewnwneneseenw"),
                    parse_directions("eesenwseswswnenwswnwnwsewwnwsene"),
                    parse_directions("sewnenenenesenwsewnenwwwse"),
                    parse_directions("wenwwweseeeweswwwnwwe"),
                    parse_directions("wsweesenenewnwwnwsenewsenwwsesesenwne"),
                    parse_directions("neeswseenwwswnwswswnw"),
                    parse_directions("nenwswwsewswnenenewsenwsenwnesesenew"),
                    parse_directions("enewnwewneswsewnwswenweswnenwsenwsw"),
                    parse_directions("sweneswneswneneenwnewenewwneswswnese"),
                    parse_directions("swwesenesewenwneswnwwneseswwne"),
                    parse_directions("enesenwswwswneneswsenwnewswseenwsese"),
                    parse_directions("wnwnesenesenenwwnenwsewesewsesesew"),
                    parse_directions("nenewswnwewswnenesenwnesewesw"),
                    parse_directions("eneswnwswnwsenenwnwnwwseeswneewsenese"),
                    parse_directions("neswnwewnwnwseenwseesewsenwsweewe"),
                    parse_directions("wseweeenwnesenwwwswnew")
                ],
                2
            ),
            12
        )
    }

    #[test]
    fn test_run_days_3() {
        assert_eq!(
            run_days(
                vec![
                    parse_directions("sesenwnenenewseeswwswswwnenewsewsw"),
                    parse_directions("neeenesenwnwwswnenewnwwsewnenwseswesw"),
                    parse_directions("seswneswswsenwwnwse"),
                    parse_directions("nwnwneseeswswnenewneswwnewseswneseene"),
                    parse_directions("swweswneswnenwsewnwneneseenw"),
                    parse_directions("eesenwseswswnenwswnwnwsewwnwsene"),
                    parse_directions("sewnenenenesenwsewnenwwwse"),
                    parse_directions("wenwwweseeeweswwwnwwe"),
                    parse_directions("wsweesenenewnwwnwsenewsenwwsesesenwne"),
                    parse_directions("neeswseenwwswnwswswnw"),
                    parse_directions("nenwswwsewswnenenewsenwsenwnesesenew"),
                    parse_directions("enewnwewneswsewnwswenweswnenwsenwsw"),
                    parse_directions("sweneswneswneneenwnewenewwneswswnese"),
                    parse_directions("swwesenesewenwneswnwwneseswwne"),
                    parse_directions("enesenwswwswneneswsenwnewswseenwsese"),
                    parse_directions("wnwnesenesenenwwnenwsewesewsesesew"),
                    parse_directions("nenewswnwewswnenesenwnesewesw"),
                    parse_directions("eneswnwswnwsenenwnwnwwseeswneewsenese"),
                    parse_directions("neswnwewnwnwseenwseesewsenwsweewe"),
                    parse_directions("wseweeenwnesenwwwswnew")
                ],
                3
            ),
            25
        )
    }

    #[test]
    fn test_run_days_10() {
        assert_eq!(
            run_days(
                vec![
                    parse_directions("sesenwnenenewseeswwswswwnenewsewsw"),
                    parse_directions("neeenesenwnwwswnenewnwwsewnenwseswesw"),
                    parse_directions("seswneswswsenwwnwse"),
                    parse_directions("nwnwneseeswswnenewneswwnewseswneseene"),
                    parse_directions("swweswneswnenwsewnwneneseenw"),
                    parse_directions("eesenwseswswnenwswnwnwsewwnwsene"),
                    parse_directions("sewnenenenesenwsewnenwwwse"),
                    parse_directions("wenwwweseeeweswwwnwwe"),
                    parse_directions("wsweesenenewnwwnwsenewsenwwsesesenwne"),
                    parse_directions("neeswseenwwswnwswswnw"),
                    parse_directions("nenwswwsewswnenenewsenwsenwnesesenew"),
                    parse_directions("enewnwewneswsewnwswenweswnenwsenwsw"),
                    parse_directions("sweneswneswneneenwnewenewwneswswnese"),
                    parse_directions("swwesenesewenwneswnwwneseswwne"),
                    parse_directions("enesenwswwswneneswsenwnewswseenwsese"),
                    parse_directions("wnwnesenesenenwwnenwsewesewsesesew"),
                    parse_directions("nenewswnwewswnenesenwnesewesw"),
                    parse_directions("eneswnwswnwsenenwnwnwwseeswneewsenese"),
                    parse_directions("neswnwewnwnwseenwseesewsenwsweewe"),
                    parse_directions("wseweeenwnesenwwwswnew")
                ],
                10
            ),
            37
        )
    }

    // takes longer than I'd like for part of the normal test suite
    // #[test]
    // fn test_run_days_100() {
    //     assert_eq!(
    //         run_days(
    //             vec![
    //                 parse_directions("sesenwnenenewseeswwswswwnenewsewsw"),
    //                 parse_directions("neeenesenwnwwswnenewnwwsewnenwseswesw"),
    //                 parse_directions("seswneswswsenwwnwse"),
    //                 parse_directions("nwnwneseeswswnenewneswwnewseswneseene"),
    //                 parse_directions("swweswneswnenwsewnwneneseenw"),
    //                 parse_directions("eesenwseswswnenwswnwnwsewwnwsene"),
    //                 parse_directions("sewnenenenesenwsewnenwwwse"),
    //                 parse_directions("wenwwweseeeweswwwnwwe"),
    //                 parse_directions("wsweesenenewnwwnwsenewsenwwsesesenwne"),
    //                 parse_directions("neeswseenwwswnwswswnw"),
    //                 parse_directions("nenwswwsewswnenenewsenwsenwnesesenew"),
    //                 parse_directions("enewnwewneswsewnwswenweswnenwsenwsw"),
    //                 parse_directions("sweneswneswneneenwnewenewwneswswnese"),
    //                 parse_directions("swwesenesewenwneswnwwneseswwne"),
    //                 parse_directions("enesenwswwswneneswsenwnewswseenwsese"),
    //                 parse_directions("wnwnesenesenenwwnenwsewesewsesesew"),
    //                 parse_directions("nenewswnwewswnenesenwnesewesw"),
    //                 parse_directions("eneswnwswnwsenenwnwnwwseeswneewsenese"),
    //                 parse_directions("neswnwewnwnwseenwseesewsenwsweewe"),
    //                 parse_directions("wseweeenwnesenwwwswnew")
    //             ],
    //             100
    //         ),
    //         2208
    //     )
    // }
}
