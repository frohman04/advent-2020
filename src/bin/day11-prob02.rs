use std::cmp::min;

fn main() {
    let spaces = std::fs::read_to_string("src/bin/day11.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| parse_line(val))
                .collect::<Vec<Vec<Space>>>()
        })
        .expect("Unable to open file");
    println!("{:?}", find_stable(spaces));
}

#[derive(Debug, PartialEq)]
enum Space {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

fn parse_line(line: &str) -> Vec<Space> {
    line.chars()
        .map(|c| match c {
            '.' => Space::Floor,
            'L' => Space::EmptySeat,
            '#' => Space::OccupiedSeat,
            a => panic!("Unable to parse space '{}'", a),
        })
        .collect()
}

fn find_stable(spaces: Vec<Vec<Space>>) -> usize {
    let mut prev: Vec<Vec<Space>> = Vec::new();
    let mut curr = spaces;
    while prev != curr {
        prev = curr;
        curr = step(&prev);
    }
    curr.iter()
        .map(|row| {
            row.iter()
                .filter(|space| **space == Space::OccupiedSeat)
                .count()
        })
        .sum()
}

fn count_occupied_neighbors(spaces: &Vec<Vec<Space>>, i: usize, j: usize) -> usize {
    let neighbor_offsets: Vec<(i8, i8)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let mut count = 0usize;
    // println!("({:#?}, {:#?})", i, j);
    for (i_off, j_off) in neighbor_offsets {
        let max_mux: i32 = min(
            if i_off < 0 {
                i as i32 / (-1 * i_off) as i32
            } else if i_off > 0 {
                (spaces.len() - i - 1) as i32 / i_off as i32
            } else {
                i32::max_value()
            },
            if j_off < 0 {
                j as i32 / (-1 * j_off) as i32
            } else if j_off > 0 {
                (spaces[i].len() - j - 1) as i32 / j_off as i32
            } else {
                i32::max_value()
            },
        );
        // println!("  ({:#?}, {:#?}) -> max mux {:#?}", i_off, j_off, max_mux);
        for mux in 1..=max_mux {
            // println!(
            //     "    spaces[{:?}][{:?}] => {:?}",
            //     (i as i32 + (i_off as i32 * mux)) as usize,
            //     (j as i32 + (j_off as i32 * mux)) as usize,
            //     spaces[(i as i32 + (i_off as i32 * mux)) as usize]
            //         [(j as i32 + (j_off as i32 * mux)) as usize]
            // );
            match spaces[(i as i32 + (i_off as i32 * mux)) as usize]
                [(j as i32 + (j_off as i32 * mux)) as usize]
            {
                Space::Floor => continue,
                Space::EmptySeat => break,
                Space::OccupiedSeat => {
                    count += 1;
                    break;
                }
            }
        }
    }

    count
}

fn step(spaces: &Vec<Vec<Space>>) -> Vec<Vec<Space>> {
    spaces
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, space)| match space {
                    Space::Floor => Space::Floor,
                    Space::OccupiedSeat => {
                        let occupied_neighbor_count = count_occupied_neighbors(&spaces, i, j);
                        if occupied_neighbor_count >= 5 {
                            Space::EmptySeat
                        } else {
                            Space::OccupiedSeat
                        }
                    }
                    Space::EmptySeat => {
                        let occupied_neighbor_count = count_occupied_neighbors(&spaces, i, j);
                        if occupied_neighbor_count == 0 {
                            Space::OccupiedSeat
                        } else {
                            Space::EmptySeat
                        }
                    }
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("#.LL.L#.##"),
            vec![
                Space::OccupiedSeat,
                Space::Floor,
                Space::EmptySeat,
                Space::EmptySeat,
                Space::Floor,
                Space::EmptySeat,
                Space::OccupiedSeat,
                Space::Floor,
                Space::OccupiedSeat,
                Space::OccupiedSeat
            ]
        )
    }

    #[test]
    fn test_count_occupied_neighbors1() {
        assert_eq!(
            count_occupied_neighbors(
                &vec![
                    parse_line(".......#."),
                    parse_line("...#....."),
                    parse_line(".#......."),
                    parse_line("........."),
                    parse_line("..#L....#"),
                    parse_line("....#...."),
                    parse_line("........."),
                    parse_line("#........"),
                    parse_line("...#.....")
                ],
                4,
                3
            ),
            8
        )
    }

    #[test]
    fn test_count_occupied_neighbors2() {
        assert_eq!(
            count_occupied_neighbors(
                &vec![
                    parse_line("............."),
                    parse_line(".L.L.#.#.#.#."),
                    parse_line(".............")
                ],
                1,
                1
            ),
            0
        )
    }

    #[test]
    fn test_count_occupied_neighbors3() {
        assert_eq!(
            count_occupied_neighbors(
                &vec![
                    parse_line(".##.##."),
                    parse_line("#.#.#.#"),
                    parse_line("##...##"),
                    parse_line("...L..."),
                    parse_line("##...##"),
                    parse_line("#.#.#.#"),
                    parse_line(".##.##.")
                ],
                3,
                3
            ),
            0
        )
    }

    #[test]
    fn test_step1() {
        assert_eq!(
            step(&vec![
                parse_line("L.LL.LL.LL"),
                parse_line("LLLLLLL.LL"),
                parse_line("L.L.L..L.."),
                parse_line("LLLL.LL.LL"),
                parse_line("L.LL.LL.LL"),
                parse_line("L.LLLLL.LL"),
                parse_line("..L.L....."),
                parse_line("LLLLLLLLLL"),
                parse_line("L.LLLLLL.L"),
                parse_line("L.LLLLL.LL")
            ]),
            vec![
                parse_line("#.##.##.##"),
                parse_line("#######.##"),
                parse_line("#.#.#..#.."),
                parse_line("####.##.##"),
                parse_line("#.##.##.##"),
                parse_line("#.#####.##"),
                parse_line("..#.#....."),
                parse_line("##########"),
                parse_line("#.######.#"),
                parse_line("#.#####.##")
            ]
        )
    }

    #[test]
    fn test_step2() {
        assert_eq!(
            step(&vec![
                parse_line("#.##.##.##"),
                parse_line("#######.##"),
                parse_line("#.#.#..#.."),
                parse_line("####.##.##"),
                parse_line("#.##.##.##"),
                parse_line("#.#####.##"),
                parse_line("..#.#....."),
                parse_line("##########"),
                parse_line("#.######.#"),
                parse_line("#.#####.##")
            ]),
            vec![
                parse_line("#.LL.LL.L#"),
                parse_line("#LLLLLL.LL"),
                parse_line("L.L.L..L.."),
                parse_line("LLLL.LL.LL"),
                parse_line("L.LL.LL.LL"),
                parse_line("L.LLLLL.LL"),
                parse_line("..L.L....."),
                parse_line("LLLLLLLLL#"),
                parse_line("#.LLLLLL.L"),
                parse_line("#.LLLLL.L#")
            ]
        )
    }

    #[test]
    fn test_step3() {
        assert_eq!(
            step(&vec![
                parse_line("#.LL.LL.L#"),
                parse_line("#LLLLLL.LL"),
                parse_line("L.L.L..L.."),
                parse_line("LLLL.LL.LL"),
                parse_line("L.LL.LL.LL"),
                parse_line("L.LLLLL.LL"),
                parse_line("..L.L....."),
                parse_line("LLLLLLLLL#"),
                parse_line("#.LLLLLL.L"),
                parse_line("#.LLLLL.L#")
            ]),
            vec![
                parse_line("#.L#.##.L#"),
                parse_line("#L#####.LL"),
                parse_line("L.#.#..#.."),
                parse_line("##L#.##.##"),
                parse_line("#.##.#L.##"),
                parse_line("#.#####.#L"),
                parse_line("..#.#....."),
                parse_line("LLL####LL#"),
                parse_line("#.L#####.L"),
                parse_line("#.L####.L#")
            ]
        )
    }

    #[test]
    fn test_find_stable() {
        assert_eq!(
            find_stable(vec![
                parse_line("L.LL.LL.LL"),
                parse_line("LLLLLLL.LL"),
                parse_line("L.L.L..L.."),
                parse_line("LLLL.LL.LL"),
                parse_line("L.LL.LL.LL"),
                parse_line("L.LLLLL.LL"),
                parse_line("..L.L....."),
                parse_line("LLLLLLLLLL"),
                parse_line("L.LLLLLL.L"),
                parse_line("L.LLLLL.LL")
            ]),
            26
        )
    }
}
