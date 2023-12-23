use std::fs::read_to_string;

fn next_nodes(map: &Vec<Vec<char>>, x: usize, y: usize, slopes: bool) -> Vec<(usize, usize)> {
    if slopes {
        //handle special slope case
        match map[y][x] {
            '^' => {
                if y > 0 {
                    if map[y-1][x] != '#' {
                        return vec![(x, y-1)];
                    }
                }
                return vec![];
            },
            'v' => {
                if y < map.len() - 1 {
                    if map[y+1][x] != '#' {
                        return vec![(x, y+1)];
                    }
                }
                return vec![];
            },
            '<' => {
                if x > 0 {
                    if map[y][x-1] != '#' {
                        return vec![(x-1, y)];
                    }
                }
                return vec![];
            },
            '>' => {
                if x < map[0].len() - 1 {
                    if map[y][x+1] != '#' {
                        return vec![(x+1, y)];
                    }
                }
                return vec![];
            },
            _ => {}
        }
    }

    let x = x as isize;
    let y = y as isize;

    let candidates = vec![
        (x-1, y),
        (x+1, y),
        (x, y-1),
        (x, y+1)
    ];

    candidates.iter().filter_map(|(x,y)| {
        if *x < 0 || map[0].len() as isize <= *x || *y < 0 || map.len() as isize <= *y {
            None
        } else {
            Some((*x as usize, *y as usize))
        }
    })
    .filter(|(x, y)| {
        map[*y][*x] != '#'
    })
    .collect()
}

fn plot_route(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>, end: (usize, usize), slopes: bool) -> usize {
    let (x, y) = *path.last().unwrap();
    //println!("{:?}", (x, y));

    if (x, y) == end {
        //return vec![(x,y)];
        return 1;
    }

    let mut routes = Vec::new();
    for node in next_nodes(map, x, y, slopes) {
        if !path.contains(&(node.0, node.1)) {
            let mut next_path = path.clone();
            next_path.push((node.0, node.1));
            routes.push(plot_route(map, &next_path, end, slopes));
        }
    }

    let mut max_route = 0;
    for route in routes {
        if route > max_route {
            max_route = route;
        }
    }

    max_route + 1
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let start = (1,0);
    let end = (map[0].len()-2, map.len()-1);

    println!("start: {:?}, end: {:?}", start, end);
    println!("map start: {:?}, map end: {:?}", map[start.1][start.0], map[end.1][end.0]);
    stacker::grow(0x10000000, ||{
        let longest_route = plot_route(&map, &vec![start], end, true);
        println!("q1: {:}", longest_route -1);
    });
}
