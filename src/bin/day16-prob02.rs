use std::collections::HashSet;
use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("src/bin/day16.txt")
        .map(|file| {
            file.lines()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    let input = Input::from_raw(input);
    println!(
        "{:?}",
        input
            .associate_fields()
            .into_iter()
            .enumerate()
            .filter(|(_, field)| field.name.starts_with("departure"))
            .map(|(i, _)| input.my_ticket.values[i] as u64)
            .product::<u64>()
    );
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

    pub fn associate_fields(&self) -> Vec<Field> {
        let valid_tickets = self
            .nearby_tickets
            .iter()
            .filter_map(|ticket| {
                if ticket.is_valid(&self.fields) {
                    Some(ticket.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Ticket>>();

        let mut matching_fields = valid_tickets
            .iter()
            .map(|ticket| {
                ticket
                    .values
                    .iter()
                    .map(|val| {
                        self.fields
                            .iter()
                            .map(|field| field.contains(val))
                            .collect::<Vec<bool>>()
                    })
                    .collect::<Vec<Vec<bool>>>()
            })
            .collect::<Vec<Vec<Vec<bool>>>>();

        let mut remaining_is: HashSet<u16> = (0..self.fields.len())
            .into_iter()
            .map(|i| i as u16)
            .collect();
        let mut ordered_fields: Vec<Option<Field>> = vec![None; self.fields.len()];
        for _ in 0..self.fields.len() {
            for field_i in 0..self.fields.len() {
                // find the values in the ticket with all tickets matching the current filter
                let val_checks = (0..matching_fields[0].len())
                    .map(|val_i| {
                        (0..valid_tickets.len())
                            .all(|ticket_i| matching_fields[ticket_i][val_i][field_i])
                    })
                    .collect::<Vec<bool>>();

                // if this field has a match, add it to the final set
                if val_checks.iter().filter(|c| **c).count() == 1 {
                    // figure out the index of the value that can be matched to a filter
                    let found_val_i = val_checks
                        .iter()
                        .enumerate()
                        .find(|(_, val)| **val)
                        .unwrap()
                        .0 as u16;
                    remaining_is.remove(&found_val_i);
                    ordered_fields[found_val_i as usize] = Some(self.fields[field_i].clone());

                    // update all tickets to mark the found value index as invalid
                    for f_field_i in 0..self.fields.len() {
                        for f_ticket_i in 0..valid_tickets.len() {
                            matching_fields[f_ticket_i][found_val_i as usize][f_field_i] = false;
                        }
                    }
                }
            }
        }

        ordered_fields
            .into_iter()
            .map(|field| field.expect("No solution found"))
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    values: Vec<u16>,
}

impl Ticket {
    pub fn new(values: Vec<u16>) -> Ticket {
        Ticket { values }
    }

    pub fn is_valid(&self, fields: &Vec<Field>) -> bool {
        self.values
            .iter()
            .all(|value| fields.iter().any(|field| field.contains(value)))
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
    fn test_input_associate_fields() {
        assert_eq!(
            Input::new(
                vec![
                    Field::new("class".to_string(), vec![0..=1, 4..=19]),
                    Field::new("row".to_string(), vec![0..=5, 8..=19]),
                    Field::new("seat".to_string(), vec![0..=13, 16..=19])
                ],
                Ticket::new(vec![11, 12, 13]),
                vec![
                    Ticket::new(vec![3, 9, 18]),
                    Ticket::new(vec![15, 1, 5]),
                    Ticket::new(vec![4, 15, 9])
                ]
            )
            .associate_fields(),
            vec![
                Field::new("row".to_string(), vec![0..=5, 8..=19]),
                Field::new("class".to_string(), vec![0..=1, 4..=19]),
                Field::new("seat".to_string(), vec![0..=13, 16..=19])
            ]
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
    fn test_ticket_is_valid_true() {
        assert_eq!(
            Ticket::new(vec![4, 10]).is_valid(&vec![
                Field::new("".to_string(), vec![1..=6]),
                Field::new("".to_string(), vec![8..=10])
            ]),
            true
        )
    }

    #[test]
    fn test_ticket_is_valid_false() {
        assert_eq!(
            Ticket::new(vec![4, 7, 10]).is_valid(&vec![
                Field::new("".to_string(), vec![1..=6]),
                Field::new("".to_string(), vec![8..=10])
            ]),
            false
        )
    }
}
