use std::fs::read_to_string;

fn get_neighbors(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        neighbors.push((x-1, y));
    }
    if x < map[0].len() - 1 {
        neighbors.push((x+1, y));
    }
    if y > 0 {
        neighbors.push((x, y-1));
    }
    if y < map.len() - 1 {
        neighbors.push((x, y+1));
    }

    neighbors
}

fn print_map(map: &Vec<Vec<char>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{:}", map[y][x]);
        }
        println!();
    }
}

fn _print_number_map(map: &Vec<Vec<usize>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{:3} ", map[y][x]);
        }
        println!();
    }
}

fn generate_step_map(map: &Vec<Vec<char>>, steps:usize) -> Vec<Vec<char>> {
    let (mut start_x, mut start_y) = (0,0);
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 'S' {
                (start_x, start_y) = (x, y);
                break;
            }
        }
    }

    let mut working_map = map.clone();
    working_map[start_y][start_x] = 'O';
    for _step in 0..steps {
        for x in 0..working_map[0].len() {
            for y in 0..working_map.len() {
                if working_map[y][x] == 'O' {
                    working_map[y][x] = 'x';
                }
            }
        }
        for x in 0..working_map[0].len() {
            for y in 0..working_map.len() {
                match working_map[y][x] {
                    'x' => {
                        for (n_x, n_y) in get_neighbors(&working_map, x, y) {
                            match working_map[n_y][n_x] {
                                '.' => working_map[n_y][n_x] = 'O',
                                'x' => working_map[n_y][n_x] = '*',
                                 _ => {},
                            }
                        }
                        working_map[y][x] = '.'
                    },
                    '*' => {
                        for (n_x, n_y) in get_neighbors(&working_map, x, y) {
                            match working_map[n_y][n_x] {
                                '.' => working_map[n_y][n_x] = 'O',
                                'x' => working_map[n_y][n_x] = '*',
                                _ => {},
                            }
                        }
                        working_map[y][x] = 'O'
                    },
                    _ => {},
                }
            }
        }
    }
    working_map
}

enum TileType {
    Left,
    Right,
    Center
}

fn number_map(map: &Vec<Vec<char>>, numbering_type:TileType) -> Vec<Vec<usize>> {
    let mut working_map: Vec<Vec<usize>> = Vec::new();

    for y in 0..map.len() {
        working_map.push(vec![0usize;map[0].len()]);
        match numbering_type {
            TileType::Left => {
                //sum from right to left.
                let mut sum = 0;
                for x in (0..map[0].len()).rev() {
                    if map[y][x] == 'O' {
                        sum+=1;
                    }
                    working_map[y][x] = sum;
                }
            },
            TileType::Right => {
                //sum from left to right.
                let mut sum = 0;
                for x in 0..map[0].len() {
                    if map[y][x] == 'O' {
                        sum+=1;
                    }
                    working_map[y][x] = sum;
                }
            },
            TileType::Center => {
                let mut sum_a = 0;
                let mut sum_b = 0;
                for x in 0..map[0].len()/2 {
                    if x <= map.len()/2 {
                        let x_a = map.len()/2 - x;
                        if map[y][x_a] == 'O' {
                            sum_a+=1;
                        }
                        working_map[y][x_a]=sum_a;
                    }

                    if map.len()/2 + x + 1 < map.len() {
                        let x_b = map.len()/2 + x + 1;
                        if map[y][x_b] == 'O' {
                            sum_b+=1;
                        }
                        working_map[y][x_b]=sum_b;
                    }
                }
            },
        }
    }

    working_map
}


fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let start_map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    println!("--start---");
    print_map(&start_map);

    let map = generate_step_map(&start_map, 64);

    let mut sum1 = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 'O' {
                sum1+=1;
            }
        }
    }
    println!("q1: {:?}", sum1);
    //print_map(&map);

    let odd_map = generate_step_map(&start_map, map.len()+2);
    println!("---odd---");
    print_map(&odd_map);
    let even_map = generate_step_map(&start_map, map.len()+3);
    println!("---even---");
    print_map(&even_map);

    let odd_left = number_map(&odd_map, TileType::Left);
    let odd_middle = number_map(&odd_map, TileType::Center);
    let odd_right = number_map(&odd_map, TileType::Right);

    //println!("---odd left---");
    //print_number_map(&odd_left);

    //println!("---odd middle---");
    //print_number_map(&odd_middle);

    //println!("---odd right---");
    //print_number_map(&odd_right);


    let even_left = number_map(&even_map, TileType::Left);
    let even_middle = number_map(&even_map, TileType::Center);
    let even_right = number_map(&even_map, TileType::Right);

    //println!("---even left---");
    //print_number_map(&even_left);

    //println!("---even middle---");
    //print_number_map(&even_middle);

    //println!("---even right---");
    //print_number_map(&even_right);


    let total_steps:usize  = 6;

    for idx in  (1..7).rev() {
        let map = generate_step_map(&start_map, idx);
        println!("---{:}---", idx);
        print_map(&map);
    }

    println!("total steps: {:}, mod map: {:}", total_steps, total_steps % map.len());

    let tiles = vec![
        vec![odd_left, odd_middle, odd_right],
        vec![even_left, even_middle, even_right],
    ];

    let mut sum = 0;

    for step in 0..total_steps {
        let tile_type = (step + total_steps) % 2;
        let down_y_offset = map.len() - (step + (map.len() - total_steps %map.len()) + map.len()/2 + 1) %map.len() - 1;
        let up_y_offset = (step + (map.len() - total_steps %map.len()) + map.len()/2 + 1) % map.len();
        let right_x_offset = (step + map.len()/2) % map.len();
        let left_x_offset = map.len() - right_x_offset - 1;
        let full_tile_count = step.saturating_sub(map.len()/2 + 1) / map.len();
        let on_full_count = full_tile_count / 2;
        let off_full_count = full_tile_count - on_full_count;
        println!("step:{:} tile_type: {:} up_y_offset: {:}, down_y_offset: {:} right_x_offset: {:}, left_x_offset {:}, full_tile_count {:}, on_full_count {:}, off_full_count {:}",step, tile_type, up_y_offset, down_y_offset, right_x_offset, left_x_offset, full_tile_count, on_full_count, off_full_count);

        if step == 0 {
            sum += tiles[tile_type][1][up_y_offset][right_x_offset];
            sum += tiles[tile_type][1][down_y_offset][right_x_offset];
        } else if step < map.len() /2 {
            sum += tiles[tile_type][1][up_y_offset][left_x_offset] + tiles[tile_type][1][up_y_offset][right_x_offset];
            sum += tiles[tile_type][1][down_y_offset][left_x_offset] + tiles[tile_type][1][down_y_offset][right_x_offset];
        } else {
            sum += tiles[tile_type][0][up_y_offset][left_x_offset] + tiles[tile_type][2][up_y_offset][right_x_offset];
            sum += tiles[1 - tile_type][0][up_y_offset][0] * off_full_count + tiles[tile_type][0][up_y_offset][0] * on_full_count;
            sum += tiles[1 - tile_type][1][up_y_offset][map.len() - 1] * off_full_count + tiles[tile_type][1][up_y_offset][map.len() - 1] * on_full_count;
            sum += tiles[tile_type][0][down_y_offset][left_x_offset] + tiles[tile_type][2][down_y_offset][right_x_offset];
            sum += tiles[1 - tile_type][0][down_y_offset][0] * off_full_count + tiles[tile_type][0][down_y_offset][0] * on_full_count;
            sum += tiles[1 - tile_type][1][down_y_offset][map.len() - 1] * off_full_count + tiles[tile_type][1][down_y_offset][map.len() - 1] * on_full_count;
        }

        println!("sum: {:}", sum);
    }
}
