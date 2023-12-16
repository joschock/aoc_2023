use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use rayon::prelude::*;

use crossterm::ExecutableCommand;
use crossterm::cursor;
use crossterm::style;
use crossterm::style::Stylize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

fn follow_beam(mirror_map: &Vec<Vec<char>>, light_map: &Mutex<Vec<Vec<BTreeSet<Direction>>>>, x: usize, y: usize, entered_from: Direction, print: bool) {
    if print {
        let _guard = light_map.lock().unwrap();
        let mut stdout = std::io::stdout();
        let _ = stdout.execute(cursor::MoveTo(x as u16,y as u16));
        let _ = stdout.execute(style::PrintStyledContent(" ".with(style::Color::White).on(style::Color::Red)));
    }
    if print {
        sleep(Duration::from_micros(1));
    }
    let next_directions = next_direction(mirror_map[y][x], entered_from);
    next_directions.par_iter().for_each(|direction|{
        let contains = {
            let guard = light_map.lock().unwrap();
            guard[y][x].contains(&direction)
        };
        if !contains {
            {
                light_map.lock().unwrap()[y][x].insert(direction.clone());
            }
            match direction {
                Direction::Up => {
                    if y == 0 {
                        return;
                    }
                    stacker::maybe_grow(0x8000, 0x100000, ||{
                        follow_beam(mirror_map, light_map, x, y-1, Direction::Down, print);
                    });
                },
                Direction::Down => {
                    if y == mirror_map.len() - 1 {
                        return;
                    }
                    stacker::maybe_grow(0x8000, 0x100000, ||{
                        follow_beam(mirror_map, light_map, x, y+1, Direction::Up, print);
                    });
                },
                Direction::Left => {
                    if x == 0 {
                        return;
                    }
                    stacker::maybe_grow(0x8000, 0x100000, ||{
                        follow_beam(mirror_map, light_map, x-1, y, Direction::Right, print);
                    });
                },
                Direction::Right => {
                    if x == mirror_map[0].len() - 1 {
                        return;
                    }
                    stacker::maybe_grow(0x8000, 0x100000, ||{
                        follow_beam(mirror_map, light_map, x+1, y, Direction::Left, print);
                    });
                }
            }
        }
    });
}

fn _print_light_map(light_map: &Vec<Vec<BTreeSet<Direction>>>) {
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

fn print_map(mirror_map: &Vec<Vec<char>>) -> Result<(),std::io::Error>{
    let mut stdout = std::io::stdout();
    stdout.execute(cursor::MoveTo(0,0))?;

    for row in mirror_map {
        for col in row {
            match col {
                '.' => {stdout.execute(style::PrintStyledContent(" ".white()))?;},
                '\\' => {stdout.execute(style::PrintStyledContent("\\".dark_yellow()))?;},
                '/' => {stdout.execute(style::PrintStyledContent("/".dark_yellow()))?;},
                '|' => {stdout.execute(style::PrintStyledContent("|".dark_green()))?;},
                '-' => {stdout.execute(style::PrintStyledContent("-".dark_green()))?;},
                 _ => panic!("eh?")
            }
        }
        stdout.execute(cursor::MoveToNextLine(1))?;
    }
    Ok(())
}

fn count_light_map(light_map: &Vec<Vec<BTreeSet<Direction>>>) -> usize {
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

fn init_light_map(mirror_map: &Vec<Vec<char>>) -> Mutex<Vec<Vec<BTreeSet<Direction>>>> {
    let mut light_map: Vec<Vec<BTreeSet<Direction>>> = Vec::new();
    for row in 0..mirror_map.len() {
        light_map.push(Vec::new());
        for _col in 0..mirror_map[0].len() {
            light_map[row].push(BTreeSet::new());
        }
    }
    Mutex::new(light_map)
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let _ = print_map(&map);

    let light_map = init_light_map(&map);

    follow_beam(&map, &light_map, 0, 0, Direction::Left, true);
     let mut stdout = std::io::stdout();
     let _ = stdout.execute(cursor::MoveTo(0, map.len() as u16));
    //_print_light_map(&light_map.lock().unwrap());

    //println!("q1: {:}", count_light_map(&light_map.lock().unwrap()));

    let mut best_beam_coverage = 0;
    for x in 0..map[0].len() {
        let light_map = init_light_map(&map);
        follow_beam(&map, &light_map, x, 0, Direction::Up, false);
        let coverage = count_light_map(&light_map.lock().unwrap()); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }

        let light_map = init_light_map(&map);
        follow_beam(&map, &light_map, x, map.len()-1, Direction::Down, false);
        let coverage = count_light_map(&light_map.lock().unwrap()); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }
    }

    for y in 0..map.len() {
        let light_map = init_light_map(&map);
        follow_beam(&map, &light_map, 0, y, Direction::Left, false);
        let coverage = count_light_map(&light_map.lock().unwrap()); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }

        let light_map = init_light_map(&map);
        follow_beam(&map, &light_map, map[0].len()-1, y, Direction::Right, false);
        let coverage = count_light_map(&light_map.lock().unwrap()); {
            if coverage > best_beam_coverage {
                best_beam_coverage = coverage;
            }
        }
    }

    //println!("q2: {:}", best_beam_coverage);

}
