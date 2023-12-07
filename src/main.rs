use std::{fs::read_to_string, collections::BTreeMap, cmp::Ordering};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Clone,Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<char>,
    hand_type: HandType,
    bid: usize,
    jacks_wild: bool,
}

pub const CARDS: [char;14] = [
    'A',
    'K',
    'Q',
    'J',
    'T',
    '9',
    '8',
    '7',
    '6',
    '5',
    '4',
    '3',
    '2',
    'J',
];

fn card_to_value(card: &char, jacks_wild: bool) -> usize {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' =>
            if jacks_wild {
                1
            } else {
                11
            },
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("eh?")
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            for (idx, card) in self.cards.iter().enumerate() {
                let cmp = card_to_value(card, self.jacks_wild).cmp(&card_to_value(&other.cards[idx], other.jacks_wild));
                if !cmp.is_eq() {
                    return cmp;
                }
            }
            return Ordering::Equal;
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let hand_lines = input.lines();

    let mut hands: Vec<Hand> = Vec::new();

    for hand_str in hand_lines.clone() {
        let mut split = hand_str.split_whitespace();
        let cards: Vec<_> = split.next().unwrap().chars().collect();
        let bid: usize = split.next().unwrap().parse().unwrap();

        let mut counts: BTreeMap<char, usize> = BTreeMap::new();
        for card in cards.clone() {
            if let Some(count) = counts.get_mut(&card) {
                *count += 1;
            } else {
                counts.insert(card, 1);
            }
        }
        let hand_type;
        if counts.values().any(|x| *x==5) {
            hand_type = HandType::Five;
        } else if counts.values().any(|x| *x==4) {
            hand_type = HandType::Four;
        } else if counts.values().any(|x|*x==3) {
            if counts.values().any(|x|*x==2) {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::Three
            }
        } else if counts.values().filter(|x|**x==2).count() == 2 {
            hand_type = HandType::TwoPair;
        } else if counts.values().any(|x|*x==2) {
            hand_type = HandType::Pair;
        } else {
            hand_type = HandType::HighCard;
        }
        hands.push(Hand {cards, hand_type, bid, jacks_wild: false});
    }
    //println!("hands: {:#?}", hands);
    hands.sort();
    //println!("hands: {:#?}", hands);

    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += (rank + 1) * hand.bid;
    }

    println!("q1: {:}", winnings);
//---
    let mut hands: Vec<Hand> = Vec::new();
    for hand_str in hand_lines {
        let mut split = hand_str.split_whitespace();
        let cards: Vec<_> = split.next().unwrap().chars().collect();
        let bid: usize = split.next().unwrap().parse().unwrap();

        let mut counts: BTreeMap<char, usize> = BTreeMap::new();
        for card in CARDS {
            counts.insert(card, 0);
        }
        for card in cards.clone() {
            if let Some(count) = counts.get_mut(&card) {
                *count += 1;
            } else {
                panic!("eh?")
            }
        }
        let wilds = counts.remove(&'J').unwrap_or(0);
        let hand_type;
        //println!("counts: {:#?}", counts);
        if counts.values().any(|x| *x+wilds == 5){
            hand_type = HandType::Five;
        } else if counts.values().any(|x| *x+wilds==4) {
            hand_type = HandType::Four;
        } else if counts.values().any(|x|*x==3) {
            if counts.values().any(|x|*x==2) {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::Three;
            }
        } else if counts.values().any(|x|*x+wilds==3) {
            if counts.values().filter(|x|**x==2).count() == 2 {
                hand_type = HandType::FullHouse;
            } else {
                hand_type = HandType::Three
            }
        } else if counts.values().filter(|x|**x==2).count() == 2 {
            hand_type = HandType::TwoPair;
        } else if counts.values().any(|x|*x+wilds ==2) {
            hand_type = HandType::Pair;
        } else {
            hand_type = HandType::HighCard;
        }
        hands.push(Hand {cards, hand_type, bid, jacks_wild: true});
    }

    //println!("hands: {:#?}", hands);
    hands.sort();
    //println!("hands: {:#?}", hands);

    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        winnings += (rank + 1) * hand.bid;
    }

    println!("q2: {:}", winnings);

}
