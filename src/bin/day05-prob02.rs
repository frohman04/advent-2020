fn main() {
    let seats = std::fs::read_to_string("src/bin/day05.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Seat::from_partition(val.to_string()))
                .collect::<Vec<Seat>>()
        })
        .expect("Unable to open file");
    println!("{:?}", find_seat(seats));
}

#[derive(Debug, Clone, PartialEq)]
struct Seat {
    row: u8,
    col: u8,
}

fn find_seat(seats: Vec<Seat>) -> u16 {
    // assign seats into array representing all seats on airplane
    let max_id = seats.iter().map(|s| s.get_id()).max().unwrap();
    let mut airplane: Vec<Option<Seat>> = vec![None; max_id as usize + 1];
    for seat in seats {
        airplane[seat.get_id() as usize] = Some(seat.clone());
    }

    let missing = airplane
        .iter()
        .enumerate()
        .filter(|(id, seat)| {
            seat.is_none()
                && (id > &0usize && airplane[id - 1].is_some())
                && (id < &((max_id + 1) as usize) && airplane[id + 1].is_some())
        })
        .map(|(id, _)| id as u16)
        .collect::<Vec<u16>>();
    if missing.len() != 1 {
        panic!("Too many seats are mine: {:?}", missing);
    }
    missing[0]
}

impl Seat {
    pub fn new(row: u8, col: u8) -> Seat {
        if row > 127 {
            panic!(format!("Invalid row: {}", row));
        }
        if col > 7 {
            panic!(format!("Invalid col: {}", col));
        }
        Seat { row, col }
    }

    fn substring(value: &String, start: usize, end: usize) -> String {
        value
            .chars()
            .skip(start)
            .take(end - start)
            .collect::<String>()
    }

    fn search(directions: String, back_char: char, forward_char: char) -> u8 {
        let mut min: u8 = 0;
        let mut max: u8 = 2u8.pow(directions.len() as u32) - 1;
        for char in directions.chars() {
            let next = (max + min) / 2;
            if char == back_char {
                max = next;
            } else if char == forward_char {
                min = next + 1;
            } else {
                panic!(format!("Found invalid direction character: {}", char));
            }
        }
        if min != max {
            panic!(format!(
                "min and max not equal after search: {} != {}",
                min, max
            ));
        }
        min
    }

    pub fn from_partition(line: String) -> Seat {
        let row = Seat::search(Seat::substring(&line, 0, 7), 'F', 'B');
        let col = Seat::search(Seat::substring(&line, 7, 10), 'L', 'R');
        Seat::new(row, col)
    }

    pub fn get_id(&self) -> u16 {
        (self.row as u16) * 8 + (self.col as u16)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_search1() {
        assert_eq!(Seat::search("FBFBBFF".to_string(), 'F', 'B'), 44)
    }

    #[test]
    fn test_search2() {
        assert_eq!(Seat::search("RLR".to_string(), 'L', 'R'), 5)
    }

    #[test]
    fn test_from_partition1() {
        assert_eq!(
            Seat::from_partition("FBFBBFFRLR".to_string()),
            Seat::new(44, 5)
        )
    }

    #[test]
    fn test_from_partition2() {
        assert_eq!(
            Seat::from_partition("BFFFBBFRRR".to_string()),
            Seat::new(70, 7)
        )
    }

    #[test]
    fn test_from_partition3() {
        assert_eq!(
            Seat::from_partition("FFFBBBFRRR".to_string()),
            Seat::new(14, 7)
        )
    }

    #[test]
    fn test_from_partition4() {
        assert_eq!(
            Seat::from_partition("BBFFBBFRLL".to_string()),
            Seat::new(102, 4)
        )
    }

    #[test]
    fn test_get_id() {
        assert_eq!(Seat::new(44, 5).get_id(), 357)
    }

    #[test]
    fn test_get_id2() {
        assert_eq!(Seat::new(70, 7).get_id(), 567)
    }

    #[test]
    fn test_get_id3() {
        assert_eq!(Seat::new(14, 7).get_id(), 119)
    }

    #[test]
    fn test_get_id4() {
        assert_eq!(Seat::new(102, 4).get_id(), 820)
    }
}
