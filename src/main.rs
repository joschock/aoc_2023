use std::{fs::read_to_string, collections::VecDeque};

#[derive(Clone, Debug)]
struct Card {
    id: usize,
    matches: usize
}

fn main() {
    let lines = read_to_string(".\\src\\test.txt").unwrap();
    let mut sum = 0;
    let mut cards: Vec<Card> = Vec::new();

    for (id, line) in lines.lines().enumerate() {
        let mut numbers = line.split("|");
        let winning_numbers = numbers.next().unwrap().split(":").nth(1).unwrap().trim();
        let my_numbers = numbers.next().unwrap().trim();
        let winning_numbers: Vec<usize> = winning_numbers.split_whitespace().map(|x|x.parse().unwrap()).collect();
        let my_numbers: Vec<usize> = my_numbers.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let matches = my_numbers.iter().filter(|x|winning_numbers.contains(x)).count();
        let score;
        if matches == 0 {
            score = 0;
        } else {
            score = 2usize.pow((matches - 1) as u32);
        }
        cards.push(Card {id: id+1, matches});
        //println!("score: {:}", score);
        sum += score;
    }
    println!("q1 sum: {:}", sum);

    let mut processed:usize = 0;
    let mut cards_to_process: VecDeque<Card> = cards.clone().into_iter().collect();
    loop {
        if let Some(card) = cards_to_process.pop_front() {
            processed+=1;
            if card.matches != 0 {
                let range_to_add = card.id..card.id+card.matches;
                //println!("range_to_add: {:?}", range_to_add);
                for copy in &cards[range_to_add] {
                    //println!("pushing card: {:?}", copy);
                    cards_to_process.push_back(copy.clone());
                }
            }
        } else {
            break;
        }
    }

    println!("q2 processed: {:}", processed);

}
