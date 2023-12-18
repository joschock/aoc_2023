use std::{fs::read_to_string, collections::BTreeSet};

use crossterm::{self, ExecutableCommand};

fn get_next_nodes(map: &Vec<Vec<usize>>, current:(usize, usize)) -> Vec<(usize, usize)> {
    let mut next_nodes = Vec::new();
    let (x, y) = current;
    if x != 0 {
        next_nodes.push((x-1, y));
    }
    if x != map[0].len()-1 {
        next_nodes.push((x+1, y));
    }
    if y != 0 {
        next_nodes.push((x, y-1));
    }
    if y != map.len()-1 {
        next_nodes.push((x, y+1));
    }
    next_nodes
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeDistance {
    distance: usize,
    path: Vec<(usize, usize)>
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn get_next_paths(map: &Vec<Vec<usize>>, current:(usize, usize)) -> Vec<Vec<(usize,usize)>> {
    let mut paths = Vec::new();
    for node1 in get_next_nodes(map, current) {
        paths.push(vec![node1]);
        for node2 in get_next_nodes(map, node1) {
            paths.push(vec![node1, node2]);
            for node3 in get_next_nodes(map, node2) {
                paths.push(vec![node1, node2, node3]);
                for node4 in get_next_nodes(map, node3){
                    paths.push(vec![node1, node2, node3, node4])
                }
            }
        }
    }
    paths
}

fn get_path_distance(map: &Vec<Vec<usize>>, path: &Vec<(usize, usize)>) -> usize {
    let mut distance = 0;
    for (x, y) in path {
        distance += map[*y][*x];
    }
    distance
}

fn legal_route(path: &Vec<(usize,usize)>) -> bool {
    if path.len() < 4 {
        return true;
    }

    if path[0..4].iter().all(|(x,_y)|*x == path[0].0) {
        return false;
    }

    for window in path.windows(5){
        if window.iter().all(|(x,_y)|*x == window[0].0) {
            return false;
        }
        if window.iter().all(|(_x, y)|*y == window[0].1) {
            return false;
        }
    }
    for window in path.windows(3) {
        if window[0] == window[2] {
            return false;
        }
    }
    true
}

fn dijkstra(map: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> (Vec<Vec<usize>>,Vec<Vec<Vec<(usize, usize)>>>) {
    let mut unvisited: BTreeSet<(usize, usize)> = BTreeSet::new();
    let mut distances: Vec<Vec<usize>> = Vec::new();
    let mut source_route: Vec<Vec<Vec<(usize, usize)>>> = Vec::new();

    for y in 0..map.len() {
        distances.push(Vec::new());
        source_route.push(Vec::new());
        for x in 0..map[0].len() {
            unvisited.insert((x,y));
            distances[y].push(usize::MAX);
            source_route[y].push(Vec::new());
        }
    }

    let mut current = start;
    unvisited.remove(&current);
    distances[current.1][current.0] = 0;

    for idx in 0usize.. {
        for neighbor in &get_next_paths(&map, current) {
            let mut proposed_route = source_route[current.1][current.0].clone();
            proposed_route.extend(neighbor);
            if !legal_route(&proposed_route) {
                continue;
            }

            let neighbor_dst = neighbor.last().unwrap();
            let distance = distances[current.1][current.0].saturating_add(get_path_distance(map, neighbor));
            if distance < distances[neighbor_dst.1][neighbor_dst.0] {
                distances[neighbor_dst.1][neighbor_dst.0] = distance;
                source_route[neighbor_dst.1][neighbor_dst.0] = proposed_route;
            }
        }
        if idx % 1000 == 0 {
            print_distance_map(&distances, true, &BTreeSet::new());
        }
        unvisited.remove(&current);
        if current == end {
            break;
        }
        current = *unvisited.iter()
            .min_by(|a, b|distances[a.1][a.0].cmp(&distances[b.1][b.0]))
            .unwrap();
    }

    (distances, source_route)
}


fn print_distance_map(map: &Vec<Vec<usize>>, in_place: bool, path: &BTreeSet<(usize, usize)>) {
    let mut stdout = std::io::stdout();
    if in_place {
        let _ = stdout.execute(crossterm::cursor::SavePosition);
    }
    print!("     ");
    for x in 0..map[0].len() {
        print!("{:3} ", x)
    }
    println!();
    for y in 0..map.len() {
        print!("{:3}: ", y);
        for x in 0..map[0].len() {
            if path.contains(&(x,y)) {
                let _ = stdout.execute(crossterm::style::SetForegroundColor(crossterm::style::Color::Green));
            }
            if map[y][x] != usize::MAX {
                print!("{:3} ", map[y][x]);
            } else {
                print!("  * ");
            }
            let _ = stdout.execute(crossterm::style::SetForegroundColor(crossterm::style::Color::White));
        }
        println!();
    }
    if in_place {
        let _ = stdout.execute(crossterm::cursor::RestorePosition);
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<usize>> = input.lines().map(|x|x.chars().map(|x|x.to_string().parse().unwrap()).collect()).collect();

    let (distances, src_routes) = dijkstra(&map, (0,0), (map[0].len()-1, map.len() -1));
    let path_set:BTreeSet<(usize, usize)> = src_routes[map.len()-1][map[0].len()-1].iter().cloned().collect();
    print_distance_map(&distances, false, &path_set);
    println!("q1: {:?}", distances[map.len()-1][map[0].len()-1]);
}
