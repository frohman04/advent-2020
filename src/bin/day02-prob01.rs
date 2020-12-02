extern crate regex;

use regex::Regex;

fn main() {
    let passwords = std::fs::read_to_string("src/bin/day02.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| Password::from_line(val.to_string()))
                .collect::<Vec<Password>>()
        })
        .expect("Unable to open file");
    println!(
        "{:?}",
        passwords
            .into_iter()
            .filter(|p| p.is_valid())
            .collect::<Vec<Password>>()
            .len()
    );
}

#[derive(Debug, PartialEq)]
struct Password {
    at_least: usize,
    at_most: usize,
    char: char,
    password: String,
}

impl Password {
    pub fn new(at_least: usize, at_most: usize, char: char, password: String) -> Password {
        Password {
            at_least,
            at_most,
            char,
            password,
        }
    }

    pub fn from_line(line: String) -> Password {
        let pattern = Regex::new(r"([0-9]+)-([0-9]+) (.): (.+)").expect("Invalid regex");
        let captures = pattern
            .captures(line.as_str())
            .expect(&format!("Unable to match line '{}'", line));

        let raw_at_least = captures
            .get(1)
            .expect(&format!("Unable to match 'at_least': {}", line))
            .as_str();
        let at_least = raw_at_least.parse::<usize>().expect(&format!(
            "Unable to parse usize for 'at_least' ({}): {}",
            raw_at_least, line
        ));
        let raw_at_most = captures
            .get(2)
            .expect(&format!("Unable to match 'at_most': {}", line))
            .as_str();
        let at_most = raw_at_most.parse::<usize>().expect(&format!(
            "Unable to parse usize for 'at_most' ({}): {}",
            raw_at_most, line
        ));
        let raw_char = captures
            .get(3)
            .expect(&format!("Unable to match 'char': {}", line))
            .as_str();
        let char = raw_char.chars().next().expect(&format!(
            "Unable to get first character for 'char' ({}): {}",
            raw_char, line
        ));
        let password = captures
            .get(4)
            .expect(&format!("Unable to match 'password': {}", line))
            .as_str()
            .to_string();

        Password::new(at_least, at_most, char, password)
    }

    pub fn is_valid(&self) -> bool {
        let count = self
            .password
            .clone()
            .chars()
            .filter(|c| *c == self.char)
            .collect::<Vec<char>>()
            .len();
        count <= self.at_most && count >= self.at_least
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_line() {
        assert_eq!(
            Password::from_line("1-3 a: abcde".to_string()),
            Password::new(1, 3, 'a', "abcde".to_string())
        );
    }

    #[test]
    fn test_from_line_doubledigit() {
        assert_eq!(
            Password::from_line("11-33 a: abcde".to_string()),
            Password::new(11, 33, 'a', "abcde".to_string())
        );
    }

    #[test]
    fn test_is_valid_yes1() {
        assert_eq!(
            Password::new(1, 3, 'a', "abcde".to_string()).is_valid(),
            true
        )
    }

    #[test]
    fn test_is_valid_yes2() {
        assert_eq!(
            Password::new(2, 9, 'c', "ccccccccc".to_string()).is_valid(),
            true
        )
    }

    #[test]
    fn test_is_valid_no() {
        assert_eq!(
            Password::new(1, 3, 'b', "cdefg".to_string()).is_valid(),
            false
        )
    }
}
