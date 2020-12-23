use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

fn main() {
    let cups = VecDeque::from(vec![3u8, 9, 8, 2, 5, 4, 7, 1, 6]);
    println!("{:?}", steps(cups, 100));
}

fn steps(cups: VecDeque<u8>, num: usize) -> String {
    // play game
    let mut cups = cups;
    for _ in 0..num {
        step(&mut cups);
    }

    // find cup #1 and drop it
    while *cups.front().unwrap() != 1 {
        let front = cups.pop_front().unwrap();
        cups.push_back(front);
    }
    cups.pop_front();

    // convert remaining cups into string
    cups.into_iter().map(|c| c.to_string()).join("")
}

fn step(cups: &mut VecDeque<u8>) -> () {
    let front = cups.pop_front().unwrap();

    let m1 = cups.pop_front().unwrap();
    let m2 = cups.pop_front().unwrap();
    let m3 = cups.pop_front().unwrap();

    let indexes = cups
        .iter()
        .enumerate()
        .map(|(i, c)| (c.clone(), i))
        .collect::<HashMap<u8, usize>>();
    let mut index: Option<usize> = None;
    for offset in 1..10 {
        let target = (front + 10 - offset) % 10;
        if indexes.contains_key(&target) {
            index = Some(indexes[&target]);
            break;
        }
    }

    cups.insert(index.unwrap() + 1, m3);
    cups.insert(index.unwrap() + 1, m2);
    cups.insert(index.unwrap() + 1, m1);

    cups.push_back(front);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step1() {
        let mut cups = VecDeque::from(vec![3u8, 8, 9, 1, 2, 5, 4, 6, 7]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![2u8, 8, 9, 1, 5, 4, 6, 7, 3]))
    }

    #[test]
    fn test_step2() {
        let mut cups = VecDeque::from(vec![2u8, 8, 9, 1, 5, 4, 6, 7, 3]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![5u8, 4, 6, 7, 8, 9, 1, 3, 2]))
    }

    #[test]
    fn test_step3() {
        let mut cups = VecDeque::from(vec![5u8, 4, 6, 7, 8, 9, 1, 3, 2]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![8u8, 9, 1, 3, 4, 6, 7, 2, 5]))
    }

    #[test]
    fn test_step4() {
        let mut cups = VecDeque::from(vec![8u8, 9, 1, 3, 4, 6, 7, 2, 5]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![4u8, 6, 7, 9, 1, 3, 2, 5, 8]))
    }

    #[test]
    fn test_step5() {
        let mut cups = VecDeque::from(vec![4u8, 6, 7, 9, 1, 3, 2, 5, 8]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![1u8, 3, 6, 7, 9, 2, 5, 8, 4]))
    }

    #[test]
    fn test_step6() {
        let mut cups = VecDeque::from(vec![1u8, 3, 6, 7, 9, 2, 5, 8, 4]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![9u8, 3, 6, 7, 2, 5, 8, 4, 1]))
    }

    #[test]
    fn test_step7() {
        let mut cups = VecDeque::from(vec![9u8, 3, 6, 7, 2, 5, 8, 4, 1]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![2u8, 5, 8, 3, 6, 7, 4, 1, 9]))
    }

    #[test]
    fn test_step8() {
        let mut cups = VecDeque::from(vec![2u8, 5, 8, 3, 6, 7, 4, 1, 9]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![6u8, 7, 4, 1, 5, 8, 3, 9, 2]))
    }

    #[test]
    fn test_step9() {
        let mut cups = VecDeque::from(vec![6u8, 7, 4, 1, 5, 8, 3, 9, 2]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![5u8, 7, 4, 1, 8, 3, 9, 2, 6]))
    }

    #[test]
    fn test_step10() {
        let mut cups = VecDeque::from(vec![5u8, 7, 4, 1, 8, 3, 9, 2, 6]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![8u8, 3, 7, 4, 1, 9, 2, 6, 5]))
    }

    #[test]
    fn test_steps1() {
        assert_eq!(
            steps(VecDeque::from(vec![3u8, 8, 9, 1, 2, 5, 4, 6, 7]), 10),
            "92658374".to_string()
        )
    }

    #[test]
    fn test_steps2() {
        assert_eq!(
            steps(VecDeque::from(vec![3u8, 8, 9, 1, 2, 5, 4, 6, 7]), 100),
            "67384529".to_string()
        )
    }
}
