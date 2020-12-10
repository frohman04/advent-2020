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

fn chain_adapters(adapters: Vec<u8>) -> u64 {
    let mut adapters = adapters.clone();
    adapters.sort();
    let max_adapter = adapters.last().unwrap() + 3;
    adapters.insert(0, 0);
    adapters.push(max_adapter);

    let adapters_bool = {
        let mut adapters_bool: Vec<bool> = Vec::new();
        for _ in 0..adapters.last().unwrap() + 1 {
            adapters_bool.push(false);
        }
        for adapter in adapters.iter() {
            adapters_bool[adapter.clone() as usize] = true;
        }
        adapters_bool
    };

    let mut chain: Vec<u64> = Vec::new();
    for _ in 0..max_adapter + 1 {
        chain.push(0);
    }
    chain[0] = 1;
    for adapter in adapters {
        let adapter = adapter as usize;
        for next_adapter in adapter + 1..adapter + 3 + 1 {
            let next_adapter = next_adapter as usize;
            if next_adapter < adapters_bool.len() && adapters_bool[next_adapter] {
                chain[next_adapter] += chain[adapter];
            }
        }
    }
    *chain.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain_adapters1() {
        assert_eq!(
            chain_adapters(vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]),
            8
        )
    }

    #[test]
    fn test_chain_adapters2() {
        assert_eq!(
            chain_adapters(vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3
            ]),
            19208
        )
    }
}
