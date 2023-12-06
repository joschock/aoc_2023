use std::{fs::read_to_string, iter::zip};

struct Record {
    time: usize,
    distance: usize,
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    let times: Vec<usize> = lines[0].split(":").nth(1).unwrap().split_whitespace().map(|x|x.parse().unwrap()).collect();
    let distances: Vec<usize> = lines[1].split(":").nth(1).unwrap().split_whitespace().map(|x|x.parse().unwrap()).collect();

    let races: Vec<Record> = zip(times.clone(), distances.clone()).map(|(time, distance)|Record{time, distance}).collect();

    let mut win_conditions: Vec<usize> = Vec::new();
    for race in races {
        let mut least_button = 0;
        for press in 0.. {
            let distance = press*(race.time - press);
            if distance > race.distance {
                least_button = press;
                break;
            }
        }
        let win_range = (race.time - least_button) - least_button + 1;
        println!("time: {:}, distance: {:}, win_range: {:}", race.time, race.distance, win_range);
        win_conditions.push(win_range);
    }
    println!("q1: {:}", win_conditions.iter().product::<usize>());

    let big_time:usize = times.iter().map(|x|x.to_string()).fold(String::new(), |x, y| x + &y).parse().unwrap();
    let big_distance:usize = distances.iter().map(|x|x.to_string()).fold(String::new(), |x, y| x + &y).parse().unwrap();
    println!("big_time: {:}", big_time);
    println!("big_distance: {:}", big_distance);

    let mut least_button = 0;
    for press in 0.. {
        let distance = press*(big_time - press);
        if distance > big_distance{
            least_button = press;
            break;
        }
    }
    let win_range = (big_time - least_button) - least_button + 1;
    println!("least_button: {:}, win_range: {:}", least_button, win_range);
    println!("q1: {:}", win_range);


}
