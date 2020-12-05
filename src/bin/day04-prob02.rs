use std::collections::HashMap;

fn main() {
    let parse_results = std::fs::read_to_string("src/bin/day04.txt")
        .map(|file| {
            file.lines()
                .map(|val| ParseResult::from_line(val.to_string()))
                .collect::<Vec<ParseResult>>()
        })
        .expect("Unable to open file");
    let passports = ParseResult::merge(parse_results);
    let valid_passports = passports
        .iter()
        .filter(|p| p.is_valid())
        .collect::<Vec<&Passport>>();
    println!("{:?}", valid_passports.len());
}

#[derive(Debug, PartialEq)]
enum ParseResult {
    Line(Passport),
    Divider,
}

impl ParseResult {
    pub fn from_line(line: String) -> ParseResult {
        if line.is_empty() {
            ParseResult::Divider
        } else {
            ParseResult::Line(Passport::from_line(line))
        }
    }

    pub fn merge(lines: Vec<ParseResult>) -> Vec<Passport> {
        let mut out: Vec<Passport> = Vec::new();

        let mut partials: Vec<Passport> = Vec::new();
        for line in lines {
            match line {
                ParseResult::Divider => {
                    out.push(Passport::merge(partials));
                    partials = Vec::new();
                }
                ParseResult::Line(passport) => partials.push(passport),
            }
        }
        if !partials.is_empty() {
            out.push(Passport::merge(partials));
        }

        out
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Passport {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    pub fn new(
        birth_year: Option<String>,
        issue_year: Option<String>,
        expiration_year: Option<String>,
        height: Option<String>,
        hair_color: Option<String>,
        eye_color: Option<String>,
        passport_id: Option<String>,
        country_id: Option<String>,
    ) -> Passport {
        Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        }
    }

    fn parse_field_str(entries: &HashMap<&str, &str>, key: &str) -> Option<String> {
        entries.get(key).map(|v| v.to_string())
    }

    pub fn from_line(line: String) -> Passport {
        let entries = line
            .split(" ")
            .map(|field| {
                let pieces = field.split(":").collect::<Vec<&str>>();
                (pieces[0], pieces[1])
            })
            .collect::<HashMap<&str, &str>>();

        Passport::new(
            Passport::parse_field_str(&entries, "byr"),
            Passport::parse_field_str(&entries, "iyr"),
            Passport::parse_field_str(&entries, "eyr"),
            Passport::parse_field_str(&entries, "hgt"),
            Passport::parse_field_str(&entries, "hcl"),
            Passport::parse_field_str(&entries, "ecl"),
            Passport::parse_field_str(&entries, "pid"),
            Passport::parse_field_str(&entries, "cid"),
        )
    }

    pub fn merge(partials: Vec<Passport>) -> Passport {
        let mut out = partials[0].clone();
        for partial in partials.iter().skip(1) {
            if out.birth_year.is_none() {
                out.birth_year = partial.birth_year.clone();
            }
            if out.issue_year.is_none() {
                out.issue_year = partial.issue_year.clone();
            }
            if out.expiration_year.is_none() {
                out.expiration_year = partial.expiration_year.clone();
            }
            if out.height.is_none() {
                out.height = partial.height.clone();
            }
            if out.hair_color.is_none() {
                out.hair_color = partial.hair_color.clone();
            }
            if out.eye_color.is_none() {
                out.eye_color = partial.eye_color.clone();
            }
            if out.passport_id.is_none() {
                out.passport_id = partial.passport_id.clone();
            }
            if out.country_id.is_none() {
                out.country_id = partial.country_id.clone();
            }
        }
        out
    }

    fn is_valid_year(value: &Option<String>, min: u16, max: u16) -> bool {
        match value {
            Some(v) => match v.parse::<u16>() {
                Ok(year) => min <= year && year <= max,
                Err(_) => false,
            },
            None => false,
        }
    }

    fn substring(value: &String, start: usize, end: usize) -> String {
        value
            .chars()
            .skip(start)
            .take(end - start)
            .collect::<String>()
    }

    fn is_valid_height(value: &Option<String>) -> bool {
        match value {
            Some(v) => {
                if v.ends_with("cm") {
                    match Passport::substring(v, 0, v.len() - 2).parse::<u16>() {
                        Ok(cm) => 150 <= cm && cm <= 193,
                        Err(_) => false,
                    }
                } else if v.ends_with("in") {
                    match Passport::substring(v, 0, v.len() - 2).parse::<u16>() {
                        Ok(inches) => 59 <= inches && inches <= 76,
                        Err(_) => false,
                    }
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_valid_hair_color(value: &Option<String>) -> bool {
        match value {
            Some(v) => {
                if v.len() == 7 && v.chars().next().unwrap() == '#' {
                    Passport::substring(v, 1, 7)
                        .chars()
                        .all(|c| ('0' <= c && c <= '9') || ('a' <= c && c <= 'f'))
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_valid_eye_color(value: &Option<String>) -> bool {
        match value {
            Some(v) => {
                v == "amb"
                    || v == "blu"
                    || v == "brn"
                    || v == "gry"
                    || v == "grn"
                    || v == "hzl"
                    || v == "oth"
            }
            None => false,
        }
    }

    fn is_valid_passport_id(value: &Option<String>) -> bool {
        match value {
            Some(v) => v.len() == 9 && v.chars().all(|c| '0' <= c && c <= '9'),
            None => false,
        }
    }

    fn is_valid_country_id(_value: &Option<String>) -> bool {
        true
    }

    pub fn is_valid(&self) -> bool {
        Passport::is_valid_year(&self.birth_year, 1920, 2002)
            && Passport::is_valid_year(&self.issue_year, 2010, 2020)
            && Passport::is_valid_year(&self.expiration_year, 2020, 2030)
            && Passport::is_valid_height(&self.height)
            && Passport::is_valid_hair_color(&self.hair_color)
            && Passport::is_valid_eye_color(&self.eye_color)
            && Passport::is_valid_passport_id(&self.passport_id)
            && Passport::is_valid_country_id(&self.country_id)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_passport_from_line_single() {
        assert_eq!(
            Passport::from_line("byr:1937".to_string()),
            Passport::new(
                Some("1937".to_string()),
                None,
                None,
                None,
                None,
                None,
                None,
                None
            )
        );
    }

    #[test]
    fn test_passport_from_line_multiple() {
        assert_eq!(
            Passport::from_line("byr:1937 ecl:gry pid:860033327".to_string()),
            Passport::new(
                Some("1937".to_string()),
                None,
                None,
                None,
                None,
                Some("gry".to_string()),
                Some("860033327".to_string()),
                None
            )
        );
    }

    #[test]
    fn test_passport_merge() {
        assert_eq!(
            Passport::merge(vec![
                Passport::new(
                    Some("1937".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                Passport::new(
                    None,
                    Some("1990".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                Passport::new(
                    None,
                    None,
                    Some("2000".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None
                )
            ]),
            Passport::new(
                Some("1937".to_string()),
                Some("1990".to_string()),
                Some("2000".to_string()),
                None,
                None,
                None,
                None,
                None
            )
        )
    }

    #[test]
    fn test_passport_is_valid1() {
        assert_eq!(
            Passport::new(
                Some("1937".to_string()),
                Some("2017".to_string()),
                Some("2020".to_string()),
                Some("183cm".to_string()),
                Some("#fffffd".to_string()),
                Some("gry".to_string()),
                Some("860033327".to_string()),
                Some("147".to_string())
            )
            .is_valid(),
            true
        )
    }

    #[test]
    fn test_passport_is_valid2() {
        assert_eq!(
            Passport::new(
                Some("1929".to_string()),
                Some("2013".to_string()),
                Some("2023".to_string()),
                None,
                Some("#cfa07d".to_string()),
                Some("amb".to_string()),
                Some("028048884".to_string()),
                Some("350".to_string())
            )
            .is_valid(),
            false
        )
    }

    #[test]
    fn test_passport_is_valid3() {
        assert_eq!(
            Passport::new(
                Some("1931".to_string()),
                Some("2013".to_string()),
                Some("2024".to_string()),
                Some("179cm".to_string()),
                Some("#ae17e1".to_string()),
                Some("brn".to_string()),
                Some("760753108".to_string()),
                None
            )
            .is_valid(),
            true
        )
    }

    #[test]
    fn test_passport_is_valid4() {
        assert_eq!(
            Passport::new(
                None,
                Some("2011".to_string()),
                Some("2025".to_string()),
                Some("59in".to_string()),
                Some("#cfa07d".to_string()),
                Some("brn".to_string()),
                Some("166559648".to_string()),
                None
            )
            .is_valid(),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_year() {
        assert_eq!(
            Passport::is_valid_year(&Some("1983".to_string()), 1980, 1990),
            true
        )
    }

    #[test]
    fn test_passport_is_valid_year_nan() {
        assert_eq!(
            Passport::is_valid_year(&Some("abcd".to_string()), 1980, 1990),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_year_too_early() {
        assert_eq!(
            Passport::is_valid_year(&Some("1975".to_string()), 1980, 1990),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_year_too_late() {
        assert_eq!(
            Passport::is_valid_year(&Some("1995".to_string()), 1980, 1990),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_year_none() {
        assert_eq!(Passport::is_valid_year(&None, 1980, 1990), false)
    }

    #[test]
    fn test_passport_is_valid_height_cm() {
        assert_eq!(Passport::is_valid_height(&Some("170cm".to_string())), true)
    }

    #[test]
    fn test_passport_is_valid_height_cm_nan() {
        assert_eq!(Passport::is_valid_height(&Some("asdcm".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_cm_too_short() {
        assert_eq!(Passport::is_valid_height(&Some("140cm".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_cm_too_tall() {
        assert_eq!(Passport::is_valid_height(&Some("200cm".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_in() {
        assert_eq!(Passport::is_valid_height(&Some("70in".to_string())), true)
    }

    #[test]
    fn test_passport_is_valid_height_in_nan() {
        assert_eq!(Passport::is_valid_height(&Some("asin".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_in_too_short() {
        assert_eq!(Passport::is_valid_height(&Some("50in".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_in_too_tall() {
        assert_eq!(Passport::is_valid_height(&Some("80in".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_bad_units() {
        assert_eq!(Passport::is_valid_height(&Some("80m".to_string())), false)
    }

    #[test]
    fn test_passport_is_valid_height_none() {
        assert_eq!(Passport::is_valid_height(&None), false)
    }

    #[test]
    fn test_passport_is_valid_hair_color() {
        assert_eq!(
            Passport::is_valid_hair_color(&Some("#ad45f0".to_string())),
            true
        )
    }

    #[test]
    fn test_passport_is_valid_hair_color_missing_pound() {
        assert_eq!(
            Passport::is_valid_hair_color(&Some("ad45f0".to_string())),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_hair_color_upper_case() {
        assert_eq!(
            Passport::is_valid_hair_color(&Some("#AD45F0".to_string())),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_hair_color_not_hex() {
        assert_eq!(
            Passport::is_valid_hair_color(&Some("#ag45f0".to_string())),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_hair_color_none() {
        assert_eq!(Passport::is_valid_hair_color(&None), false)
    }

    #[test]
    fn test_passport_is_valid_eye_color() {
        assert_eq!(Passport::is_valid_eye_color(&Some("amb".to_string())), true)
    }

    #[test]
    fn test_passport_is_valid_eye_color_invalid_color() {
        assert_eq!(
            Passport::is_valid_eye_color(&Some("foo".to_string())),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_eye_color_none() {
        assert_eq!(Passport::is_valid_eye_color(&None), false)
    }

    #[test]
    fn test_passport_is_valid_passport_id() {
        assert_eq!(
            Passport::is_valid_passport_id(&Some("012345679".to_string())),
            true
        )
    }

    #[test]
    fn test_passport_is_valid_passport_id_length() {
        assert_eq!(
            Passport::is_valid_passport_id(&Some("123".to_string())),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_passport_id_chars() {
        assert_eq!(
            Passport::is_valid_passport_id(&Some("abcdefghi".to_string())),
            false
        )
    }

    #[test]
    fn test_passport_is_valid_passport_id_none() {
        assert_eq!(Passport::is_valid_passport_id(&None), false)
    }

    #[test]
    fn test_passport_is_valid_country_id() {
        assert_eq!(
            Passport::is_valid_country_id(&Some("123".to_string())),
            true
        )
    }

    #[test]
    fn test_passport_is_valid_country_id_none() {
        assert_eq!(Passport::is_valid_country_id(&None), true)
    }

    #[test]
    fn test_parseresult_from_line_empty() {
        assert_eq!(ParseResult::from_line("".to_string()), ParseResult::Divider)
    }

    #[test]
    fn test_parseresult_from_line_full() {
        assert_eq!(
            ParseResult::from_line("byr:1937".to_string()),
            ParseResult::Line(Passport::new(
                Some("1937".to_string()),
                None,
                None,
                None,
                None,
                None,
                None,
                None
            ))
        )
    }

    #[test]
    fn test_parseresult_merge_single_term_divider() {
        assert_eq!(
            ParseResult::merge(vec![
                ParseResult::Line(Passport::new(
                    Some("1937".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Line(Passport::new(
                    None,
                    Some("1990".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Divider
            ]),
            vec![Passport::new(
                Some("1937".to_string()),
                Some("1990".to_string()),
                None,
                None,
                None,
                None,
                None,
                None
            )]
        )
    }

    #[test]
    fn test_parseresult_merge_single_no_term_divider() {
        assert_eq!(
            ParseResult::merge(vec![
                ParseResult::Line(Passport::new(
                    Some("1937".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Line(Passport::new(
                    None,
                    Some("1990".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
            ]),
            vec![Passport::new(
                Some("1937".to_string()),
                Some("1990".to_string()),
                None,
                None,
                None,
                None,
                None,
                None
            )]
        )
    }

    #[test]
    fn test_parseresult_merge_multiple() {
        assert_eq!(
            ParseResult::merge(vec![
                ParseResult::Line(Passport::new(
                    Some("1937".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Line(Passport::new(
                    None,
                    Some("1990".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Divider,
                ParseResult::Line(Passport::new(
                    None,
                    Some("1990".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Line(Passport::new(
                    None,
                    None,
                    Some("2000".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None
                )),
                ParseResult::Divider
            ]),
            vec![
                Passport::new(
                    Some("1937".to_string()),
                    Some("1990".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None
                ),
                Passport::new(
                    None,
                    Some("1990".to_string()),
                    Some("2000".to_string()),
                    None,
                    None,
                    None,
                    None,
                    None
                )
            ]
        )
    }
}
