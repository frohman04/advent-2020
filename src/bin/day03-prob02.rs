use std::rc::Rc;

fn main() {
    let hill = Rc::new(
        std::fs::read_to_string("src/bin/day03.txt")
            .map(|file| {
                file.lines()
                    .filter(|line| !line.is_empty())
                    .map(|val| parse_line(val.to_string()))
                    .collect::<Vec<Vec<bool>>>()
            })
            .expect("Unable to open file"),
    );
    println!(
        "{:?}",
        traverse_hill(hill.clone(), 1, 1)
            * traverse_hill(hill.clone(), 3, 1)
            * traverse_hill(hill.clone(), 5, 1)
            * traverse_hill(hill.clone(), 7, 1)
            * traverse_hill(hill.clone(), 1, 2)
    );
}

fn parse_line(line: String) -> Vec<bool> {
    line.chars()
        .map(|x| match x {
            '.' => false,
            '#' => true,
            _ => panic!("Unable to parse line"),
        })
        .collect()
}

fn traverse_hill(hill: Rc<Vec<Vec<bool>>>, right: usize, down: usize) -> usize {
    let mut point = Point::initial(hill);
    let mut end_loop = false;
    let mut num_trees: usize = 0;
    while !end_loop {
        match point.traverse(down, right) {
            Ok(new_point) => {
                if new_point.is_tree {
                    num_trees += 1;
                }
                point = new_point;
            }
            Err(()) => end_loop = true,
        }
    }
    num_trees
}

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    is_tree: bool,
    hill: Rc<Vec<Vec<bool>>>,
}

impl Point {
    pub fn initial(hill: Rc<Vec<Vec<bool>>>) -> Point {
        Point::new(0, 0, false, hill)
    }

    pub fn new(x: usize, y: usize, is_tree: bool, hill: Rc<Vec<Vec<bool>>>) -> Point {
        Point {
            x,
            y,
            is_tree,
            hill,
        }
    }

    pub fn traverse(&self, down: usize, right: usize) -> Result<Point, ()> {
        let new_y = self.y + down;
        if new_y >= self.hill.len() {
            Err(())
        } else {
            let new_x = self.x + right;

            let temp_x = new_x % self.hill[new_y].len();
            let new_is_tree = self.hill[new_y][temp_x];

            Ok(Point::new(new_x, new_y, new_is_tree, self.hill.clone()))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("..##.......".to_string()),
            vec![false, false, true, true, false, false, false, false, false, false, false]
        )
    }

    #[test]
    fn test_traverse_normal() {
        let hill = Rc::new(vec![vec![false, false, false], vec![false, true, false]]);
        assert_eq!(
            Point::initial(hill.clone()).traverse(1, 1),
            Ok(Point::new(1, 1, true, hill.clone()))
        )
    }

    #[test]
    fn test_traverse_go_oob() {
        let hill = Rc::new(vec![vec![false, false, false], vec![true, false, false]]);
        assert_eq!(
            Point::initial(hill.clone()).traverse(1, 3),
            Ok(Point::new(3, 1, true, hill.clone()))
        )
    }

    #[test]
    fn test_traverse_remain_oob() {
        let hill = Rc::new(vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![true, false, false],
        ]);
        assert_eq!(
            Point::initial(hill.clone())
                .traverse(1, 3)
                .unwrap()
                .traverse(1, 3),
            Ok(Point::new(6, 2, true, hill.clone()))
        )
    }

    #[test]
    fn test_traverse_reach_end() {
        let hill = Rc::new(vec![vec![false, false, false], vec![true, false, false]]);
        assert_eq!(
            Point::initial(hill.clone())
                .traverse(1, 3)
                .unwrap()
                .traverse(1, 3),
            Err(())
        )
    }

    #[test]
    fn test_traverse_hill() {
        let hill = Rc::new(vec![
            vec![
                false, false, true, true, false, false, false, false, false, false, false,
            ],
            vec![
                true, false, false, false, true, false, false, false, true, false, false,
            ],
            vec![
                false, true, false, false, false, false, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, false, false, false, true, false, true,
            ],
            vec![
                false, true, false, false, false, true, true, false, false, true, false,
            ],
            vec![
                false, false, true, false, true, true, false, false, false, false, false,
            ],
            vec![
                false, true, false, true, false, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, false, false, false, false, false, false, true,
            ],
            vec![
                true, false, true, true, false, false, false, true, false, false, false,
            ],
            vec![
                true, false, false, false, true, true, false, false, false, false, true,
            ],
            vec![
                false, true, false, false, true, false, false, false, true, false, true,
            ],
        ]);
        assert_eq!(traverse_hill(hill, 3, 1), 7)
    }
}
