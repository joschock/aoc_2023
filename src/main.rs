use std::{fs::read_to_string, fmt::format};

const STRINGS:&[(&str, usize)]  = &[
    ("1", 1),
    ("one", 1),
    ("2", 2),
    ("two", 2),
    ("3", 3),
    ("three", 3),
    ("4", 4),
    ("four", 4),
    ("5", 5),
    ("five", 5),
    ("6", 6),
    ("six", 6),
    ("7", 7),
    ("seven", 7),
    ("8", 8),
    ("eight", 8),
    ("9", 9),
    ("nine", 9),
    //("0", 0),
    //("zero", 0)
];

fn main() {
    let lines = read_to_string(".\\src\\test.txt").unwrap();
    // let mut sum = 0;
    // for line in lines.lines() {
    //     let first_digit = line.chars().find(|x| x.is_digit(10)).unwrap();
    //     let last_digit = line.chars().rev().find(|x| x.is_digit(10)).unwrap();
    //     let valstr = format!("{:}{:}", first_digit, last_digit);
    //     let value:usize = valstr.parse().unwrap();
    //     //println!("Val: {:}", value);
    //     sum += value;
    // }
    // println!("First Sum: {:}", sum);

    let mut sum = 0;
    for line in lines.lines() {
        let mut first_value = 0;
        let mut last_value = 0;
        let mut first_index = line.len();
        let mut last_index = 0;
        for (digit_str, val) in STRINGS {
            if let Some(idx) = line.find(digit_str) {
                if idx <= first_index {
                    first_index = idx;
                    first_value = *val;
                }
            }
            if let Some(idx) = line.rfind(digit_str) {
                if idx >= last_index {
                    last_index = idx;
                    last_value = *val;
                }
            }
        }
        if first_index == last_index {
            last_value = first_value;
        //    first_value = 0;
        }
        if last_index < first_index {
            panic!("eh?")
        }
        println!("{}, {}, {}", line, first_value, last_value);
        let value:usize = format!("{:}{:}", first_value, last_value).parse().unwrap();
        println!("Val: {:}", value);

        sum += value;
    }
    println!("Second Sum: {:}", sum);
}
