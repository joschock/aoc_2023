use std::{fs::read_to_string, collections::HashMap};

fn compute_hash(acc: u8, next: char) -> u8 {
    let code = next as usize;
    let code = acc as usize + code;
    let code = code * 17;
    let code = code % 256;
    code as u8
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let steps: Vec<&str> = input.split(',').collect();

    let mut sum1:usize = 0;
    for step in steps.clone() {
        let hash = step.chars().fold(0, compute_hash);
        println!("step: {:?}, hash: {:?}", step, hash);
        sum1 += hash as usize;
    }
    println!("q1: {:?}", sum1);

    let mut box_map: HashMap<u8, Vec<(String, usize)>> = HashMap::new();
    for step in steps {
        let mut step_split = step.split_inclusive(|x|x=='-' || x=='=');
        let label_op = step_split.next().unwrap();
        let label = &label_op[..label_op.len()-1];
        let op = &label_op[label_op.len()-1..];
        let hash = label.chars().fold(0, compute_hash);
        println!("label: {:}, hash: {:}, op: {:}", label, hash, op);
        let mut lens_box = box_map.remove(&hash).unwrap_or(Vec::new());
        match op {
            "-"=> {
                lens_box.retain(|(lens_label, _)|lens_label != label);
            },
            "="=> {
                let focal_length:usize = step_split.next().unwrap().parse().unwrap();
                let mut found_lens = false;
                for lens in lens_box.iter_mut() {
                    if lens.0 == label {
                        lens.1 = focal_length;
                        found_lens = true;
                        break;
                    }
                }
                if !found_lens {
                    lens_box.push((label.to_owned(), focal_length));
                }
            },
            _ => panic!("eh?")
        }
        box_map.insert(hash, lens_box);
        //println!("box_map:{:?}", box_map);
    }
    let mut sum2 = 0;
    for (lens_box, lens_list) in box_map {
        let box_value: usize = (lens_box as usize) + 1;
        for (idx, lens) in lens_list.iter().enumerate() {
            sum2 += box_value *(idx + 1) * lens.1;
        }
    }
    println!("q2: {:}", sum2);
}
