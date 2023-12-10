use std::{fs::read_to_string, collections::BTreeSet};

fn neighbors(x:usize, y:usize, max_x:usize, max_y:usize)  -> Vec<(usize, usize)> {
    let mut neighbors: Vec<_> = Vec::new();
    for (j, k) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        if let Some(n_y) = y.checked_add_signed(j) {
            if let Some(n_x) = x.checked_add_signed(k) {
                if n_y < max_y && n_x < max_x {
                    neighbors.push((n_x, n_y));
                }
            }
        }
    }
    neighbors
}

fn get_next_coords(map: &Vec<Vec<char>>, x:usize, y: usize) -> Vec<(usize, usize)> {
    //println!("  get_next_coords x {:}, y {:}, char: {:}", x, y, map[y][x]);
    let foo = match map[y][x] {
        '|' => vec![(x, y+1), (x, y-1)],
        '-' => vec![(x+1, y), (x-1, y)],
        'L' => vec![(x, y-1), (x+1, y)],
        'J' => vec![(x, y-1), (x-1, y)],
        '7' => vec![(x, y+1), (x-1, y)],
        'F' => vec![(x, y+1), (x+1, y)],
        'S' => {
            let start_neighbors = neighbors(x, y, map[0].len(), map.len());
            //println!("start_neighbors: {:?}", start_neighbors);
            start_neighbors.iter()
                .cloned()
                .filter(|(n_x,n_y)|{
                    get_next_coords(map, *n_x, *n_y).contains(&(x,y))
            }).collect()
         },
        _ => vec![]
    };
    //println!("  {:?}", foo);
    foo
}

fn mark_map(map: &mut Vec<Vec<char>>, loop_coords: &BTreeSet<(usize, usize)>, x: usize, y: usize) -> char {
    println!("recurse to {:?}, map:", (x, y));
    for row in &*map {
        println!("{:?}", row);
    }
    let mut found_edge = false;
    let orig_loop;
    if loop_coords.contains(&(x,y)) {
        orig_loop = Some(map[y][x]);
    } else {
        orig_loop = None;
    }
    //mark current "p" (in progress).
    if orig_loop.is_some() {
        map[y][x] = 'l';
    } else {
        map[y][x] = 'p';
    }

    //north: x+0, y-1
    if y == 0 { //base case, found an edge.
        found_edge = true;
    } else {
        //
        'check_north: {
            if map[y-1][x] == 'p' || map[y-1][x] == 'l' {
                //cycle
                break 'check_north;
            }
            if loop_coords.contains(&(x, y-1)) {
                //moving north, can't pass '-' that is part of the loop.
                if map[y-1][x] == '-' {
                    break 'check_north;
                }
            }
            //recurse
            println!("head north from {:?} orig_loop: {:?}", (x, y), orig_loop);
            if mark_map(map, loop_coords, x, y-1) == 'O' {
                found_edge = true;
            }
        }
    }

    //south: x+0, y+1
    if y == map.len()-1 { //base case, found an edge.
        found_edge = true;
    } else {
        //
        'check_south: {
            if map[y+1][x] == 'p' || map[y+1][x] == 'l' {
                //cycle
                break 'check_south;
            }
            if loop_coords.contains(&(x, y+1)) {
                println!("next: {:?} = {:}", (x, y+1), map[y+1][x]);
                //moving south, can't pass '-' that is part of the loop.
                if map[y+1][x] == '-' {
                    break 'check_south;
                }
            }
            //recurse
            println!("head south from {:?} orig_loop: {:?}", (x, y), orig_loop);
            if mark_map(map, loop_coords, x, y+1) == 'O' {
                found_edge = true;
            }
        }
    }

    //east: x+1, y
    if x == map[0].len()-1 { //base case, found an edge.
        found_edge = true;
    } else {
        //
        'check_east: {
            if map[y][x+1] == 'p' || map[y][x+1] == 'l' {
                //cycle
                break 'check_east;
            }
            if loop_coords.contains(&(x+1, y)) {
                //moving east, can't pass '|' that is part of the loop.
                if map[y][x+1] == '|' {
                    break 'check_east;
                }
            }
            //recurse
            println!("head east from {:?} orig_loop: {:?}", (x, y), orig_loop);
            if mark_map(map, loop_coords, x+1, y) == 'O' {
                found_edge = true;
            }
        }
    }

    //west: x-1, y
    if x == 0 { //base case, found an edge.
        found_edge = true;
    } else {
        'check_west: {
            if map[y][x-1] == 'p' || map[y][x-1] == 'l' {
                //cycle
                break 'check_west;
            }
            if loop_coords.contains(&(x-1, y)) {
                //moving east, can't pass '|' that is part of the loop.
                if map[y][x-1] == '|' {
                    break 'check_west;
                }
            }
            //recurse
            println!("head west from {:?} orig_loop: {:?}", (x, y), orig_loop);
            if mark_map(map, loop_coords, x-1, y) == 'O' {
                found_edge = true;
            }
        }
    }

    if let Some(val) = orig_loop {
        map[y][x] = val;
    }

    if found_edge {
        return 'O';
    } else {
        return 'p';
    }
}

fn mark_all_in_progress(map: &mut Vec<Vec<char>>, mark: char) {
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[x][y] == 'p' {
                map[x][y] = mark;
            }
        }
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let mut start_x = 0;
    let mut start_y = 0;
    'find_start: for (y, row) in map.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if *char == 'S' {
                start_x = x;
                start_y = y;
                break 'find_start;
            }
        }
    }
    println!("start: {:}:{:}", start_x, start_y);
    let mut loop_coords: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut current_coords = get_next_coords(&map, start_x, start_y);
    let mut prev_coords = vec![(start_x, start_y)];
    loop_coords.insert((start_x, start_y));
    for coord in &current_coords {
        loop_coords.insert(coord.clone());
    }
    for step in 0.. {
        //println!("{:}: current: {:?}", step, current_coords);
        let mut next_coords = Vec::new();
        for (x, y) in &current_coords {
            //println!(" char: {:}", map[*y][*x]);
            let next = get_next_coords(&map, *x, *y);
            //println!("   prev: {:?}, cur: {:?}, next:{:?}", prev_coords, current_coords, next);
            next_coords.push(
                next.iter()
                    .filter(|(n_x, n_y)|
                        !prev_coords.contains(&(*n_x, *n_y))
                    )
                    .nth(0).unwrap().clone()
            );
            loop_coords.insert(next_coords.last().unwrap().clone());
        }
        if next_coords[0] == next_coords[1] {
            println!("step: {:}", step+2);
            break;
        }
        prev_coords = current_coords;
        current_coords = next_coords;
    }

    let mut marked_map = map.clone();
    let mut outside_regions = 0;
    for x in 0..marked_map[0].len() {
        for y in 0..marked_map.len() {
            if loop_coords.contains(&(x, y)) {
                continue;
            }
            if marked_map[x][y] == 'I' || marked_map[x][y] == 'O' {
                continue;
            }
            let region = mark_map(&mut marked_map, &loop_coords, x, y);
            if region == 'O' {
                outside_regions +=1;
                //debug
                mark_all_in_progress(&mut marked_map, 'O');
            } else {
                mark_all_in_progress(&mut marked_map, 'I');
            }

        }
    }
    println!("marked map:");
    for row in marked_map {
        println!("{:?}", row);
    }
    println!("outside_regions: {:}", outside_regions);
}
