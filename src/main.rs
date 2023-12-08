use std::{fs::read_to_string, collections::BTreeMap, ptr::swap_nonoverlapping};

struct MapEntry {
    node: String,
    left_node: String,
    right_node: String,
}
fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let mut map_lines = input.lines();
    let path: Vec<_> = map_lines.next().unwrap().chars().collect();
    map_lines.next();

    let mut map: BTreeMap<String, MapEntry> = BTreeMap::new();

    for line in map_lines {
        let mut split = line.split("=");
        let node = split.next().unwrap().trim().to_owned();
        let next_nodes = split.next().unwrap().trim();
        let next_nodes = next_nodes[1..next_nodes.len()-1].to_owned();
        let mut node_split = next_nodes.split(",");
        let left_node = node_split.next().unwrap().trim().to_owned();
        let right_node = node_split.next().unwrap().trim().to_owned();
        map.insert(node.clone(), MapEntry {node, left_node, right_node});
    }

    if let Some(mut current_node) = map.get("AAA") {
        let mut path_count = 0;
        for idx in 0usize.. {
            //println!("current node: {:}, path: {:}", current_node.node, path[idx % path.len()]);
            let next_node = match path[idx % path.len()] {
                'L' => &current_node.left_node,
                'R' => &current_node.right_node,
                _ => panic!("eh?")
            };
            //println!("next node: {:}", next_node);
            path_count += 1;
            if next_node == "ZZZ" {
                break;
            }
            current_node = map.get(next_node).unwrap();
        }
        println!("q1: {:}", path_count);
    } else {
        println!("q1: no valid start node.");
    }

    let start_nodes: Vec<String> = map.keys().clone().filter(|x| x.ends_with("A")).cloned().collect();
    let mut cycle_counts: Vec<usize> = Vec::new();
    for node in start_nodes {
        let mut current_node = map.get(&node).unwrap();
        let mut path_count = 0;
        for idx in 0usize.. {
            //println!("current node: {:}, path: {:}", current_node.node, path[idx % path.len()]);
            let next_node = match path[idx % path.len()] {
                'L' => &current_node.left_node,
                'R' => &current_node.right_node,
                _ => panic!("eh?")
            };
            //println!("next node: {:}", next_node);
            path_count += 1;
            if next_node.ends_with("Z") {
                break;
            }
            current_node = map.get(next_node).unwrap();
        }
        cycle_counts.push(path_count);
    }
    println!("cycle_counts {:?}", cycle_counts);
    println!("q2: {:?}", cycle_counts.iter().fold(0, |acc, element| {
        if acc == 0 {
            *element
        } else {
            lcm(acc, *element)
        }
    }));

}

fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a > b { (a, b) } else {(b,a)};

    loop {
        let res = a % b;
        if res == 0 {
            return b;
        }
        a = b;
        b = res;
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd (a, b)
}
