#![allow(unused_imports)]

#[macro_use]
extern crate maplit;

use std::cell::RefCell;
use std::collections::HashSet;

fn main() {
    let cells_vec = std::fs::read_to_string("src/bin/day17.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.chars().map(|c| c == '#').collect::<Vec<bool>>())
                .collect::<Vec<Vec<bool>>>()
        })
        .expect("Unable to open file");

    let mut cells: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    for (x, row) in cells_vec.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell {
                cells.insert((x as i32, y as i32, 0, 0));
            }
        }
    }

    println!("{:?}", boot(cells));
}

fn boot(init_state: HashSet<(i32, i32, i32, i32)>) -> usize {
    let state = RefCell::new(init_state);
    for _ in 0..6 {
        state.replace_with(|old_state| run_cycle(old_state));
    }
    let count = state.borrow().iter().count();
    count
}

fn run_cycle(state: &HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
    let mut out: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    for point in state.iter() {
        if next_cell_state(&state, point) {
            out.insert(point.clone());
        }
        for npoint in get_neighbors(point) {
            if next_cell_state(&state, &npoint) {
                out.insert(npoint.clone());
            }
        }
    }

    out
}

fn next_cell_state(state: &HashSet<(i32, i32, i32, i32)>, point: &(i32, i32, i32, i32)) -> bool {
    let active_neighbors = get_neighbors(point)
        .into_iter()
        .filter_map(|(x, y, z, w)| {
            if state.contains(&(x, y, z, w)) {
                Some(())
            } else {
                None
            }
        })
        .count();
    let curr_is_active = state.contains(point);
    if curr_is_active && (active_neighbors == 2 || active_neighbors == 3) {
        true
    } else if !curr_is_active && active_neighbors == 3 {
        true
    } else {
        false
    }
}

fn get_neighbors(point: &(i32, i32, i32, i32)) -> Vec<(i32, i32, i32, i32)> {
    let (x, y, z, w) = point;
    let mut out: Vec<(i32, i32, i32, i32)> = Vec::new();
    for x_off in -1..=1 {
        for y_off in -1..=1 {
            for z_off in -1..=1 {
                for w_off in -1..=1 {
                    if !(x_off == 0 && y_off == 0 && z_off == 0 && w_off == 0) {
                        out.push((*x + x_off, *y + y_off, *z + z_off, *w + w_off));
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test_next_cell_state_active_true() {
        assert_eq!(
            next_cell_state(
                &hashset!((0, 0, 0, 0), (1, 0, 0, 0), (0, 1, 0, 0)),
                &(0, 0, 0, 0)
            ),
            true
        )
    }

    #[test]
    fn test_next_cell_state_active_false() {
        assert_eq!(
            next_cell_state(&hashset!((0, 0, 0, 0), (1, 0, 0, 0)), &(0, 0, 0, 0)),
            false
        )
    }

    #[test]
    fn test_next_cell_state_inactive_true() {
        assert_eq!(
            next_cell_state(
                &hashset!((1, 0, 0, 0), (0, 1, 0, 0), (1, 1, 0, 0)),
                &(0, 0, 0, 0)
            ),
            true
        )
    }

    #[test]
    fn test_next_cell_state_inactive_false() {
        assert_eq!(
            next_cell_state(&hashset!((1, 0, 0, 0), (0, 1, 0, 0)), &(0, 0, 0, 0)),
            false
        )
    }

    #[test]
    fn test_run_cycle1() {
        assert_eq!(
            run_cycle(&hashset!(
                (1, -1, 0, 0),
                (2, 0, 0, 0),
                (0, 1, 0, 0),
                (1, 1, 0, 0),
                (2, 1, 0, 0)
            )),
            hashset!(
                (0, 0, -1, -1),
                (2, 1, -1, -1),
                (1, 2, -1, -1),
                (0, 0, 0, -1),
                (2, 1, 0, -1),
                (1, 2, 0, -1),
                (0, 0, 1, -1),
                (2, 1, 1, -1),
                (1, 2, 1, -1),
                (0, 0, -1, 0),
                (2, 1, -1, 0),
                (1, 2, -1, 0),
                (0, 0, 0, 0),
                (2, 0, 0, 0),
                (1, 1, 0, 0),
                (2, 1, 0, 0),
                (1, 2, 0, 0),
                (0, 0, 1, 0),
                (2, 1, 1, 0),
                (1, 2, 1, 0),
                (0, 0, -1, 1),
                (2, 1, -1, 1),
                (1, 2, -1, 1),
                (0, 0, 0, 1),
                (2, 1, 0, 1),
                (1, 2, 0, 1),
                (0, 0, 1, 1),
                (2, 1, 1, 1),
                (1, 2, 1, 1),
            )
        )
    }

    #[test]
    fn test_run_cycle2() {
        println!(
            "{:?}",
            run_cycle(&hashset!(
                (0, 0, -1, -1),
                (2, 1, -1, -1),
                (1, 2, -1, -1),
                (0, 0, 0, -1),
                (2, 1, 0, -1),
                (1, 2, 0, -1),
                (0, 0, 1, -1),
                (2, 1, 1, -1),
                (1, 2, 1, -1),
                (0, 0, -1, 0),
                (2, 1, -1, 0),
                (1, 2, -1, 0),
                (0, 0, 0, 0),
                (2, 0, 0, 0),
                (1, 1, 0, 0),
                (2, 1, 0, 0),
                (1, 2, 0, 0),
                (0, 0, 1, 0),
                (2, 1, 1, 0),
                (1, 2, 1, 0),
                (0, 0, -1, 1),
                (2, 1, -1, 1),
                (1, 2, -1, 1),
                (0, 0, 0, 1),
                (2, 1, 0, 1),
                (1, 2, 0, 1),
                (0, 0, 1, 1),
                (2, 1, 1, 1),
                (1, 2, 1, 1),
            ))
            .iter()
            .sorted()
            .collect::<Vec<_>>()
        );
        println!(
            "{:?}",
            hashset!(
                (1, 1, -2, -2),
                (-1, -1, 0, -2),
                (0, -1, 0, -2),
                (1, -1, 0, -2),
                (-1, 0, 0, -2),
                (0, 0, 0, -2),
                (2, 0, 0, -2),
                (3, 0, 0, -2),
                (-1, 1, 0, -2),
                (3, 1, 0, -2),
                (0, 2, 0, -2),
                (3, 2, 0, -2),
                (0, 3, 0, -2),
                (1, 3, 0, -2),
                (2, 3, 0, -2),
                (1, 1, 2, -2),
                (-1, -1, -2, 0),
                (0, -1, -2, 0),
                (1, -1, -2, 0),
                (-1, 0, -2, 0),
                (0, 0, -2, 0),
                (2, 0, -2, 0),
                (3, 0, -2, 0),
                (-1, 1, -2, 0),
                (3, 1, -2, 0),
                (0, 2, -2, 0),
                (3, 2, -2, 0),
                (0, 3, -2, 0),
                (1, 3, -2, 0),
                (2, 3, -2, 0),
                (-1, -1, 2, 0),
                (0, -1, 2, 0),
                (1, -1, 2, 0),
                (-1, 0, 2, 0),
                (0, 0, 2, 0),
                (2, 0, 2, 0),
                (3, 0, 2, 0),
                (-1, 1, 2, 0),
                (3, 1, 2, 0),
                (0, 2, 2, 0),
                (3, 2, 2, 0),
                (0, 3, 2, 0),
                (1, 3, 2, 0),
                (2, 3, 2, 0),
                (1, 1, -2, 2),
                (-1, -1, 0, 2),
                (0, -1, 0, 2),
                (1, -1, 0, 2),
                (-1, 0, 0, 2),
                (0, 0, 0, 2),
                (2, 0, 0, 2),
                (3, 0, 0, 2),
                (-1, 1, 0, 2),
                (3, 1, 0, 2),
                (0, 2, 0, 2),
                (3, 2, 0, 2),
                (0, 3, 0, 2),
                (1, 3, 0, 2),
                (2, 3, 0, 2),
                (1, 1, 2, 2)
            )
            .iter()
            .sorted()
            .collect::<Vec<_>>()
        );
        assert_eq!(
            run_cycle(&hashset!(
                (0, 0, -1, -1),
                (2, 1, -1, -1),
                (1, 2, -1, -1),
                (0, 0, 0, -1),
                (2, 1, 0, -1),
                (1, 2, 0, -1),
                (0, 0, 1, -1),
                (2, 1, 1, -1),
                (1, 2, 1, -1),
                (0, 0, -1, 0),
                (2, 1, -1, 0),
                (1, 2, -1, 0),
                (0, 0, 0, 0),
                (2, 0, 0, 0),
                (1, 1, 0, 0),
                (2, 1, 0, 0),
                (1, 2, 0, 0),
                (0, 0, 1, 0),
                (2, 1, 1, 0),
                (1, 2, 1, 0),
                (0, 0, -1, 1),
                (2, 1, -1, 1),
                (1, 2, -1, 1),
                (0, 0, 0, 1),
                (2, 1, 0, 1),
                (1, 2, 0, 1),
                (0, 0, 1, 1),
                (2, 1, 1, 1),
                (1, 2, 1, 1),
            )),
            hashset!(
                (1, 1, -2, -2),
                (-1, -1, 0, -2),
                (0, -1, 0, -2),
                (1, -1, 0, -2),
                (-1, 0, 0, -2),
                (0, 0, 0, -2),
                (2, 0, 0, -2),
                (3, 0, 0, -2),
                (-1, 1, 0, -2),
                (3, 1, 0, -2),
                (0, 2, 0, -2),
                (3, 2, 0, -2),
                (0, 3, 0, -2),
                (1, 3, 0, -2),
                (2, 3, 0, -2),
                (1, 1, 2, -2),
                (-1, -1, -2, 0),
                (0, -1, -2, 0),
                (1, -1, -2, 0),
                (-1, 0, -2, 0),
                (0, 0, -2, 0),
                (2, 0, -2, 0),
                (3, 0, -2, 0),
                (-1, 1, -2, 0),
                (3, 1, -2, 0),
                (0, 2, -2, 0),
                (3, 2, -2, 0),
                (0, 3, -2, 0),
                (1, 3, -2, 0),
                (2, 3, -2, 0),
                (-1, -1, 2, 0),
                (0, -1, 2, 0),
                (1, -1, 2, 0),
                (-1, 0, 2, 0),
                (0, 0, 2, 0),
                (2, 0, 2, 0),
                (3, 0, 2, 0),
                (-1, 1, 2, 0),
                (3, 1, 2, 0),
                (0, 2, 2, 0),
                (3, 2, 2, 0),
                (0, 3, 2, 0),
                (1, 3, 2, 0),
                (2, 3, 2, 0),
                (1, 1, -2, 2),
                (-1, -1, 0, 2),
                (0, -1, 0, 2),
                (1, -1, 0, 2),
                (-1, 0, 0, 2),
                (0, 0, 0, 2),
                (2, 0, 0, 2),
                (3, 0, 0, 2),
                (-1, 1, 0, 2),
                (3, 1, 0, 2),
                (0, 2, 0, 2),
                (3, 2, 0, 2),
                (0, 3, 0, 2),
                (1, 3, 0, 2),
                (2, 3, 0, 2),
                (1, 1, 2, 2)
            )
        )
    }

    // long-running test that passes, but is disabled to improve overall test runtime
    // #[test]
    // fn test_boot() {
    //     assert_eq!(
    //         boot(hashset!(
    //             (1, -1, 0, 0),
    //             (2, 0, 0, 0),
    //             (0, 1, 0, 0),
    //             (1, 1, 0, 0),
    //             (2, 1, 0, 0)
    //         )),
    //         848
    //     )
    // }
}
