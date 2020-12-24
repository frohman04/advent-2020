use std::collections::HashMap;

fn main() {
    let directions = std::fs::read_to_string("src/bin/day24.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| parse_directions(val))
                .collect::<Vec<Vec<Direction>>>()
        })
        .expect("Unable to open file");
    println!("{:?}", flip_tiles(directions));
}

#[derive(Debug, PartialEq)]
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

fn flip_tiles(directions: Vec<Vec<Direction>>) -> usize {
    let mut tiles: HashMap<(i32, i32), bool> = HashMap::new();
    for direction in directions {
        let val = tiles.entry(calc_target(direction)).or_default();
        *val = !*val;
    }
    tiles.values().into_iter().filter(|val| **val).count()
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
    fn test_flip_tiles() {
        assert_eq!(
            flip_tiles(vec![
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
            ]),
            10
        )
    }
}
