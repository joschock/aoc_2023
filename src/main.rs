use std::fs::read_to_string;

fn get_neighbors(map: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x < map[0].len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y < map.len() - 1 {
        neighbors.push((x, y + 1));
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

fn generate_step_map(map: &Vec<Vec<char>>, start: (usize, usize), steps: usize) -> Vec<Vec<char>> {
    let (start_x, start_y) = start;
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
                                _ => {}
                            }
                        }
                        working_map[y][x] = '.'
                    }
                    '*' => {
                        for (n_x, n_y) in get_neighbors(&working_map, x, y) {
                            match working_map[n_y][n_x] {
                                '.' => working_map[n_y][n_x] = 'O',
                                'x' => working_map[n_y][n_x] = '*',
                                _ => {}
                            }
                        }
                        working_map[y][x] = 'O'
                    }
                    _ => {}
                }
            }
        }
    }
    working_map
}

fn count_map(map: &Vec<Vec<char>>) -> usize {
    let mut sum1 = 0;
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 'O' {
                sum1 += 1;
            }
        }
    }
    sum1
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    println!("--start---");
    //print_map(&map);

    let start_x = map.len()/2;
    let start_y = map.len()/2;

    assert_eq!(map[start_y][start_x], 'S');
    map[start_y][start_x] = '.';

    let step_map = generate_step_map(&map, (start_x, start_y), 64);
    println!("q1: {:?}", count_map(&step_map));

    //       a
    //      bfd
    //      ecg
    //     bcfcd
    //     efcfg
    //    bfcfcfd
    //    ecfcfcg
    //   bcfcfcfcd
    //   efcfcfcfg
    //  hfcfcfcfcfi
    //   lfcfcfcfm
    //   jcfcfcfck
    //    lcfcfcm
    //    jfcfcfk
    //     lfcfm
    //     jcfck
    //      lcm
    //      jfk
    //       n

    //steps = a + h + i + n; // points
    //steps += (b + d + e + g + j + k + l + m) * ((steps - map.len() /2)/map.len() - 1); //outside slopes.
    //steps += (f+c) * ((steps - map.len() /2)/map.len() - 1) * 4 + f //cross
    //base = ((steps - map.len() /2)/map.len() - 2)
    //steps += (f+c) * base(base+1)/2 * 4 //whole tile quadrants.

    let map_a = generate_step_map(&map, (start_x, step_map.len()-1), step_map.len()-1);
    //print_map(&map_a);
    let a = count_map(&map_a);
    println!("map a: {:?}", a);

    let map_b = generate_step_map(&map, (map.len()-1, map.len()-1), step_map.len()/2);
    //print_map(&map_b);
    let b = count_map(&map_b);
    println!("map b: {:?}", b);

    let map_c = generate_step_map(&map, (start_x, start_y), step_map.len()+1);
    //print_map(&map_c);
    let c = count_map(&map_c);
    println!("map c: {:?}", c);

    let map_d = generate_step_map(&map, (0, map.len()-1), step_map.len()/2);
    //print_map(&map_d);
    let d = count_map(&map_d);
    println!("map d: {:?}", d);

    let map_e = generate_step_map(&map, (map.len()-1, map.len()-1), step_map.len() + step_map.len()/2);
    //print_map(&map_e);
    let e = count_map(&map_e);
    println!("map e: {:?}", e);

    let map_f = generate_step_map(&map, (start_x, start_y), step_map.len());
    //print_map(&map_f);
    let f = count_map(&map_f);
    println!("map f: {:?}", f);

    let map_g = generate_step_map(&map, (0, map.len()-1), step_map.len() + step_map.len()/2);
    //print_map(&map_g);
    let g = count_map(&map_g);
    println!("map g: {:?}", g);

    let map_h = generate_step_map(&map, (map.len()-1, start_y), step_map.len() - 1);
    //print_map(&map_h);
    let h = count_map(&map_h);
    println!("map h: {:?}", h);

    let map_i = generate_step_map(&map, (0, start_y), step_map.len() - 1);
    //print_map(&map_i);
    let i = count_map(&map_i);
    println!("map i: {:?}", i);

    let map_j = generate_step_map(&map, (map.len()-1, 0), step_map.len()/2);
    //print_map(&map_j);
    let j = count_map(&map_j);
    println!("map j: {:?}", j);

    let map_k = generate_step_map(&map, (0, 0), step_map.len()/2);
    //print_map(&map_k);
    let k = count_map(&map_k);
    println!("map k: {:?}", k);

    let map_l = generate_step_map(&map, (map.len()-1, 0), step_map.len() + step_map.len()/2);
    //print_map(&map_l);
    let l = count_map(&map_l);
    println!("map l: {:?}", l);

    let map_m = generate_step_map(&map, (0, 0), step_map.len() + step_map.len()/2);
    //print_map(&map_m);
    let m = count_map(&map_m);
    println!("map m: {:?}", m);

    let map_n = generate_step_map(&map, (start_x, 0), step_map.len()-1);
    //print_map(&map_n);
    let n = count_map(&map_n);
    println!("map n: {:?}", n);

    let step_count = 26501365;

    //spots = a + h + i + n; // points
    //spots += (b + d + e + g + j + k + l + m) * ((step_count - map.len() /2)/map.len() - 1); //outside slopes.
    //spots += (f+c) * ((step_count - map.len() /2)/map.len() - 1) * 4 + f //cross
    //base = ((step_count - map.len() /2)/map.len() - 2)
    //spots += (f+c) * base(base+1)/2 * 4 //whole tile quadrants.
    let base = (step_count - map.len()/2)/map.len() - 1;
    let mut spots = a + h + i + n; //points
    spots += (b + d + e + g + j + k + l + m) * base; //outside slopes.
    spots += (f+c) * base * 4 + f; //cross.
    let qbase = base - 1;
    spots += (f+c) * (qbase * (qbase + 1))/2 *4; //quadrants.

    println!("spots: {:?}", spots);
}
