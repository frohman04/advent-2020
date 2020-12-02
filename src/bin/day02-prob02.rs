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
    first_i: usize,
    second_i: usize,
    char: char,
    password: String,
}

impl Password {
    pub fn new(first_i: usize, second_i: usize, char: char, password: String) -> Password {
        Password {
            first_i,
            second_i,
            char,
            password,
        }
    }

    pub fn from_line(line: String) -> Password {
        let pattern = Regex::new(r"([0-9]+)-([0-9]+) (.): (.+)").expect("Invalid regex");
        let captures = pattern
            .captures(line.as_str())
            .expect(&format!("Unable to match line '{}'", line));

        let raw_first_i = captures
            .get(1)
            .expect(&format!("Unable to match 'first_i': {}", line))
            .as_str();
        let first_i = raw_first_i.parse::<usize>().expect(&format!(
            "Unable to parse usize for 'first_i' ({}): {}",
            raw_first_i, line
        ));
        let raw_second_i = captures
            .get(2)
            .expect(&format!("Unable to match 'second_i': {}", line))
            .as_str();
        let second_i = raw_second_i.parse::<usize>().expect(&format!(
            "Unable to parse usize for 'second_i' ({}): {}",
            raw_second_i, line
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

        Password::new(first_i, second_i, char, password)
    }

    pub fn is_valid(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<char>>();
        let has_first = chars[self.first_i - 1] == self.char;
        let has_second = chars[self.second_i - 1] == self.char;
        (has_first || has_second) && !(has_first && has_second)
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
    fn test_is_valid_yes() {
        assert_eq!(
            Password::new(1, 3, 'a', "abcde".to_string()).is_valid(),
            true
        )
    }

    #[test]
    fn test_is_valid_no1() {
        assert_eq!(
            Password::new(1, 3, 'b', "cdefg".to_string()).is_valid(),
            false
        )
    }

    #[test]
    fn test_is_valid_no2() {
        assert_eq!(
            Password::new(2, 9, 'c', "ccccccccc".to_string()).is_valid(),
            false
        )
    }
}
