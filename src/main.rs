use std::{fs::read_to_string, ops::Range, iter::zip};

#[derive(Debug, Default)]
struct AlmanacMap {
    _from: String,
    _to: String,
    src_ranges: Vec<Range<usize>>,
    dst_ranges: Vec<Range<usize>>
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();

    let mut seeds: Vec<usize> = Vec::new();
    let mut maps: Vec<AlmanacMap> = Vec::new();

    let mut lines = input.lines();
    loop {
        if let Some(line) = lines.next() {
            if line.contains("seeds:") {
                seeds = line.split(":").nth(1).unwrap().split_whitespace().map(|x|x.parse().unwrap()).collect();
            } else if line.contains("map:") {
                let map_names:Vec<_> = line.split(" ").nth(0).unwrap().split("-").collect();
                let mut new_map = AlmanacMap {
                    _from: map_names[0].to_owned(),
                    _to: map_names[2].to_owned(),
                    ..Default::default()
                };

                loop {
                    if let Some(range) = lines.next() {
                        if range.trim() == "" {
                            break;
                        }
                        let range_info:Vec<usize> =  range.split_whitespace().map(|x|x.parse().unwrap()).collect();
                        new_map.dst_ranges.push(range_info[0]..range_info[0] + range_info[2]);
                        new_map.src_ranges.push(range_info[1]..range_info[1] + range_info[2]);
                    } else {
                        break;
                    }
                }
                maps.push(new_map);
            }
        } else {
            break;
        }
    }

    //println!("maps: {:#?}", maps);
    let mut lowest_distance = usize::MAX;

    for seed in seeds.clone() {
        let mut mapped_seed = seed;
        for map in &maps {
            //print!("mapped {:} to ", mapped_seed);
            for (src_range, dst_range) in zip(&map.src_ranges, &map.dst_ranges) {
                if src_range.contains(&mapped_seed) {
                    let offset = mapped_seed - src_range.start;
                    mapped_seed = dst_range.start + offset;
                    break;
                }
            }
            //println!("{:}", mapped_seed);
        }
        //println!("seed {:} -> {:}", seed, mapped_seed);
        if mapped_seed < lowest_distance {
            lowest_distance = mapped_seed;
        }
    }
    println!("q1 lowest location: {:}", lowest_distance);


    //println!("seeds: {:?}", seeds);

    maps.reverse();

    for location in 0.. {
        if location % 10000 == 0{
            print!(".")
        }
        let mut mapped_location = location;
        for map in &maps {
            //print!("mapped {:} to ", mapped_location);
            for (src_range, dst_range) in zip(&map.src_ranges, &map.dst_ranges) {
                if dst_range.contains(&mapped_location) {
                    let offset = mapped_location - dst_range.start;
                    mapped_location = src_range.start + offset;
                    break;
                }
            }
            //println!("{:}", mapped_location);
        }
        //println!("location {:} -> {:}", location, mapped_location);
        if seeds.chunks(2).any(|x| (x[0]..x[0]+x[1]).contains(&mapped_location)) {
            println!("\nq2 lowest location: {:}", location);
            break;
        }
    }
}
