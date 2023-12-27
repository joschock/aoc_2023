use std::{fs::read_to_string, collections::{BTreeMap, BTreeSet}};

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

fn find_next_branch(
    map: &Vec<Vec<char>>,
    visited: &mut BTreeSet<(usize, usize)>,
    start: &(usize, usize),
    end: (usize, usize),
    slopes: bool
) -> Option<((usize, usize), usize)>
{
    //println!("   find_branch: {:?}", start);
    let mut current = *start;
    let mut current_path = vec![current];
    let mut candidates;
    loop {
        candidates = next_nodes(map, current.0, current.1, slopes);
        //println!("       candidates: {:?}, current: {:?}", candidates, current);
        candidates.retain(|x|!visited.contains(x) && !current_path.contains(x));
        if candidates.len() != 1  {
            break;
        }
        current = candidates.pop().unwrap();
        current_path.push(current);
    }
    //println!("   final candidates: {:?}, current: {:?}", candidates, current);

    if candidates.len() != 0 || current == end{
        Some(((current.0, current.1), current_path.len()))
    } else {
        None
    }
}


fn condense_map(
    map: &Vec<Vec<char>>,
    points: &mut BTreeMap<(usize, usize),BTreeSet<((usize, usize), usize)>>,
    visited: &mut BTreeSet<(usize, usize)>,
    next: (usize, usize),
    end: (usize, usize),
    slopes: bool
) {

    //println!("next: {:?}", next);
    let candidates = next_nodes(map, next.0, next.1, slopes);

    let candidates:BTreeSet<((usize, usize), usize)> = candidates.iter()
        .filter_map(
            |x|{
                find_next_branch(map, visited, x, end, slopes)
            }
        ).collect();

    //println!("  candidates: {:?}", candidates);
    if !points.contains_key(&next) {
        points.insert(next, BTreeSet::new());
    }

    for candidate  in candidates {
        visited.insert(candidate.0);
        if !points.contains_key(&candidate.0) {
            points.insert(candidate.0, BTreeSet::new());
        }

        points.get_mut(&next).unwrap().insert(candidate);
        points.get_mut(&candidate.0).unwrap().insert(((next.0, next.1),candidate.1));

        condense_map(map, points, visited, candidate.0, end, slopes);
        visited.remove(&candidate.0);
    }
}

fn plot_longest_route(
    points: &mut BTreeMap<(usize, usize),BTreeSet<((usize, usize), usize)>>,
    visited: &mut BTreeSet<(usize, usize)>,
    next: (usize, usize),
    end: (usize, usize),
) -> Option<(usize, Vec<(usize, usize)>)> {
    if visited.contains(&next) {
        return None;
    }

    if next == end {
        //println!("found end");
        return Some((next.1, vec![end]));
    }

    visited.insert(next);

    let mut longest_route = 0;
    let mut longest_path = Vec::new();
    let nodes = points.get(&next).unwrap().clone();
    //println!("next: {:?}\n\tnodes: {:?}", next, nodes);
    for node in nodes {
        //stacker::maybe_grow(0x8000, 0x100000, ||{
            if let Some((route_len, path)) = plot_longest_route(points, visited, node.0, end) {
                if route_len > longest_route  && path.contains(&end) {
                    longest_route = route_len;
                    longest_path = path;
                }
            }
        //});
    }

    visited.remove(&next);
    longest_path.push(next);
    Some((longest_route + next.1, longest_path))
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let start = (1,0);
    let end = (map[0].len()-2, map.len()-1);

    println!("start: {:?}, end: {:?}", start, end);
    println!("map start: {:?}, map end: {:?}", map[start.1][start.0], map[end.1][end.0]);

    let slopes = true;
    let mut visited:BTreeSet<(usize, usize)>  = BTreeSet::new();
    let first_branch = find_next_branch(&map, &mut visited, &start, end, slopes).unwrap();
    let mut condensed:BTreeMap<(usize, usize),BTreeSet<((usize, usize), usize)>> = BTreeMap::new();
    let mut first_branch_set = BTreeSet::new();
    first_branch_set.insert(first_branch);
    condensed.insert(start, first_branch_set);
    visited.insert(first_branch.0);
    condense_map(&map, &mut condensed, &mut visited, first_branch.0, end,slopes);

    for (key, value) in &condensed {
       println!("point: {:?}", key);
       println!("  edges: {:?}", value);
    }

    let (longest_route, mut path) = plot_longest_route(&mut condensed, &mut BTreeSet::new(), start, end).unwrap();
    path.reverse();
    println!("longest_route: {:}, {:}, {:?}", longest_route, path.len(), path);

    let mut sum = 0;
    for idx in 0..path.len()-1 {
        let node = path[idx];
        let next_node = path[idx+1];
        let edges = condensed.get(&node).unwrap();
        let edge = edges.iter().find(|(x, _)| *x == next_node).unwrap();
        sum += edge.1;
        println!("node: {:?} next: {:?} length: {:?}, sum: {:?}", node, next_node, edge.1, sum);
    }

}
