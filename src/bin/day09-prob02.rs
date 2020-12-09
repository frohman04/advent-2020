use std::cmp;

fn main() {
    let cypher_text = std::fs::read_to_string("src/bin/day09.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.parse::<u64>().expect("Unable to parse"))
                .collect::<Vec<u64>>()
        })
        .expect("Unable to open file");
    println!(
        "{:?}",
        xmas_fix(&cypher_text, xmas_corruption(&cypher_text, 25))
    );
}

fn xmas_corruption(cypher_text: &Vec<u64>, preamble_length: usize) -> u64 {
    for target_i in preamble_length..cypher_text.len() {
        let target = cypher_text[target_i];
        let mut found = false;
        for i1 in (target_i - preamble_length)..target_i {
            for i2 in i1 + 1..target_i {
                if cypher_text[i1] + cypher_text[i2] == target {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if !found {
            return target;
        }
    }
    panic!("Unable to find corrupted item in cypher text")
}

fn xmas_fix(cypher_text: &Vec<u64>, corrupted_value: u64) -> u64 {
    for start_i in 0..cypher_text.len() {
        let mut sum = cypher_text[start_i];
        let mut min = cypher_text[start_i];
        let mut max = cypher_text[start_i];
        for end_i in (start_i + 1)..cypher_text.len() {
            sum += cypher_text[end_i];
            min = cmp::min(min, cypher_text[end_i]);
            max = cmp::max(max, cypher_text[end_i]);
            if sum == corrupted_value {
                return min + max;
            } else if sum > corrupted_value {
                break;
            }
        }
    }
    panic!("Unable to find fix for corruption")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xmas_corruption() {
        assert_eq!(
            xmas_corruption(
                &vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            ),
            127
        )
    }

    #[test]
    fn test_xmas_fix() {
        assert_eq!(
            xmas_fix(
                &vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                127
            ),
            62
        )
    }
}
