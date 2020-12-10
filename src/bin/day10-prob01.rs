fn main() {
    let adapters = std::fs::read_to_string("src/bin/day10.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.parse::<u8>().expect("Unable to parse"))
                .collect::<Vec<u8>>()
        })
        .expect("Unable to open file");
    println!("{:?}", chain_adapters(adapters));
}

fn chain_adapters(adapters: Vec<u8>) -> u32 {
    let mut adapters = adapters.clone();
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let mut counts = [0u32; 4];
    let mut current = 0;
    for adapter in adapters {
        let diff = adapter - current;
        if diff < 1 || 3 < diff {
            panic!(
                "Unable to complete chain, adapter {} is too large!",
                adapter
            );
        }
        counts[diff as usize] += 1;
        current = adapter;
    }
    counts[1] * counts[3]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain_adapters1() {
        assert_eq!(
            chain_adapters(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]),
            35
        )
    }

    #[test]
    fn test_chain_adapters2() {
        assert_eq!(
            chain_adapters(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ]),
            220
        )
    }
}
