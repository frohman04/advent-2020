fn main() {
    let cypher_text = std::fs::read_to_string("src/bin/day09.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.parse::<u64>().expect("Unable to parse"))
                .collect::<Vec<u64>>()
        })
        .expect("Unable to open file");
    println!("{:?}", xmas(cypher_text, 25));
}

fn xmas(cypher_text: Vec<u64>, preamble_length: usize) -> u64 {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_xmas() {
        assert_eq!(
            xmas(
                vec![
                    35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277,
                    309, 576
                ],
                5
            ),
            127
        )
    }
}
