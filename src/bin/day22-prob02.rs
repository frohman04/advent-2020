use std::collections::{HashSet, VecDeque};

fn main() {
    let lines = std::fs::read_to_string("src/bin/day22.txt")
        .map(|file| {
            file.lines()
                .filter(|line| !line.is_empty())
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
        })
        .expect("Unable to open file");
    let (p1, p2) = parse_decks(lines);
    println!("{:?}", play_game(p1, p2));
}

fn parse_decks(lines: Vec<String>) -> (VecDeque<u8>, VecDeque<u8>) {
    let lines = lines
        .into_iter()
        .filter(|l| !l.is_empty() && !l.starts_with("Player"))
        .collect::<Vec<String>>();
    let player1 = lines[0..lines.len() / 2]
        .iter()
        .map(|val| {
            val.parse::<u8>()
                .expect(&format!("Unable to parse {} as u8", val))
        })
        .collect();
    let player2 = lines[lines.len() / 2..lines.len()]
        .iter()
        .map(|val| {
            val.parse::<u8>()
                .expect(&format!("Unable to parse {} as u8", val))
        })
        .collect();
    (player1, player2)
}

fn play_round(p1: &mut VecDeque<u8>, p2: &mut VecDeque<u8>) -> () {
    // println!("Player 1's deck: {:?}", p1);
    // println!("Player 2's deck: {:?}", p2);
    let top1 = p1
        .pop_front()
        .expect("Tried to play card from empty hand for p1");
    let top2 = p2
        .pop_front()
        .expect("Tried to play card from empty hand for p2");
    // println!("Player 1 plays: {}", top1);
    // println!("Player 2 plays: {}", top2);
    if top1 as usize <= p1.len() && top2 as usize <= p2.len() {
        // println!("Playing a sub-game to determine the winner...");
        // println!("=============================================");
        let p1_rec = p1
            .iter()
            .take(top1 as usize)
            .map(|v| v.clone())
            .collect::<VecDeque<u8>>();
        let p2_rec = p2
            .iter()
            .take(top2 as usize)
            .map(|v| v.clone())
            .collect::<VecDeque<u8>>();
        let (winner, _) = play_game(p1_rec, p2_rec);
        // println!("=============================================");
        if winner == 1 {
            // println!("Player 1 wins round");
            p1.push_back(top1);
            p1.push_back(top2);
        } else {
            // println!("Player 2 wins round");
            p2.push_back(top2);
            p2.push_back(top1);
        }
    } else {
        if top1 > top2 {
            // println!("Player 1 wins round");
            p1.push_back(top1);
            p1.push_back(top2);
        } else {
            // println!("Player 2 wins round");
            p2.push_back(top2);
            p2.push_back(top1);
        }
    }
    // println!();
}

fn play_game(p1: VecDeque<u8>, p2: VecDeque<u8>) -> (u8, u64) {
    let mut prev_hands: HashSet<(VecDeque<u8>, VecDeque<u8>)> = HashSet::new();
    let mut p1 = p1;
    let mut p2 = p2;
    while !p1.is_empty() && !p2.is_empty() {
        if prev_hands.contains(&(p1.clone(), p2.clone())) {
            // println!(
            //     "!!! INFINITE LOOP PREVENTER !!!\n  {:?}\n  {:?}\n  {:?}",
            //     p1, p2, prev_hands
            // );
            return (1, calc_score(p1));
        }
        prev_hands.insert((p1.clone(), p2.clone()));
        play_round(&mut p1, &mut p2);
    }
    if p1.is_empty() {
        (2, calc_score(p2))
    } else {
        (1, calc_score(p1))
    }
}

fn calc_score(winner: VecDeque<u8>) -> u64 {
    winner
        .into_iter()
        .rev()
        .enumerate()
        .fold(0u64, |acc, (i, card)| acc + (i as u64 + 1) * card as u64)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_decks() {
        assert_eq!(
            parse_decks(vec![
                "Player 1:".to_string(),
                "9".to_string(),
                "2".to_string(),
                "6".to_string(),
                "3".to_string(),
                "1".to_string(),
                "".to_string(),
                "Player 2:".to_string(),
                "5".to_string(),
                "8".to_string(),
                "4".to_string(),
                "7".to_string(),
                "10".to_string(),
            ]),
            (
                VecDeque::from(vec![9, 2, 6, 3, 1]),
                VecDeque::from(vec![5, 8, 4, 7, 10])
            )
        )
    }

    #[test]
    fn test_play_round_p1() {
        let mut p1 = VecDeque::from(vec![9, 2, 6, 3, 1]);
        let mut p2 = VecDeque::from(vec![5, 8, 4, 7, 10]);
        play_round(&mut p1, &mut p2);
        assert_eq!(
            (p1, p2),
            (
                VecDeque::from(vec![2, 6, 3, 1, 9, 5]),
                VecDeque::from(vec![8, 4, 7, 10])
            )
        )
    }

    #[test]
    fn test_play_round_p2() {
        let mut p1 = VecDeque::from(vec![2, 6, 3, 1, 9, 5]);
        let mut p2 = VecDeque::from(vec![8, 4, 7, 10]);
        play_round(&mut p1, &mut p2);
        assert_eq!(
            (p1, p2),
            (
                VecDeque::from(vec![6, 3, 1, 9, 5]),
                VecDeque::from(vec![4, 7, 10, 8, 2])
            )
        )
    }

    #[test]
    fn test_play_round_p9() {
        let mut p1 = VecDeque::from(vec![4, 9, 8, 5, 2]);
        let mut p2 = VecDeque::from(vec![3, 10, 1, 7, 6]);
        play_round(&mut p1, &mut p2);
        assert_eq!(
            (p1, p2),
            (
                VecDeque::from(vec![9, 8, 5, 2]),
                VecDeque::from(vec![10, 1, 7, 6, 3, 4])
            )
        )
    }

    #[test]
    fn test_play_round_p15() {
        let mut p1 = VecDeque::from(vec![1, 8, 3]);
        let mut p2 = VecDeque::from(vec![4, 10, 9, 7, 5, 6, 2]);
        play_round(&mut p1, &mut p2);
        assert_eq!(
            (p1, p2),
            (
                VecDeque::from(vec![8, 3]),
                VecDeque::from(vec![10, 9, 7, 5, 6, 2, 4, 1])
            )
        )
    }

    #[test]
    fn test_play_game() {
        assert_eq!(
            play_game(
                VecDeque::from(vec![9, 2, 6, 3, 1]),
                VecDeque::from(vec![5, 8, 4, 7, 10]),
            ),
            (2, 291)
        )
    }
}
