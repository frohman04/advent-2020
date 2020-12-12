fn main() {
    let commands = std::fs::read_to_string("src/bin/day12.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Command::from_line(val))
                .collect::<Vec<Command>>()
        })
        .expect("Unable to open file");
    println!("{:?}", navigate(commands));
}

fn substring(value: &str, start: usize, end: usize) -> String {
    value
        .chars()
        .skip(start)
        .take(end - start)
        .collect::<String>()
}

#[derive(Debug, PartialEq, Clone)]
enum Command {
    Move(Direction, u16),
    MoveForward(u16),
    Turn(Turn, u16),
}

impl Command {
    pub fn from_line(line: &str) -> Command {
        let raw_command = line.chars().take(1).collect::<Vec<char>>()[0];
        let raw_amount = substring(line, 1, line.len());
        let amount = raw_amount
            .parse::<u16>()
            .expect(&format!("Unable to parse amount: {}", raw_amount));
        match raw_command {
            'N' => Command::Move(Direction::North, amount),
            'S' => Command::Move(Direction::South, amount),
            'E' => Command::Move(Direction::East, amount),
            'W' => Command::Move(Direction::West, amount),
            'L' => Command::Turn(Turn::Left, amount),
            'R' => Command::Turn(Turn::Right, amount),
            'F' => Command::MoveForward(amount),
            c => panic!("Invalid command: {}", c),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn(self, amt_deg: i16) -> Direction {
        let curr = match self {
            Direction::North => 0,
            Direction::East => 90,
            Direction::South => 180,
            Direction::West => 270,
        };
        let new = {
            let n = (curr + amt_deg) % 360;
            if n < 0 {
                n + 360
            } else {
                n
            }
        };
        match new {
            0 => Direction::North,
            90 => Direction::East,
            180 => Direction::South,
            270 => Direction::West,
            d => panic!("Invalid direction: {}.  Must turn in increments of 90", d),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Turn {
    Left,
    Right,
}

fn navigate(commands: Vec<Command>) -> u32 {
    let mut heading = Direction::East;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for command in commands {
        match command {
            Command::Turn(turn, deg) => match turn {
                Turn::Left => heading = heading.turn(-(deg as i16)),
                Turn::Right => heading = heading.turn(deg as i16),
            },
            Command::Move(dir, amount) => match dir {
                Direction::North => y += amount as i32,
                Direction::East => x += amount as i32,
                Direction::South => y -= amount as i32,
                Direction::West => x -= amount as i32,
            },
            Command::MoveForward(amount) => match heading {
                Direction::North => y += amount as i32,
                Direction::East => x += amount as i32,
                Direction::South => y -= amount as i32,
                Direction::West => x -= amount as i32,
            },
        }
        // println!("{:?} => h:{:?} ({:?}, {:?})", command, heading, x, y);
    }
    (x.abs() + y.abs()) as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_command_from_line_move_north() {
        assert_eq!(
            Command::from_line("N21"),
            Command::Move(Direction::North, 21)
        )
    }

    #[test]
    fn test_command_from_line_move_south() {
        assert_eq!(
            Command::from_line("S42"),
            Command::Move(Direction::South, 42)
        )
    }

    #[test]
    fn test_command_from_line_move_east() {
        assert_eq!(Command::from_line("E4"), Command::Move(Direction::East, 4))
    }

    #[test]
    fn test_command_from_line_move_west() {
        assert_eq!(
            Command::from_line("W51"),
            Command::Move(Direction::West, 51)
        )
    }

    #[test]
    fn test_command_from_line_move_forward() {
        assert_eq!(Command::from_line("F4"), Command::MoveForward(4))
    }

    #[test]
    fn test_command_from_line_turn_left() {
        assert_eq!(Command::from_line("L90"), Command::Turn(Turn::Left, 90))
    }

    #[test]
    fn test_command_from_line_turn_right() {
        assert_eq!(Command::from_line("R270"), Command::Turn(Turn::Right, 270))
    }

    #[test]
    fn test_direction_turn() {
        assert_eq!(Direction::North.turn(90), Direction::East)
    }

    #[test]
    fn test_direction_turn_negative_deg() {
        assert_eq!(Direction::North.turn(-90), Direction::West)
    }

    #[test]
    fn test_direction_turn_positive_wrap() {
        assert_eq!(Direction::West.turn(90), Direction::North)
    }

    #[test]
    fn test_navigate() {
        assert_eq!(
            navigate(vec![
                Command::MoveForward(10),
                Command::Move(Direction::North, 3),
                Command::MoveForward(7),
                Command::Turn(Turn::Right, 90),
                Command::MoveForward(11)
            ]),
            25
        )
    }
}
