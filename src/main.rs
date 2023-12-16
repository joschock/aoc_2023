use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn next_direction(current_tile: char, entered_from: Direction) -> Vec<Direction> {
    match current_tile {
        '.' => {
            match entered_from {
                Direction::Up => vec![Direction::Down],
                Direction::Down => vec![Direction::Up],
                Direction::Left => vec![Direction::Right],
                Direction::Right => vec![Direction::Left],
            }
        },
        '\\' => {
            match entered_from {
                Direction::Up => vec![Direction::Right],
                Direction::Down => vec![Direction::Left],
                Direction::Left => vec![Direction::Down],
                Direction::Right => vec![Direction::Up],
            }
        },
        '/' => {
            match entered_from {
                Direction::Up => vec![Direction::Left],
                Direction::Down => vec![Direction::Right],
                Direction::Left => vec![Direction::Up],
                Direction::Right => vec![Direction::Down]
            }
        },
        '|' => {
            match entered_from {
                Direction::Up => vec![Direction::Down],
                Direction::Down => vec![Direction::Up],
                Direction::Left => vec![Direction::Up, Direction::Down],
                Direction::Right => vec![Direction::Up, Direction::Down]
            }
        },
        '-' => {
            match entered_from {
                Direction::Up => vec![Direction::Left, Direction::Right],
                Direction::Down => vec![Direction::Left, Direction::Right],
                Direction::Left => vec![Direction::Right],
                Direction::Right => vec![Direction::Left]
            }
        },
        _=> panic!("eh?")
    }
}

fn follow_beam(mirror_map: &Vec<Vec<char>>, light_map: &mut Vec<Vec<Vec<Direction>>>, x: usize, y: usize, entered_from: Direction) {
    for direction in next_direction(mirror_map[y][x], entered_from) {
        if !light_map[y][x].contains(&direction) {
            light_map[y][x].push(direction.clone());
            match direction {
                Direction::Up => {
                    if y == 0 {
                        continue;
                    }
                    follow_beam(mirror_map, light_map, x, y-1, Direction::Down);
                },
                Direction::Down => {
                    if y == mirror_map.len() - 1 {
                        continue;
                    }
                    follow_beam(mirror_map, light_map, x, y+1, Direction::Up);
                },
                Direction::Left => {
                    if x == 0 {
                        continue;
                    }
                    follow_beam(mirror_map, light_map, x-1, y, Direction::Right);
                },
                Direction::Right => {
                    if x == mirror_map[0].len() - 1 {
                        continue;
                    }
                    follow_beam(mirror_map, light_map, x+1, y, Direction::Left);
                }
            }
        }
    }
}

fn print_light_map(light_map: &Vec<Vec<Vec<Direction>>>) {
    for row in light_map {
        for col in row {
            if col.len() > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn count_light_map(light_map: &Vec<Vec<Vec<Direction>>>) -> usize {
    let mut sum = 0;
    for row in light_map {
        for col in row {
            if col.len() > 0 {
                sum += 1;
            }
        }
    }
    sum
}

fn init_light_map(mirror_map: &Vec<Vec<char>>) -> Vec<Vec<Vec<Direction>>> {
    let mut light_map: Vec<Vec<Vec<Direction>>> = Vec::new();
    for row in 0..mirror_map.len() {
        light_map.push(Vec::new());
        for _col in 0..mirror_map[0].len() {
            light_map[row].push(Vec::new());
        }
    }
    light_map
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let mut light_map = init_light_map(&map);

    stacker::grow(0x10000000, ||{
        follow_beam(&map, &mut light_map, 0, 0, Direction::Left);
    });

    print_light_map(&light_map);

    println!("q1: {:}", count_light_map(&light_map));

    let mut best_beam_coverage = 0;
    for x in 0..map[0].len() {
        let mut light_map = init_light_map(&map);
        stacker::grow(0x10000000, ||{
            follow_beam(&map, &mut light_map, x, 0, Direction::Up);
        });
        let coverage = count_light_map(&light_map); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }

        let mut light_map = init_light_map(&map);
        stacker::grow(0x10000000, ||{
            follow_beam(&map, &mut light_map, x, map.len()-1, Direction::Down);
        });
        let coverage = count_light_map(&light_map); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }
    }

    for y in 0..map.len() {
        let mut light_map = init_light_map(&map);
        stacker::grow(0x10000000, ||{
            follow_beam(&map, &mut light_map, 0, y, Direction::Left);
        });
        let coverage = count_light_map(&light_map); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }

        let mut light_map = init_light_map(&map);
        stacker::grow(0x10000000, ||{
            follow_beam(&map, &mut light_map, map[0].len()-1, y, Direction::Right);
        });
        let coverage = count_light_map(&light_map); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }
    }

    println!("q2: {:}", best_beam_coverage);

}
