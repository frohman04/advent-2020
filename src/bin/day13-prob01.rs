fn main() {
    let lines = std::fs::read_to_string("src/bin/day13.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    let earliest = lines[0]
        .parse::<u32>()
        .expect(&format!("Unable to parse earliest timestamp: {}", lines[0]));
    let ids = lines[1]
        .split(",")
        .filter_map(|id| {
            if id == "x" {
                None
            } else {
                Some(
                    id.parse::<u16>()
                        .expect(&format!("Unable to parse id: {}", id)),
                )
            }
        })
        .collect::<Vec<u16>>();
    println!("{:?}", find_first_shuttle(earliest, ids));
}

fn find_first_shuttle(earliest: u32, ids: Vec<u16>) -> u32 {
    let (pickup_time, first_shuttle) = ids
        .into_iter()
        .map(|id| ((earliest / id as u32) * id as u32 + id as u32, id))
        .min()
        .unwrap();
    // println!("{} {}", pickup_time, first_shuttle);
    first_shuttle as u32 * (pickup_time - earliest)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_first_shuttle() {
        assert_eq!(find_first_shuttle(939, vec![7, 13, 59, 31, 19]), 295)
    }
}
