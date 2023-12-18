use std::{fs::read_to_string, collections::BTreeSet};

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

#[derive(Debug, Clone)]
struct NodeDistance {
    distance: usize,
    path: Vec<(usize, usize)>
}

fn dijkstra(map: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<NodeDistance>> {
    let mut unvisited: BTreeSet<(usize,usize)> = BTreeSet::new();
    let mut distances: Vec<Vec<NodeDistance>> = Vec::new();
    for y in 0..map.len() {
        distances.push(Vec::new());
        for x in 0..map[0].len() {
            unvisited.insert((x,y));
            distances[y].push(NodeDistance { distance: usize::MAX, path: Vec::new()});
        }
    }

    let mut current = start;
    unvisited.remove(&current);
    distances[current.1][current.0] = NodeDistance { distance: 0, path: Vec::new()};

    for idx in 0.. {
        for neighbor in get_next_nodes(map, current).iter().filter(|x|unvisited.contains(*x)) {
            let current_path = &distances[current.1][current.0].path;

            if current_path.len() >= 2 && (
                current_path[current_path.len() - 2..].iter().all(|(x,_y)|*x == neighbor.0) ||
                current_path[current_path.len() - 2..].iter().all(|(_x,y)|*y == neighbor.1)
              )
            {
                continue;
            }

            let distance = map[neighbor.1][neighbor.0].saturating_add(distances[current.1][current.0].distance);
            if distance < distances[neighbor.1][neighbor.0].distance {
                let mut path = current_path.clone();
                path.push(current);
                distances[neighbor.1][neighbor.0] = NodeDistance {distance, path};
            }
        }

        println!("step {:}, current: {:?}", idx, current);
        print_distance_map(&distances);

        unvisited.remove(&current);
        if current == end {
            break;
        }
        current = *unvisited.iter().min_by(|a, b|{
            distances[a.1][a.0].distance.cmp(&distances[b.1][b.0].distance)
        }).unwrap();
    }

    distances
}

fn print_distance_map(map: &Vec<Vec<NodeDistance>>) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x].distance != usize::MAX {
                print!("{:3} ", map[y][x].distance);
            } else {
                print!("  * ");
            }
        }
        println!();
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<usize>> = input.lines().map(|x|x.chars().map(|x|x.to_string().parse().unwrap()).collect()).collect();

    let distances = dijkstra(&map, (0,0), (map[0].len()-1, map.len() -1));
    println!("q1: {:?}", distances[map.len()-1][map[0].len()-1]);
    print_distance_map(&distances);
}
