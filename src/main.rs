use std::{fs::read_to_string, iter::zip, collections::VecDeque};

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();

    let mut sequences: Vec<Vec<isize>> = Vec::new();
    for line in input.lines() {
        sequences.push(line.split_whitespace().map(|x|x.parse().unwrap()).collect());
    }

    //println!("Sequences: {:?}", sequences);

    let mut sum_back = 0;
    let mut sum_front = 0;
    for sequence in sequences {
        let mut diff_sequences: Vec<VecDeque<isize>> = Vec::new();
        let mut current_sequence: VecDeque<isize> = sequence.clone().into_iter().collect();
        diff_sequences.push(current_sequence.clone());
        loop {
            let a = current_sequence.iter();
            let b = current_sequence.iter().skip(1);
            let next_sequence: VecDeque<isize> = zip(a,b).map(|(x, y)| y - x).collect();
            //println!("next_sequence: {:?}", next_sequence);
            diff_sequences.push(next_sequence);
            if diff_sequences.last().unwrap().iter().all(|x|*x == 0) {
                break;
            }
            current_sequence = diff_sequences.last().unwrap().clone()
        }

        diff_sequences.reverse();

        for idx in 1..diff_sequences.len() {
            let next_back_val = diff_sequences[idx].back().unwrap() + diff_sequences[idx-1].back().unwrap();
            diff_sequences[idx].push_back(next_back_val);

            let next_front_val = diff_sequences[idx].front().unwrap() - diff_sequences[idx-1].front().unwrap();
            diff_sequences[idx].push_front(next_front_val);

        }
        //println!("front: {:}", diff_sequences.last().unwrap().front().unwrap());
        sum_back += diff_sequences.last().unwrap().back().unwrap();
        sum_front += diff_sequences.last().unwrap().front().unwrap();
    }
    println!("q1: {:}", sum_back);
    println!("q2: {:}", sum_front);

}


