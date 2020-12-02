fn main() {
    let charges = std::fs::read_to_string("src/bin/day01.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.parse::<u32>().ok().unwrap())
                .collect::<Vec<u32>>()
        })
        .expect("Unable to open file");
    println!(
        "{:?}",
        find(charges, 2020).expect("Unable to find number pair")
    );
}

fn find(mut charges: Vec<u32>, amount: u32) -> Result<u32, ()> {
    charges.sort();
    let mid = match charges.binary_search(&(amount / 2)) {
        Ok(i) => i,
        Err(i) => i,
    };

    let mut out: Result<u32, ()> = Err(());
    for i in 0..mid {
        for j in mid..charges.len() {
            if charges[i] + charges[j] == amount {
                out = Ok(charges[i] * charges[j]);
            }
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find(vec![1721, 979, 366, 299, 675, 1456], 2020), Ok(514579))
    }
}
