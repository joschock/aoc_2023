use std::{fs::read_to_string, collections::{BinaryHeap, BTreeSet}, cmp::Reverse};



fn dijkstra(map: &Vec<Vec<usize>>, start: (usize, usize), end: (usize, usize), min_straight: usize, max_straight: usize) -> usize {
    let mut frontier = BinaryHeap::new();
    let mut visited = BTreeSet::new();


    frontier.push(Reverse((0, (start.0, start.1, 0))));
    frontier.push(Reverse((0, (start.0, start.1, 1))));

    while !frontier.is_empty() {

        let (heat, (x, y, direction)) = frontier.pop().unwrap().0;

        if (x,y) == end {
            return heat;
        }

        if visited.contains(&(x,y,direction)) {
            continue;
        }

        visited.insert((x,y,direction));

        for sign in [-1, 1] {
            let mut new_heat = heat;
            let (mut new_x, mut new_y) = (x as isize, y as isize);
            for step in 1isize..(max_straight + 1) as isize {
                if direction == 1 {
                    new_x = x as isize + step * sign;
                } else {
                    new_y = y as isize + step * sign;
                }
                if new_x < 0 || new_y < 0 || new_x >= map[0].len() as isize || new_y >= map.len() as isize {
                    break;
                }

                let new_x = new_x as usize;
                let new_y = new_y as usize;

                new_heat += map[new_y][new_x];

                if visited.contains(&(new_x, new_y, 1 - direction)) {
                    continue;
                }

                if step >= min_straight as isize {
                    frontier.push(Reverse((new_heat, (new_x, new_y, 1 - direction))));
                }
            }
        }
    }
    unreachable!()
}


fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let map: Vec<Vec<usize>> = input.lines().map(|x|x.chars().map(|x|x.to_string().parse().unwrap()).collect()).collect();

    let result = dijkstra(&map, (0, 0), (map[0].len() -1 , map.len() -1), 1, 3);
    println!("q1: {:}", result);

    let result = dijkstra(&map, (0, 0), (map[0].len() -1 , map.len() -1), 4, 10);
    println!("q2: {:}", result);
}
