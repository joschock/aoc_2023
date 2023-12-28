//translated from approach described in https://aoc.csokavar.hu/?day=21
use std::{fs::read_to_string, collections::HashSet, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    y: isize,
    x: isize,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {y: self.y + rhs.y, x: self.x + rhs.x}
    }
}

impl Point {
    fn rem_euclid(self, rhs: isize) -> Self {
        Point {y: self.y.rem_euclid(rhs), x: self.x.rem_euclid(rhs)}
    }
}


fn step (valid_points: &HashSet<Point>, current_positions: &HashSet<Point>) -> HashSet<Point> {
    let directions: [Point; 4] = [
        Point{y:1, x:0},
        Point{y:-1, x:0},
        Point{y:0, x:1},
        Point{y:0, x:-1}];

    let mut next_positions: HashSet<Point> = HashSet::new();

    for position in current_positions {
        for direction in directions {
            let next_pos = *position + direction;
            let tile_pos = next_pos.rem_euclid(131);
            if valid_points.contains(&tile_pos) {
                next_positions.insert(next_pos);
            }
        }
    }
    next_positions
}

struct StepIterator {
    valid_points: HashSet<Point>,
    positions: HashSet<Point>
}

impl Iterator for StepIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let result = Some(self.positions.len());
        self.positions = step(&self.valid_points, &self.positions);
        result
    }
}


fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let valid_points: HashSet<Point> = map.iter()
        .enumerate()
        .flat_map(|(row, values)|{
            values.iter().enumerate().filter_map(move |(col, value)|{
                if *value != '#' {
                    Some(Point {y: row as isize, x: col as isize})
                } else {
                    None
                }
            })
        })
        .collect();
    let start_position: HashSet<Point> = [Point {x:65, y:65}].into_iter().collect();
    let step_iterator = StepIterator {valid_points: valid_points, positions: start_position};
    let steps: Vec<usize> = step_iterator.take(328).collect();

    println!("q1: {:?}", steps[64]);

    let (x0, y0) = (65 as f64, steps[65] as f64);
    let (x1, y1) = (196 as f64, steps[196] as f64);
    let (x2, y2) = (327 as f64, steps[327] as f64);

    let y01 = (y1 - y0)/(x1 -x0);
    let y12 = (y2 - y1)/(x2 - x1);
    let y012 = (y12 - y01) / (x2 - x0);

    let step_count:f64 = 26501365f64;

    let result = y0 + y01 * (step_count - x0) + y012 *(step_count - x0)*(step_count - x1);

    println!("q2: {:?}", result);

}
