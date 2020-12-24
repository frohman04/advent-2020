use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let cups = VecDeque::from(vec![3u32, 9, 8, 2, 5, 4, 7, 1, 6]);
    let next_two = steps(cups, 10_000_000);
    println!("{:?}", next_two);
    println!("{:?}", next_two.0 as u64 * next_two.1 as u64);
}

fn steps(cups: VecDeque<u32>, num: usize) -> (u32, u32) {
    let mut cups = cups;

    // fill cups to 1,000,000
    while cups.len() != 1_000_000 {
        cups.push_back(cups.len() as u32 + 1);
    }
    println!("filled list to 1M cups");

    // play game
    let start_time = Instant::now();
    for i in 0..num {
        step(&mut cups);

        if i % 1000 == 0 {
            let pct_complete = (i as f32 + 1f32) / num as f32;
            let duration = Instant::now() - start_time;
            let time_per_round = duration / (i as u32 + 1u32);
            let eta = time_per_round * num as u32;
            println!(
                "Completed step {} of {} ({}%) (duration: {:?}, time per round: {:?}, eta: {:?})",
                i + 1,
                num,
                pct_complete * 100f32,
                duration,
                time_per_round,
                eta - duration
            );
        }
    }

    // find cup #1 and drop it
    while *cups.front().unwrap() != 1 {
        cups.pop_front().unwrap();
    }
    cups.pop_front();

    // get the next two cups and return them
    (cups.pop_front().unwrap(), cups.pop_front().unwrap())
}

fn step(cups: &mut VecDeque<u32>) -> () {
    let num_cups = cups.len() as u32;

    let front = cups.pop_front().unwrap();

    let m1 = cups.pop_front().unwrap();
    let m2 = cups.pop_front().unwrap();
    let m3 = cups.pop_front().unwrap();

    let mut target = front - 1;
    if target == 0 {
        target = num_cups;
    }
    while target == m1 || target == m2 || target == m3 {
        target -= 1;
        if target == 0 {
            target = num_cups;
        }
    }

    let mut index = 0;
    for i in 0..cups.len() {
        if cups[i] == target {
            index = i;
            break;
        }
    }

    cups.insert(index + 1, m3);
    cups.insert(index + 1, m2);
    cups.insert(index + 1, m1);

    cups.push_back(front);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step1() {
        let mut cups = VecDeque::from(vec![3u32, 8, 9, 1, 2, 5, 4, 6, 7]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![2u32, 8, 9, 1, 5, 4, 6, 7, 3]))
    }

    #[test]
    fn test_step2() {
        let mut cups = VecDeque::from(vec![2u32, 8, 9, 1, 5, 4, 6, 7, 3]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![5u32, 4, 6, 7, 8, 9, 1, 3, 2]))
    }

    #[test]
    fn test_step3() {
        let mut cups = VecDeque::from(vec![5u32, 4, 6, 7, 8, 9, 1, 3, 2]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![8u32, 9, 1, 3, 4, 6, 7, 2, 5]))
    }

    #[test]
    fn test_step4() {
        let mut cups = VecDeque::from(vec![8u32, 9, 1, 3, 4, 6, 7, 2, 5]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![4u32, 6, 7, 9, 1, 3, 2, 5, 8]))
    }

    #[test]
    fn test_step5() {
        let mut cups = VecDeque::from(vec![4u32, 6, 7, 9, 1, 3, 2, 5, 8]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![1u32, 3, 6, 7, 9, 2, 5, 8, 4]))
    }

    #[test]
    fn test_step6() {
        let mut cups = VecDeque::from(vec![1u32, 3, 6, 7, 9, 2, 5, 8, 4]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![9u32, 3, 6, 7, 2, 5, 8, 4, 1]))
    }

    #[test]
    fn test_step7() {
        let mut cups = VecDeque::from(vec![9u32, 3, 6, 7, 2, 5, 8, 4, 1]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![2u32, 5, 8, 3, 6, 7, 4, 1, 9]))
    }

    #[test]
    fn test_step8() {
        let mut cups = VecDeque::from(vec![2u32, 5, 8, 3, 6, 7, 4, 1, 9]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![6u32, 7, 4, 1, 5, 8, 3, 9, 2]))
    }

    #[test]
    fn test_step9() {
        let mut cups = VecDeque::from(vec![6u32, 7, 4, 1, 5, 8, 3, 9, 2]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![5u32, 7, 4, 1, 8, 3, 9, 2, 6]))
    }

    #[test]
    fn test_step10() {
        let mut cups = VecDeque::from(vec![5u32, 7, 4, 1, 8, 3, 9, 2, 6]);
        step(&mut cups);
        assert_eq!(cups, VecDeque::from(vec![8u32, 3, 7, 4, 1, 9, 2, 6, 5]))
    }

    // test takes 45m to run :(
    // #[test]
    // fn test_steps() {
    //     assert_eq!(
    //         steps(VecDeque::from(vec![3u32, 8, 9, 1, 2, 5, 4, 6, 7]), 10_000_000),
    //         (934001, 159792)
    //     )
    // }
}
