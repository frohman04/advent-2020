use std::collections::HashMap;

fn main() {
    let starter_numbers: Vec<usize> = vec![1, 0, 15, 2, 10, 13];
    println!("{:?}", chain(starter_numbers, 2020));
}

#[derive(Debug)]
struct Diff {
    pub prev: Option<usize>,
    pub next: Option<usize>,
}

impl Diff {
    pub fn new(curr: usize) -> Diff {
        Diff {
            prev: None,
            next: Some(curr),
        }
    }

    pub fn push(&mut self, i: usize) -> () {
        self.prev = self.next;
        self.next = Some(i);
    }

    pub fn diff(&self) -> Result<usize, ()> {
        if !(self.prev.is_some() && self.next.is_some()) {
            Err(())
        } else {
            Ok(self.next.unwrap() - self.prev.unwrap())
        }
    }
}

fn chain(starter_numbers: Vec<usize>, n: usize) -> usize {
    if n <= starter_numbers.len() {
        starter_numbers[n - 1]
    } else {
        let mut last_seen: HashMap<usize, Diff> = HashMap::new();
        last_seen.extend(
            starter_numbers
                .iter()
                .enumerate()
                .map(|(i, num)| (*num, Diff::new(i))),
        );
        let mut most_recent = *starter_numbers.last().unwrap();
        for i in starter_numbers.len()..n {
            // println!("{:?} {:?}", most_recent, last_seen);
            let next_num = last_seen
                .get(&most_recent)
                .unwrap()
                .diff()
                .map_or(0, |val| val);
            // println!("{:?}", next_num);

            if last_seen.contains_key(&next_num) {
                let next_diff = last_seen.get_mut(&next_num).unwrap();
                next_diff.push(i);
            } else {
                last_seen.insert(next_num, Diff::new(i));
            }

            most_recent = next_num;
        }
        most_recent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chain1() {
        assert_eq!(chain(vec![0, 3, 6], 1), 0)
    }

    #[test]
    fn test_chain2() {
        assert_eq!(chain(vec![0, 3, 6], 2), 3)
    }

    #[test]
    fn test_chain3() {
        assert_eq!(chain(vec![0, 3, 6], 3), 6)
    }

    #[test]
    fn test_chain4() {
        assert_eq!(chain(vec![0, 3, 6], 4), 0)
    }

    #[test]
    fn test_chain5() {
        assert_eq!(chain(vec![0, 3, 6], 5), 3)
    }

    #[test]
    fn test_chain6() {
        assert_eq!(chain(vec![0, 3, 6], 6), 3)
    }

    #[test]
    fn test_chain7() {
        assert_eq!(chain(vec![0, 3, 6], 7), 1)
    }

    #[test]
    fn test_chain8() {
        assert_eq!(chain(vec![0, 3, 6], 8), 0)
    }

    #[test]
    fn test_chain9() {
        assert_eq!(chain(vec![0, 3, 6], 9), 4)
    }

    #[test]
    fn test_chain10() {
        assert_eq!(chain(vec![0, 3, 6], 10), 0)
    }
}
