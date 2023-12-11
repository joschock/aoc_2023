use colored::Colorize;
use std::{fs::read_to_string, collections::BTreeSet};

fn neighbors(x:usize, y:usize, max_x:usize, max_y:usize)  -> Vec<(usize, usize, char)> {
    let mut neighbors: Vec<_> = Vec::new();
    for (x_shift, y_shift, direction) in [(-1, 0, 'w'), (0, -1, 'n'), (1, 0, 'e' ), (0, 1,'s')] {
        if let Some(n_y) = y.checked_add_signed(y_shift) {
            if let Some(n_x) = x.checked_add_signed(x_shift) {
                if n_y < max_y && n_x < max_x {
                    neighbors.push((n_x, n_y, direction));
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
            start_neighbors.iter()
                .cloned()
                .filter_map(|(n_x,n_y, _)|{
                    if get_next_coords(map, n_x, n_y).contains(&(x,y)) {
                        Some((n_x, n_y))
                    } else {
                        None
                    }
            }).collect()
         },
        _ => vec![]
    };
    //println!("  {:?}", foo);
    foo
}

fn get_start_pipe_type(map: &Vec<Vec<char>>, x: usize, y: usize) -> char {
    let start_neighbors = neighbors(x, y, map[0].len(), map.len());
    let start_neighbors: Vec<_> = start_neighbors.iter()
        .cloned()
        .filter(|(n_x,n_y, _)|{
            get_next_coords(map, *n_x, *n_y).contains(&(x,y))
    }).collect();

    //println!("start_neighbors: {:?}", start_neighbors);

    if start_neighbors.iter().any(|(_,_,dir)| *dir == 'n') &&
       start_neighbors.iter().any(|(_,_,dir)| *dir == 's')
    {
         return '|';
    }
    if start_neighbors.iter().any(|(_,_,dir)| *dir == 'e') &&
       start_neighbors.iter().any(|(_,_,dir)| *dir == 'w')
    {
         return '-';
    }
    if start_neighbors.iter().any(|(_,_,dir)| *dir == 'n') &&
       start_neighbors.iter().any(|(_,_,dir)| *dir == 'e')
    {
         return 'L';
    }

    if start_neighbors.iter().any(|(_,_,dir)| *dir == 'n') &&
       start_neighbors.iter().any(|(_,_,dir)| *dir == 'w')
    {
         return 'J';
    }

    if start_neighbors.iter().any(|(_,_,dir)| *dir == 's') &&
       start_neighbors.iter().any(|(_,_,dir)| *dir == 'w')
    {
         return '7';
    }

    if start_neighbors.iter().any(|(_,_,dir)| *dir == 's') &&
       start_neighbors.iter().any(|(_,_,dir)| *dir == 'e')
    {
         return 'F';
    }

    panic!("eh?")
}


fn generate_expanded_tile(expanded_map: &mut Vec<Vec<char>>, map: &Vec<Vec<char>>, x: usize, y: usize, pipe: char) {
    match pipe {
        '|' => {
            expanded_map[y*3+0][x*3..x*3+3].copy_from_slice(&['*','l','*']);
            expanded_map[y*3+1][x*3..x*3+3].copy_from_slice(&['*','l','*']);
            expanded_map[y*3+2][x*3..x*3+3].copy_from_slice(&['*','l','*']);
        },
        '-' => {
            expanded_map[y*3+0][x*3..x*3+3].copy_from_slice(&['*','*','*']);
            expanded_map[y*3+1][x*3..x*3+3].copy_from_slice(&['l','l','l']);
            expanded_map[y*3+2][x*3..x*3+3].copy_from_slice(&['*','*','*']);

        },
        'L' => {
            expanded_map[y*3+0][x*3..x*3+3].copy_from_slice(&['*','l','*']);
            expanded_map[y*3+1][x*3..x*3+3].copy_from_slice(&['*','l','l']);
            expanded_map[y*3+2][x*3..x*3+3].copy_from_slice(&['*','*','*']);

        },
        'J' => {
            expanded_map[y*3+0][x*3..x*3+3].copy_from_slice(&['*','l','*']);
            expanded_map[y*3+1][x*3..x*3+3].copy_from_slice(&['l','l','*']);
            expanded_map[y*3+2][x*3..x*3+3].copy_from_slice(&['*','*','*']);

        },
        '7' => {
            expanded_map[y*3+0][x*3..x*3+3].copy_from_slice(&['*','*','*']);
            expanded_map[y*3+1][x*3..x*3+3].copy_from_slice(&['l','l','*']);
            expanded_map[y*3+2][x*3..x*3+3].copy_from_slice(&['*','l','*']);

        },
        'F' => {
            expanded_map[y*3+0][x*3..x*3+3].copy_from_slice(&['*','*','*']);
            expanded_map[y*3+1][x*3..x*3+3].copy_from_slice(&['*','l','l']);
            expanded_map[y*3+2][x*3..x*3+3].copy_from_slice(&['*','l','*']);

        },
        'S' => {
            let start_pipe = get_start_pipe_type(map, x, y);
            //println!("Start Pipe: {:}", start_pipe);
            generate_expanded_tile(expanded_map, map, x, y, start_pipe);
        }
        _=> panic!("eh?")
    }
}

fn color_map(map: &mut Vec<Vec<char>>, x: usize, y: usize)  {
    //base case: pipe loop
    if map[y][x] == 'l' {
        return;
    }
    //base case: already visited
    if map[y][x] == 'O' {
        return;
    }

    //println!("recurse {:?}", (x, y));
    //print_map(map);

    map[y][x] = 'O';

    // check each direction, recurse if appropriate.
    //north: x, y-1
    if y != 0 {
        color_map(map, x, y-1);
    }

    //east: x+1, y
    if x+1 != map[0].len() {
        color_map(map, x+1, y);
    }

    //south: x, y+1
    if y+1 != map.len() {
        color_map(map, x, y+1);
    }

    //west: x-1, y
    if x != 0 {
        color_map(map, x-1, y);
    }
}

fn print_map (map: &Vec<Vec<char>>) {
    for row in map {
        for col in row {
            let string = match col {
                'O' => "O".blue(),
                'I' => "I".red(),
                'l' => "l".bright_green(),
                '*' => "*".green(),
                 x => x.to_string().white()
            };
            print!("{:}", string);
        }
        println!();
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

    //pt2 approach 1: expand and fill
    let mut expanded_map: Vec<Vec<char>> = vec![vec!['I'; map[0].len()*3]; map.len()*3];
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if loop_coords.contains(&(x,y)) {
                generate_expanded_tile(&mut expanded_map, &map, x, y, map[y][x]);
            }
        }
    }
    println!("expanded map:");
    print_map(&expanded_map);

    //assumes 0,0 is outside the loop (it is for the given problem inputs)
    //for big maps, this blows the stack. Use stacker to compensate instead of rewriting the whole thing on the heap.
    stacker::grow(0x10000000, ||{
        color_map(&mut expanded_map, 0, 0);
    });

    println!("\nexpanded colored map:");
    print_map(&expanded_map);

    let inner_tiles = expanded_map.iter().flatten().filter(|x| **x=='I').count() / 9;
    println!("q2 expand and fill inner_tiles: {:}\n", inner_tiles);

    //pt2 approach2: row scan
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum ScanState {
        Outside,
        Inside
    }

    let mut row_scan_map: Vec<Vec<char>> = map.clone();
    row_scan_map[start_y][start_x] = get_start_pipe_type(&map, start_x, start_y);
    println!("row_scan map:");
    print_map(&row_scan_map);
    let mut inner_tiles = 0;
    for (y, row) in row_scan_map.iter_mut().enumerate() {
        let mut scan_state: ScanState = ScanState::Outside;
        let mut x = 0;
        while x < row.len() {
            if !loop_coords.contains(&(x, y)) {
                match scan_state {
                    ScanState::Inside => {row[x] = 'I'; inner_tiles += 1;}
                    ScanState::Outside => {row[x] = 'O'; }
                }
            } else {
                let mut toggle = false;
                if row[x] == '|' {
                    toggle = true;
                } else {
                    let pipe_start = row[x];
                    while x+1 < row.len() && row[x+1] == '-' {
                        x+=1;
                    }
                    x+=1;
                    let pipe_end = row[x];
                    match (pipe_start, pipe_end)  {
                        ('F','7') | ('L', 'J') => (), // no change.
                        ('F','J') | ('L', '7') => toggle = true,
                        _ => panic!("eh?")
                    }
                }
                if toggle {
                    if scan_state == ScanState::Outside {
                        scan_state = ScanState::Inside;
                    } else {
                        scan_state = ScanState::Outside;
                    }
                }
            }
            x+=1;
        }
    }
    println!("\ncolored row_scan map:");
    print_map(&row_scan_map);
    println!("q2 row scan inner_tiles: {:}\n", inner_tiles);
}
