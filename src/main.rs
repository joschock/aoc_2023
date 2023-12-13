use std::{fs::read_to_string, cmp::min, iter::zip};

fn transpose(src: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut new_map: Vec<Vec<char>> = Vec::new();

    for col in 0..src[0].len() {
        let mut new_row: Vec<char> = Vec::new();
        for row in 0..src.len() {
            new_row.push(src[row][col]);
        }
        new_map.push(new_row);
    }
    new_map
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        println!("{:?}", line);
    }
}

fn compute_delta(map1: &Vec<Vec<char>>, map2: &Vec<Vec<char>>) -> usize {

    if map1.len() != map2.len() {
        panic!("eh?");
    }

    if map1[0].len() != map2[0].len() {
        panic!("eh?");
    }

    let mut delta = 0;

    for (line1, line2) in zip(map1.iter(), map2.iter()) {
        for (char1, char2) in zip(line1, line2) {
            if char1 != char2 {
                delta +=1
            }
        }
    }
    delta
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();

    let all_lines: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let mut maps: Vec<Vec<Vec<char>>> = Vec::new();
    let mut current_map: Vec<Vec<char>> = Vec::new();
    for line in all_lines {
        if line.is_empty() {
            maps.push(current_map);
            current_map = Vec::new();
        } else {
            current_map.push(line);
        }
    }
    maps.push(current_map);


    let mut sum1 = 0;
    for map in &maps{
        //print_map(&map);
        let mut row_idx = 0;
        for idx in 1..map.len() {
            let reflection_length = min(idx, map.len() - idx);

            let range1 = map[idx-reflection_length..idx].to_vec();
            let mut range2 = map[idx..idx+reflection_length].to_vec();
            range2.reverse();

            if range1 == range2 {
                row_idx = idx;
                break;
            }
        }

        let map = transpose(&map);
        let mut col_idx = 0;
        for idx in 1..map.len() {
            let reflection_length = min(idx, map.len() - idx);

            let range1 = map[idx-reflection_length..idx].to_vec();
            let mut range2 = map[idx..idx+reflection_length].to_vec();
            range2.reverse();

            if range1 == range2 {
                col_idx = idx;
                break;
            }
        }
        println!("part 1 map score: {:}, {:} = {:}", col_idx, row_idx, col_idx + 100* row_idx);
        sum1 += col_idx + 100* row_idx;
    }
    println!("q1: {:}", sum1);

    let mut sum2 = 0;
    for map in &maps{
        //print_map(&map);
        let mut row_idx = 0;
        for idx in 1..map.len() {
            let reflection_length = min(idx, map.len() - idx);

            let range1 = map[idx-reflection_length..idx].to_vec();
            let mut range2 = map[idx..idx+reflection_length].to_vec();
            range2.reverse();

            if compute_delta(&range1, &range2) == 1 {
                row_idx = idx;
                break;
            }
        }

        let map = transpose(&map);
        let mut col_idx = 0;
        for idx in 1..map.len() {
            let reflection_length = min(idx, map.len() - idx);

            let range1 = map[idx-reflection_length..idx].to_vec();
            let mut range2 = map[idx..idx+reflection_length].to_vec();
            range2.reverse();

            if compute_delta(&range1, &range2) == 1 {
                col_idx = idx;
                break;
            }
        }
        println!("part 2 map score: {:}, {:} = {:}", col_idx, row_idx, col_idx + 100* row_idx);
        sum2 += col_idx + 100* row_idx;
    }
    println!("q2: {:}", sum2);
}
