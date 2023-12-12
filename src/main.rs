use std::fs::read_to_string;

#[derive(Debug)]
struct Spring {
    map: String,
    list: Vec<usize>
}

fn process_spring(remaining_map: &mut str, remaining_list: &[usize], result: &mut u64) {
    if remaining_list.iter().map(|x|x+1).sum::<usize>()-1 > remaining_map.len() {
        return;
    }
    //println!("\nremaining_map: {:?}\nremaining_list: {:?}", remaining_map, remaining_list);
    let mut consumed_chars = 0;
    let mut consumed_list = 0;
    for element in remaining_map.split(|x|x == '.') {
        if element.len() == 0 {
            consumed_chars += 1;
            continue;
        }
        if !element.contains("?") {
            if consumed_list == remaining_list.len() {
                //println!("  base: bad map - not enough springs");
                return; //base case: bad map - ran out of springs, but still have places to put them.
            }
            if element.len() != remaining_list[consumed_list] {
                //println!("  base: bad map - next spring don't match");
                return; //base case: bad map - next broken spring size doesn't match next list size.
            }
            consumed_chars += element.len() + 1;
            consumed_list += 1;
        } else {
            if consumed_list < remaining_list.len() {
                if element.find("?").unwrap() > remaining_list[consumed_list] {
                    return; // too many broken springs on the front.
                }
            }
            break;
        }
    }

    //println!("  consumed_chars: {:}", consumed_chars);
    //println!("  consumed_list: {:}", consumed_list);

    if consumed_list == remaining_list.len() {
        //base case: good map - exactly consumed the remaining springs with no ? left.
        *result +=1;
        return;
    }

    //recursive case
    if consumed_chars >= remaining_map.len() {
        //println!("\nremaining_map: {:?}\nremaining_list: {:?}", remaining_map, remaining_list);
        return;
    }

    let next_list = &remaining_list[consumed_list..];
    let next_map = &mut remaining_map[consumed_chars..];
    if let Some(qidx) = next_map.find('?') {
        //assumes ascii
        unsafe {next_map.as_bytes_mut()[qidx] = '.' as u8};
        process_spring(next_map, next_list, result);
        unsafe {next_map.as_bytes_mut()[qidx] = '#' as u8};
        process_spring(next_map, next_list, result);
        unsafe {next_map.as_bytes_mut()[qidx] = '?' as u8};
    }

}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let mut part_1_springs: Vec<Spring> = Vec::new();
    let mut part_2_springs: Vec<Spring> = Vec::new();
    for line in input.lines() {
        let mut line_split = line.split_whitespace();
        let mut map: Vec<char> = line_split.next().unwrap().chars().collect();
        let mut list: Vec<usize> = line_split.next().unwrap().split(",").map(|x|x.parse().unwrap()).collect();

        let map1: String = map.clone().iter().collect();

        part_1_springs.push(Spring{map: map1.trim_end_matches('.').to_owned(), list: list.clone()});

        let map_len = map.len();
        let list_len = list.len();
        for _x in 0..4 {
             map.push('?');
             map.extend_from_within(..map_len);
             list.extend_from_within(..list_len);
        }

        let map2: String = map.iter().collect();
        part_2_springs.push(Spring { map: map2.trim_end_matches('.').to_owned(), list});
    }

    let mut sum = 0;
    for spring in part_1_springs.iter_mut() {
        //println!("checking spring: {:?}", spring);
        let mut result = 0;
        process_spring(&mut spring.map, &spring.list, &mut result);
        //println!("result: {:}", result);
        sum += result;
    }

    println!("q1: {:}", sum);

    let mut sum = 0;

    let spring_count = part_2_springs.len();
    for (idx,spring) in part_2_springs.iter_mut().enumerate() {
        println!("checking spring ({:}/{:}): {:?}", idx, spring_count, spring);
        let mut result = 0;
        process_spring(&mut spring.map, &spring.list, &mut result);
        println!("result: {:}", result);
        sum += result;
    }
    println!("q2: {:}", sum);
}
