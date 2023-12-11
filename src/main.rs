use std::fs::read_to_string;

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

fn _print_map(map: &Vec<Vec<char>>) {
    for row in map.iter() {
        for char in row.iter() {
            print!("{:}", char);
        }
        println!();
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    //println!("map:");
    //print_map(&map);

    let mut expanded_rows: Vec<Vec<char>> = Vec::new();
    for rows in map.iter() {
        expanded_rows.push(rows.clone());
        if rows.iter().all(|x|*x == '.') {
            expanded_rows.push(rows.clone())
        }
    }

    let mut expanded_map: Vec<Vec<char>> = Vec::new();
    for rows in transpose(&mut expanded_rows).iter() {
        expanded_map.push(rows.clone());
        if rows.iter().all(|x|*x=='.') {
            expanded_map.push(rows.clone());
        }
    }
    let expanded_map = transpose(&expanded_map);
    //println!("expanded map:");
    //print_map(&expanded_map);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for row in 0..expanded_map.len() {
        for col in 0..expanded_map[0].len() {
            if expanded_map[row][col] == '#' {
                galaxies.push((row,col));
            }
        }
    }

    let mut sum = 0;
    for galaxy in &galaxies {
        for other in &galaxies {
            let distance = (galaxy.0 as isize - other.0 as isize).abs() + (galaxy.1 as isize - other.1 as isize).abs();
            //println!("galaxy: {:?}, other: {:?} distance: {:}", galaxy, other, distance);
            sum += distance;
        }
    }
    println!("q1 sum: {:}", sum/2);

    let mut expanded_rows: Vec<Vec<char>> = Vec::new();
    for row in map.iter() {
        if row.iter().all(|x|*x == '.') {
            expanded_rows.push(vec!['e';map[0].len()]);
        } else {
            expanded_rows.push(row.clone());
        }
    }

    let mut expanded_map: Vec<Vec<char>> = Vec::new();
    for row in transpose(&expanded_rows).iter() {
        if row.iter().all(|x|*x == '.' || *x == 'e') {
            expanded_map.push(vec!['e';map[0].len()]);
        } else {
            expanded_map.push(row.clone());
        }
    }
    let expanded_map = transpose(&expanded_map);
    //println!("q2 map");
    //_print_map(&expanded_map);

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for row in 0..expanded_map.len() {
        for col in 0..expanded_map[0].len() {
            if expanded_map[row][col] == '#' {
                galaxies.push((row,col));
            }
        }
    }

    let mut sum: i64 = 0;
    for galaxy in &galaxies {
        for other in &galaxies {
            let row_start = std::cmp::min(galaxy.0, other.0);
            let row_end = std::cmp::max(galaxy.0, other.0);
            let mut row_distance = 0;
            for row in row_start+1..=row_end {
                match expanded_map[row][galaxy.1] {
                    'e' => row_distance += 1000000,
                    '.'|'#' => row_distance +=1,
                    _ => panic!("eh?")
                }
            }

            let col_start = std::cmp::min(galaxy.1, other.1);
            let col_end = std::cmp::max(galaxy.1, other.1);
            let mut col_distance = 0;
            for col in col_start+1..=col_end {
                match expanded_map[galaxy.0][col] {
                    'e' => col_distance += 1000000,
                    '.'|'#' => col_distance +=1,
                    _ => panic!("eh?")
                }
            }

            //let distance = (galaxy.0 as isize - other.0 as isize).abs() + (galaxy.1 as isize - other.1 as isize).abs();
            let distance = row_distance + col_distance;
            //println!("galaxy: {:?}, other: {:?} distance: {:}", galaxy, other, distance);
            sum += distance;
        }
    }
    println!("q2 sum: {:}", sum/2);

}
