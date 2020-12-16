use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("src/bin/day16.txt")
        .map(|file| {
            file.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    println!("{:?}", Input::from_raw(input).nearby_error_rate());
}

#[derive(Debug, PartialEq)]
struct Input {
    fields: Vec<Field>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Input {
    pub fn new(fields: Vec<Field>, my_ticket: Ticket, nearby_tickets: Vec<Ticket>) -> Input {
        Input {
            fields,
            my_ticket,
            nearby_tickets,
        }
    }

    pub fn from_raw(lines: Vec<String>) -> Input {
        let mut in_fields = true;
        let mut in_my_ticket = false;
        let mut in_nearby_tickets = false;
        let mut skip_next = false;

        let mut fields: Vec<Field> = Vec::new();
        let mut my_ticket: Option<Ticket> = None;
        let mut nearby_tickets: Vec<Ticket> = Vec::new();

        for line in lines {
            // println!("{}", line);
            // println!("  in_fields:         {}", in_fields);
            // println!("  in_my_ticket:      {}", in_my_ticket);
            // println!("  in_nearby_tickets: {}", in_nearby_tickets);
            // println!("  skip_next:         {}", skip_next);
            if skip_next {
                skip_next = false;
                continue;
            }

            if line.is_empty() {
                if in_fields {
                    in_fields = false;
                    in_my_ticket = true;
                    skip_next = true;
                } else if in_my_ticket {
                    in_my_ticket = false;
                    in_nearby_tickets = true;
                    skip_next = true;
                }
            } else {
                if in_fields {
                    let pieces = line.split(": ").collect::<Vec<&str>>();
                    let field_name = pieces[0].clone();
                    let ranges = pieces[1]
                        .split(" or ")
                        .map(|range_raw| {
                            let bounds = range_raw
                                .split("-")
                                .map(|raw| {
                                    raw.parse::<u16>()
                                        .expect(&format!("Unable to parse value {} to u16", raw))
                                })
                                .collect::<Vec<u16>>();
                            bounds[0]..=bounds[1]
                        })
                        .collect::<Vec<RangeInclusive<u16>>>();

                    fields.push(Field::new(field_name.to_string(), ranges));
                } else if in_my_ticket {
                    my_ticket = Some(Ticket::new(
                        line.split(",")
                            .map(|val| val.parse::<u16>().expect("Unable to parse value as u16"))
                            .collect(),
                    ));
                } else if in_nearby_tickets {
                    nearby_tickets.push(Ticket::new(
                        line.split(",")
                            .map(|val| val.parse::<u16>().expect("Unable to parse value as u16"))
                            .collect(),
                    ));
                }
            }
        }

        if fields.is_empty() {
            panic!("Did not find any fields defined in the input file");
        }
        if my_ticket.is_none() {
            panic!("Did not find my ticket in the input file");
        }
        if nearby_tickets.is_empty() {
            panic!("Did not find nearby tickets defined in the input file");
        }

        Input::new(fields, my_ticket.unwrap(), nearby_tickets)
    }

    pub fn nearby_error_rate(&self) -> u16 {
        self.nearby_tickets
            .iter()
            .map(|ticket| ticket.invalid_values(&self.fields).iter().sum::<u16>())
            .sum::<u16>()
    }
}

#[derive(Debug, PartialEq)]
struct Field {
    name: String,
    ranges: Vec<RangeInclusive<u16>>,
}

impl Field {
    pub fn new(name: String, ranges: Vec<RangeInclusive<u16>>) -> Field {
        Field { name, ranges }
    }

    pub fn contains(&self, val: &u16) -> bool {
        self.ranges.iter().any(|range| range.contains(&val))
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    values: Vec<u16>,
}

impl Ticket {
    pub fn new(values: Vec<u16>) -> Ticket {
        Ticket { values }
    }

    pub fn invalid_values(&self, fields: &Vec<Field>) -> Vec<u16> {
        self.values
            .iter()
            .filter_map(|value| {
                if !fields.iter().any(|field| field.contains(value)) {
                    Some(*value)
                } else {
                    None
                }
            })
            .collect::<Vec<u16>>()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input_from_raw() {
        let input = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ];
        assert_eq!(
            Input::from_raw(
                input
                    .iter()
                    .map(|line| line.to_string())
                    .collect::<Vec<String>>()
            ),
            Input::new(
                vec![
                    Field::new("class".to_string(), vec![1..=3, 5..=7]),
                    Field::new("row".to_string(), vec![6..=11, 33..=44]),
                    Field::new("seat".to_string(), vec![13..=40, 45..=50])
                ],
                Ticket::new(vec![7, 1, 14]),
                vec![
                    Ticket::new(vec![7, 3, 47]),
                    Ticket::new(vec![40, 4, 50]),
                    Ticket::new(vec![55, 2, 20]),
                    Ticket::new(vec![38, 6, 12])
                ]
            )
        )
    }

    #[test]
    fn test_input_nearby_error_rate() {
        assert_eq!(
            Input::new(
                vec![
                    Field::new("class".to_string(), vec![1..=3, 5..=7]),
                    Field::new("row".to_string(), vec![6..=11, 33..=44]),
                    Field::new("seat".to_string(), vec![13..=40, 45..=50])
                ],
                Ticket::new(vec![7, 1, 14]),
                vec![
                    Ticket::new(vec![7, 3, 47]),
                    Ticket::new(vec![40, 4, 50]),
                    Ticket::new(vec![55, 2, 20]),
                    Ticket::new(vec![38, 6, 12])
                ]
            )
            .nearby_error_rate(),
            71
        )
    }

    #[test]
    fn test_field_contains_true1() {
        assert_eq!(
            Field::new("".to_string(), vec![1..=6, 8..=10]).contains(&4),
            true
        )
    }

    #[test]
    fn test_field_contains_true2() {
        assert_eq!(
            Field::new("".to_string(), vec![1..=6, 8..=10]).contains(&10),
            true
        )
    }

    #[test]
    fn test_field_contains_false1() {
        assert_eq!(
            Field::new("".to_string(), vec![1..=6, 8..=10]).contains(&0),
            false
        )
    }

    #[test]
    fn test_field_contains_false2() {
        assert_eq!(
            Field::new("".to_string(), vec![1..=6, 8..=10]).contains(&7),
            false
        )
    }

    #[test]
    fn test_field_contains_false3() {
        assert_eq!(
            Field::new("".to_string(), vec![1..=6, 8..=10]).contains(&11),
            false
        )
    }

    #[test]
    fn test_ticket_invalid_fields() {
        assert_eq!(
            Ticket::new(vec![4, 7, 10]).invalid_values(&vec![
                Field::new("".to_string(), vec![1..=6]),
                Field::new("".to_string(), vec![8..=10])
            ]),
            vec![7]
        )
    }
}
