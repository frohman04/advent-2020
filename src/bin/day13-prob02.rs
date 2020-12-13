extern crate num;

use num::integer::gcd;

fn main() {
    let lines = std::fs::read_to_string("src/bin/day13.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    let ids = lines[1]
        .split(",")
        .map(|id| {
            if id == "x" {
                None
            } else {
                Some(
                    id.parse::<u16>()
                        .expect(&format!("Unable to parse id: {}", id)),
                )
            }
        })
        .collect::<Vec<Option<u16>>>();
    println!("{:?}", find_sequential_departures(ids));
}

fn find_sequential_departures(ids: Vec<Option<u16>>) -> u64 {
    let ids = ids
        .into_iter()
        .enumerate()
        .filter_map(|(i, id_opt)| id_opt.map(|id| (id as u64, i as u64)))
        .collect::<Vec<(u64, u64)>>();
    for i in 0..ids.len() - 1 {
        for j in i + 1..ids.len() {
            if gcd(ids[i].0, ids[j].0) != 1 {
                panic!(
                    "ids[{}]={} and ids[{}]={} are not coprime",
                    i, ids[i].0, j, ids[j].0
                );
            }
        }
    }
    // println!("original: {:?}", ids);

    let global_offset = ids[0].1;
    let ids = ids
        .iter()
        .map(|(id, offset)| (*id, offset - global_offset))
        .collect::<Vec<(u64, u64)>>();
    // println!("adjusted: {:?}", ids);

    // https://www.reddit.com/r/adventofcode/comments/kc5bl5/weird_math_trick_goes_viral/gfotzko/
    let (mut increment, mut pos) = (ids[0].0, ids[0].1);
    for (time, offset) in ids.into_iter().skip(1) {
        while (pos + offset) % time != 0 {
            pos += increment;
        }
        increment *= time;
    }
    pos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_sequential_departure1() {
        assert_eq!(
            find_sequential_departures(vec![
                Some(7),
                Some(13),
                None,
                None,
                Some(59),
                None,
                Some(31),
                Some(19)
            ]),
            1068781
        )
    }

    #[test]
    fn test_find_sequential_departure2() {
        assert_eq!(
            find_sequential_departures(vec![Some(17), None, Some(13), Some(19)]),
            3417
        )
    }

    #[test]
    fn test_find_sequential_departure3() {
        assert_eq!(
            find_sequential_departures(vec![Some(67), Some(7), Some(59), Some(61)]),
            754018
        )
    }

    #[test]
    fn test_find_sequential_departure4() {
        assert_eq!(
            find_sequential_departures(vec![Some(67), None, Some(7), Some(59), Some(61)]),
            779210
        )
    }

    #[test]
    fn test_find_sequential_departure5() {
        assert_eq!(
            find_sequential_departures(vec![Some(67), Some(7), None, Some(59), Some(61)]),
            1261476
        )
    }

    #[test]
    fn test_find_sequential_departure6() {
        assert_eq!(
            find_sequential_departures(vec![Some(1789), Some(37), Some(47), Some(1889)]),
            1202161486
        )
    }
}
