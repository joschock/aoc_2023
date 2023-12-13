use cache_macro::cache;
use lru_cache::LruCache;

use std::{fs::read_to_string, io::Write, time::Instant};

#[derive(Debug)]
struct Spring {
    map: String,
    list: Vec<usize>
}

#[cache(LruCache : LruCache::new(200000))]
fn process_spring(remaining_map: &mut String, remaining_list: &mut Vec<usize>) -> u64 {

    //println!("\nremaining_map: {:?}\nremaining_list: {:?}", remaining_map, remaining_list);
    let mut next_map = remaining_map.clone();
    let mut next_list = remaining_list.clone();

    loop {
        //println!("  next_map: {:?}, next_list: {:?}", next_map, next_list);
        let idx = next_map.find(|x|x != '.').unwrap_or(next_map.len());
        if idx == next_map.len() {
            //base case - string ends with '.'
            if next_list.len() == 0 {
                return 1;
            }
            return 0;
        }
        next_map = next_map[idx..].to_owned();
        //println!( "  idx {:} next_map: {:?}", idx, next_map);
        let idx = next_map.find(|x|x == '.').unwrap_or(next_map.len());
        let element = next_map[..idx].to_owned();
        //println !("    element: {:?}", element);
        if !element.contains("?") {
            if next_list.is_empty() {
                return 0; // base case - list is empty, but we found a broken spring.
            }
            if element.len() != next_list[0] {
                return 0; // base case - next list element doesn't match spring.
            }
            next_list = next_list[1..].to_vec();
            next_map = next_map[element.len()..].to_owned();
        } else {
            break; //recursive case - next element contains a '?'
        }
    }
    let qidx = next_map.find('?').unwrap();
    //println!(       "recurse: qidx {:}", qidx);

    //assumes ascii
    unsafe {
        next_map.as_bytes_mut()[qidx] = '#' as u8;
    };
    let res1 = process_spring(&mut next_map, &mut next_list);

    unsafe {
        next_map.as_bytes_mut()[qidx] = '.' as u8;
    };
    let res2 = process_spring(&mut next_map, &mut next_list);
    unsafe {
        next_map.as_bytes_mut()[qidx] = '?' as u8;
    };
    return res1+res2
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let mut spring_maps: Vec<Vec<Spring>> = Vec::new();

    for line in input.lines() {
        let mut line_split = line.split_whitespace();
        let mut map: Vec<char> = line_split.next().unwrap().chars().collect();
        let mut list: Vec<usize> = line_split.next().unwrap().split(",").map(|x|x.parse().unwrap()).collect();

        let mut line_maps: Vec<Spring> = Vec::new();

        //push bare map
        let bare = map.clone();
        let bare:String = bare.iter().collect();
        line_maps.push(Spring{ map: bare, list: list.clone()});

        //push bare map + ?
        //let mut bare_plus = map.clone();
        //bare_plus.push('?');
        //let bare_plus:String = bare_plus.iter().collect();
        //line_maps.push(Spring{ map: bare_plus.trim_end_matches('.').to_owned(), list: list.clone()});

        let map_len = map.len();
        let list_len = list.len();
        for _x in 0..4 {
             map.push('?');
             map.extend_from_within(..map_len);
             list.extend_from_within(..list_len);

             let concat_maps:String = map.clone().iter().collect();
             line_maps.push(Spring{map: concat_maps, list: list.clone()});

        }
        spring_maps.push(line_maps);
    }


    let mut sum1 = 0;
    let mut sum2 = 0;
    for (idx,spring_list) in spring_maps.iter_mut().enumerate() {
        let mut results:Vec<u64> = Vec::new();
        print!("{:}: {:20} ", idx, spring_list[0].map);
        let start = Instant::now();
        for spring in spring_list.iter_mut() {
            let result = process_spring(&mut spring.map, &mut spring.list);
            print!("\t{:10}", result);
            let _ = std::io::stdout().flush();
            results.push(result);
        }
        let elapsed = start.elapsed();
        print!("\t{:?}", elapsed);
        sum1 += results[0];
        sum2 += results[4];
        println!()
    }
    println!("\nq1 sum: {:}, q2 sum: {:}", sum1, sum2);
}
