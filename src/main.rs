use std::fs::read_to_string;

fn roll_north(map: &mut Vec<Vec<char>>) {
    for y in 1..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                let mut rock_placement = y-1;

                loop {
                    if map[rock_placement][x] == '.' {
                        map[rock_placement][x] = 'O';
                        map[rock_placement+1][x] = '.';
                        if rock_placement > 0 {
                            rock_placement-=1;
                            continue;
                        }
                    }
                    break;
                }
            }
        }
    }
}

fn transpose(src: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for col in 0..src[0].len() {
        let mut new_row: Vec<char> = Vec::new();
        for row in 0..src.len() {
            new_row.push(src[row][col]);
        }
        new_row.reverse();
        new_map.push(new_row);
    }
    //new_map.reverse();
    new_map
}

fn _print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{:?}", line);
    }
}

fn cycle(start_map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = start_map.clone();
    for _ in 0..4 {
        roll_north(&mut map);
        map = transpose(&map);
    }
    map
}

fn calc_load(map: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for (idx, line) in map.iter().enumerate() {
        let score = map.len() - idx;
        sum += line.iter().filter(|x|**x == 'O').count()*score;
    }
    sum
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();


    let mut q1_map = map.clone();
    roll_north(&mut q1_map);

    let mut sum1 = 0;
    for (idx, line) in q1_map.iter().enumerate() {
        let score = q1_map.len() - idx;
        sum1 += line.iter().filter(|x|**x == 'O').count()*score;
    }
    println!("q1: {:}", sum1);

    let mut q2_map = map.clone();

    println!("calculating first 1000");
    let mut results: Vec<usize> = Vec::new();
    for _ in 0..1000 {
        q2_map = cycle(&q2_map);
        let load = calc_load(&q2_map);
        results.push(load);
        //println!("calc_load: {:}", calc_load(&q2_map));
    }
    //assume 1000 is enough to settle to a periodic state.
    println!("looking for cycles");
     let mut cycle_len = 2;
     loop {
        if results[results.len() - 1] == results[results.len() - cycle_len] {
            break;
        }
        cycle_len += 1;
     }
     cycle_len -= 1;
     println!("cycle len: {:?}", cycle_len);
     //println!("cycle:   {:?}", &results[results.len() - cycle_len..]);
     //println!("cycle-1: {:?}", &results[results.len() - cycle_len *2..results.len() - cycle_len]);
     let mut cycle = results[results.len() - cycle_len..].to_vec();

     let mut first_cycle:usize = results.len();
     'outer: for (idx, chunk) in results.windows(cycle_len).enumerate() {
        for _ in 0..cycle_len {
            if chunk == cycle {
                first_cycle = idx;
                break 'outer;
            }
            cycle.rotate_right(1);
        }
     }
     println!("first cycle: {:?}", first_cycle);
     println!("cycle: {:?}", cycle);

     let final_load = (1000000000 - 1 - first_cycle) % cycle_len;
     println!("final_load idx: {:?}, final_load: {:?}", final_load, cycle[final_load]);

}
