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

#[derive(Debug, PartialEq, Clone)]
enum Turn {
    Left,
    Right,
}

fn turn(x: i32, y: i32, deg: i16) -> (i32, i32) {
    let deg = if deg < 0 { deg + 360 } else { deg };
    match deg {
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => panic!("Invalid turn amount: {}", deg),
    }
}

fn navigate(commands: Vec<Command>) -> u32 {
    let mut waypoint_x: i32 = 10;
    let mut waypoint_y: i32 = 1;
    let mut ship_x: i64 = 0;
    let mut ship_y: i64 = 0;
    for command in commands {
        match command {
            Command::Turn(t, deg) => match t {
                Turn::Left => {
                    let (x, y) = turn(waypoint_x, waypoint_y, -(deg as i16));
                    waypoint_x = x;
                    waypoint_y = y;
                }
                Turn::Right => {
                    let (x, y) = turn(waypoint_x, waypoint_y, deg as i16);
                    waypoint_x = x;
                    waypoint_y = y;
                }
            },
            Command::Move(dir, amount) => match dir {
                Direction::North => waypoint_y += amount as i32,
                Direction::East => waypoint_x += amount as i32,
                Direction::South => waypoint_y -= amount as i32,
                Direction::West => waypoint_x -= amount as i32,
            },
            Command::MoveForward(amount) => {
                ship_x += (waypoint_x * amount as i32) as i64;
                ship_y += (waypoint_y * amount as i32) as i64;
            }
        }
        // println!(
        //     "{:?} => w:({:?}, {:?}) s({:?}, {:?})",
        //     command, waypoint_x, waypoint_y, ship_x, ship_y
        // );
    }
    (ship_x.abs() + ship_y.abs()) as u32
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
        assert_eq!(turn(4, 10, 90), (10, -4))
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
            286
        )
    }
}
