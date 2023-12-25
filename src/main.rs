use std::{fs::read_to_string, collections::{BTreeSet, BTreeMap}};

fn merge_random_nodes(connections: &mut BTreeMap<String, Vec<String>>) {
    let u = connections.keys().nth(rand::random::<usize>() % connections.len()).unwrap().clone();
    let mut u_edges = connections.remove(&u).unwrap();

    //println!("u:{:}", u);
    //println!("u_edges:{:?}", u_edges);

    let v = u_edges[rand::random::<usize>() % u_edges.len()].clone();
    //println!("v:{:}", v);
    let mut v_edges = connections.remove( &v).unwrap();
    //println!("v_edges:{:?}", v_edges);

    u_edges.retain(|x| *x != u && *x != v);
    v_edges.retain(|x| *x != v && *x != u);

    let new_vertex = format!("{:},{:}", u, v);
    let mut new_edges = u_edges;
    new_edges.extend(v_edges);


    for key in connections.clone().keys() {
        if let Some(edge_list) = connections.get_mut(key) {
            for edge in edge_list.iter_mut() {
                if *edge == u || *edge == v {
                    *edge = new_vertex.clone();
                }
            }
        }
    }
    connections.insert(new_vertex, new_edges);
}

fn identify_groups_by_random_contraction(connections: &BTreeMap<String, BTreeSet<String>>) -> (usize, usize, usize) {
    //Karger's
    let mut merged_connections: BTreeMap<String, Vec<String>> = connections.clone().iter()
        .map(|(key, value)| (key.to_owned(), value.iter().cloned().collect::<Vec<String>>()))
        .collect();

    while merged_connections.len() > 2 {
        // for entry in &merged_connections {
        //     println!("{:?}", entry);
        // }
        // println!("------------");
        merge_random_nodes(&mut merged_connections);
    }

    let mut keys = merged_connections.keys();
    let size1 = keys.next().unwrap().split(",").count();
    let size2 = keys.next().unwrap().split(",").count();
    let links = merged_connections.first_key_value().unwrap().1.len();

    (size1, size2, links)
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();

    let mut connections: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for line in input.lines() {
        let mut split = line.split(": ");
        let a = split.next().unwrap().to_owned();
        let connected_set: BTreeSet<String> = split.next().unwrap().split_whitespace().map(|x|x.to_owned()).collect();
        if let Some(set) = connections.get_mut(&a) {
            set.extend(connected_set.clone());
        } else {
            connections.insert(a.clone(), connected_set.clone());
        }

        for element in connected_set {
            if let Some(set) = connections.get_mut(&element) {
                set.insert(a.clone());
            } else {
                connections.insert(element, [a.clone()].iter().cloned().collect());
            }
        }
    }

    let mut max_set = (0, 0, usize::MAX);

    for _test in 0..1000 {
        let sets = identify_groups_by_random_contraction(&connections);
        println!("current: {:?}, best: {:?} = {:}, ", sets, max_set, max_set.0*max_set.1);
        if sets.2 < max_set.2 {
            max_set = sets;
            if max_set.2 == 3 {
                break;
            }
        }
    }
    println!("q1 sets: {:?} = {:}", max_set, max_set.0*max_set.1);
}
