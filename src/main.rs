use std::fs::read_to_string;

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();


    let mut current = (0isize, 0isize);
    let mut point_list = vec![current];
    let mut trench_length = 0;
    for line in input.lines() {
        let mut line_split = line.split_whitespace();

        let direction = line_split.next().unwrap();
        let distance:usize  =  line_split.next().unwrap().parse().unwrap();
        let _color = line_split.next();

        match direction {
            "R" => {
                current.0 += distance as isize;
            },
            "L" => {
                current.0 -= distance as isize;
            },
            "U" => {
                current.1 -= distance as isize;
            },
            "D" => {
                current.1 += distance as isize;
            },
            _=> panic!("eh?")
        }
        point_list.push(current.clone());
        trench_length += distance;
    }

    println!("trench: {:}", trench_length);

    //green's theorem
    let mut green_sum: isize = 0;
    for i in 0..point_list.len() {
        let k = point_list[i];
        let kplus = point_list[(i+1) % point_list.len()];

        green_sum += (kplus.1*k.0) as isize - (k.1*kplus.0) as isize;
    }
    //Green's theorem expects you to go counter-clockwise, or the sign flips. Since we only care about magnitude, just
    //take abs(), then it doesn't matter which direction we go.
    green_sum = (green_sum/2).abs();

    println!("greens: {:}", green_sum);
    println!("q1 full: {:}", green_sum + (trench_length as isize)/2 + 1);


    let mut current = (0i64, 0i64);
    let mut point_list = vec![current];
    let mut trench_length:i64 = 0;

    for line in input.lines() {
        let mut line_split = line.split_whitespace();
        let color = line_split.nth(2).unwrap();
        let color = &color[2..color.len()-1];

        let direction = &color[color.len()-1..];
        let distance = i64::from_str_radix(&color[..color.len()-1], 16).unwrap();

        match direction {
            "0" => {
                current.0 += distance;
            },
            "2" => {
                current.0 -= distance;
            },
            "3" => {
                current.1 -= distance;
            },
            "1" => {
                current.1 += distance;
            },
            _=> panic!("eh?")
        }
        point_list.push(current.clone());
        trench_length += distance;
    }

    //green's theorem
    let mut green_sum: i64 = 0;
    for i in 0..point_list.len() {
        let k = point_list[i];
        let kplus = point_list[(i+1) % point_list.len()];

        green_sum += (kplus.1*k.0) - (k.1*kplus.0);
    }
    //Green's theorem expects you to go counter-clockwise, or the sign flips. Since we only care about magnitude, just
    //take abs(), then it doesn't matter which direction we go.
    green_sum = (green_sum/2).abs();

    println!("greens: {:}", green_sum);
    println!("q2 full: {:}", green_sum + (trench_length)/2 + 1);


}
