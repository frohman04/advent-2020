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

fn find(charges: Vec<u32>, amount: u32) -> Result<u32, ()> {
    let mut out: Result<u32, ()> = Err(());
    for i in 0..(charges.len() - 2) {
        for j in (i + 1)..(charges.len() - 1) {
            for k in (j + 1)..charges.len() {
                if charges[i] + charges[j] + charges[k] == amount {
                    out = Ok(charges[i] * charges[j] * charges[k]);
                }
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
        assert_eq!(
            find(vec![1721, 979, 366, 299, 675, 1456], 2020),
            Ok(241861950)
        )
    }
}
