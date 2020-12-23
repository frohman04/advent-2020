use std::collections::VecDeque;

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
    let top1 = p1
        .pop_front()
        .expect("Tried to play card from empty hand for p1");
    let top2 = p2
        .pop_front()
        .expect("Tried to play card from empty hand for p2");
    if top1 > top2 {
        p1.push_back(top1);
        p1.push_back(top2);
    } else {
        p2.push_back(top2);
        p2.push_back(top1);
    }
}

fn play_game(p1: VecDeque<u8>, p2: VecDeque<u8>) -> u64 {
    let mut p1 = p1;
    let mut p2 = p2;
    while !p1.is_empty() && !p2.is_empty() {
        play_round(&mut p1, &mut p2);
    }
    let winner = if p1.is_empty() { p2 } else { p1 };
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
    fn test_play_game() {
        assert_eq!(
            play_game(
                VecDeque::from(vec![9, 2, 6, 3, 1]),
                VecDeque::from(vec![5, 8, 4, 7, 10])
            ),
            306
        )
    }
}
