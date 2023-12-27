use std::{fs::read_to_string, collections::{BTreeMap, BTreeSet}, time::Instant};

fn next_nodes(map: &Vec<Vec<char>>, pos: (usize, usize), slopes: bool) -> Vec<(usize, usize)> {
    let (x, y) = pos;
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

#[derive(Debug)]
struct Edge {
    _a: (usize, usize),
    b: (usize, usize),
    weight: usize
}

#[derive(Debug)]
struct Node {
    _pos: (usize, usize),
    edges: Vec<Edge>
}

impl Node {
    fn degree(&self) -> usize {
        self.edges.len()
    }
}

fn map_to_graph(
    map: &Vec<Vec<char>>,
    visited: &mut BTreeSet<(usize, usize)>,
    next: (usize, usize),
    nodes: &mut BTreeMap<(usize, usize), Node>,
    slopes: bool
) {
    visited.insert(next);

    let mut node = Node {_pos: next, edges: Vec::new()};

    for neighbor in next_nodes(map, next, slopes) {
        node.edges.push(Edge {_a: next, b: neighbor, weight: 1});
        if !visited.contains(&neighbor) {
            stacker::maybe_grow(0x8000, 0x100000, ||{
                map_to_graph(map, visited, neighbor, nodes, slopes);
            });
        }
    }
    nodes.insert(next, node);
}

fn contract_graph(
    nodes: &mut BTreeMap<(usize, usize), Node>
) {

    let positions: Vec<(usize, usize)> = nodes.keys().cloned().collect();

    for pos in positions {
        if let Some(node) = nodes.get(&pos) {
            if node.degree() == 2 {
                //println!("node: {:?}", node);
                let node = nodes.remove(&pos).unwrap();
                let n_1 = node.edges[0].b;
                let n_1_weight = node.edges[1].weight;
                let n_2 = node.edges[1].b;
                let n_2_weight = node.edges[0].weight;

                let n_1_node = nodes.get_mut(&n_1).unwrap();
                //println!("  n1 before: {:?}", n_1_node);
                if let Some(n_1_edge) = n_1_node.edges.iter_mut().find(|x|x.b == pos) {
                    n_1_edge.b = n_2;
                    n_1_edge.weight += n_1_weight;
                }
                //println!("  n1 after: {:?}", n_1_node);

                let n_2_node = nodes.get_mut(&n_2).unwrap();
                //println!("  n2 before: {:?}", n_2_node);
                if let Some(n_2_edge) = n_2_node.edges.iter_mut().find(|x|x.b == pos) {
                    n_2_edge.b = n_1;
                    n_2_edge.weight += n_2_weight;
                }
                //println!("  n2 after: {:?}", n_2_node);
            }
        }
    }
}

fn plot_longest_route(
    nodes: &mut BTreeMap<(usize, usize), Node>,
    next: (usize, usize),
    end: (usize, usize)
) -> (usize, Vec<(usize, usize)>) {
    if next == end {
        return (0, vec![end]);
    }

    let mut longest = 0;
    let mut longest_path = Vec::new();
    if let Some(node) = nodes.remove(&next) {
        for edge in &node.edges {
            stacker::maybe_grow(0x8000, 0x100000, ||{
                let (length, path) = plot_longest_route(nodes, edge.b, end);
                if length  + edge.weight > longest && path.contains(&end){
                    longest = length  + edge.weight;
                    longest_path = path;
                }
            });
        }
        nodes.insert(next, node);
    }
    longest_path.push(next);
    (longest, longest_path)
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|x|x.chars().collect()).collect();

    let start = (1,0);
    let end = (map[0].len()-2, map.len()-1);

    println!("start: {:?}, end: {:?}", start, end);
    println!("map start: {:?}, map end: {:?}", map[start.1][start.0], map[end.1][end.0]);

    let slopes = false;
    let mut nodes = BTreeMap::new();
    println!("generating graph");
    let start_time = Instant::now();
    map_to_graph(&map, &mut BTreeSet::new(), start, &mut nodes, slopes);
    println!("map complete in {:?}", start_time.elapsed());

    println!("contracting graph edges");
    let start_time = Instant::now();
    contract_graph(&mut nodes);
    println!("contraction complete in {:?}", start_time.elapsed());
    //for node in &nodes {
    //    println!("{:?}", node);
    //}

    println!("plotting longest route");
    let start_time = Instant::now();
    let (longest, mut path) = plot_longest_route(&mut nodes, start, end);
    println!("plot complete in {:?}", start_time.elapsed());
    println!("longest: {:}", longest);
    path.reverse();
    println!("path: {:?}", path);
}
