fn main() {
    let pubkey1 = 6930903u64;
    let pubkey2 = 19716708u64;
    println!(
        "{:?}",
        (
            get_encryption_key(pubkey1, get_iterations(pubkey2, 7)),
            get_encryption_key(pubkey2, get_iterations(pubkey1, 7))
        )
    );
}

fn get_iterations(pubkey: u64, subject_number: u64) -> u64 {
    let mut value = 1u64;
    let mut loop_count = 0u64;
    while value != pubkey {
        value *= subject_number;
        value = value % 20201227;
        loop_count += 1;
    }
    loop_count
}

fn get_encryption_key(pubkey: u64, loop_count: u64) -> u64 {
    let mut value = 1u64;
    for _ in 0..loop_count {
        value *= pubkey;
        value = value % 20201227;
    }
    value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_iterations1() {
        assert_eq!(get_iterations(5764801, 7), 8)
    }

    #[test]
    fn test_get_iterations2() {
        assert_eq!(get_iterations(17807724, 7), 11)
    }

    #[test]
    fn test_get_encryption_key1() {
        assert_eq!(get_encryption_key(17807724, 8), 14897079)
    }

    #[test]
    fn test_get_encryption_key2() {
        assert_eq!(get_encryption_key(5764801, 11), 14897079)
    }
}
