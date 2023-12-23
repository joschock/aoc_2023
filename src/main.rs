use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}


#[derive(Debug, Clone, Eq)]
struct Brick {
    id: usize,
    a: Point,
    b: Point
}

impl PartialEq for Brick {
    fn eq(&self, other: &Self) -> bool {
        (self.a, self.b).eq(&(other.a, other.b))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let c = min(self.a.z, self.b.z).cmp(&min(other.a.z, other.b.z));
        if !c.is_eq() {
            return c;
        }
        let c = min(self.a.x, self.b.x).cmp(&min(other.a.x, other.b.x));
        if !c.is_eq() {
            return c;
        }
        let c = min(self.a.y, self.b.y).cmp(&min(other.a.y, other.b.y));
        if !c.is_eq() {
            return c;
        }
        let c = self.a.cmp(&other.a);
        if !c.is_eq() {
            return c;
        }
        self.b.cmp(&other.b)
    }
}

impl Brick {
    fn lowest_point(&self) -> &Point {
        if self.a.z <= self.b.z {
            &self.a
        } else {
            &self.b
        }
     }

     fn intersects(&self, other: &Brick) -> bool {
        //print!("intersect {:?} and {:?}: ", self, other);
        let other_x =  min(other.a.x, other.b.x)..=max(other.a.x, other.b.x);
        let other_y =  min(other.a.y, other.b.y)..=max(other.a.y, other.b.y);
        let other_z =  min(other.a.z, other.b.z)..=max(other.a.z, other.b.z);


        for x in min(self.a.x, self.b.x)..=max(self.a.x, self.b.x) {
            for y in min(self.a.y, self.b.y)..=max(self.a.y, self.b.y) {
                for z in min(self.a.z, self.b.z)..=max(self.a.z, self.b.z) {
                    if other_x.contains(&x) &&
                       other_y.contains(&y) &&
                       other_z.contains(&z) {
                        //println!("true");
                        return true;
                    }
                }
            }
        }
        //println!("false");
        return false;
     }

     fn lower(&self) -> Brick {
        let mut new_a = self.a;
        let mut new_b = self.b;
        new_a.z -= 1;
        new_b.z -= 1;
        Brick{id: self.id, a:new_a, b: new_b}
     }

     fn set(&mut self, brick: &Brick) {
        self.a = brick.a;
        self.b = brick.b;
     }
}

fn settle(bricks: &mut BTreeSet<Brick>) -> usize {

    let mut brick_list: Vec<Brick> = Vec::new();
    while let Some(brick) = bricks.pop_first() {
        brick_list.push(brick);
    }

    //println!("bricks: {:#?}", brick_list);
    let mut move_count = 0;
    for idx in 0..brick_list.len() {
        let mut moved = false;
        'lower: loop {
            if brick_list[idx].lowest_point().z == 1 {
                break;
            }
            let candidate = brick_list[idx].lower();
            for lower_brick in (0..idx).rev() {
                if brick_list[lower_brick].intersects(&candidate) {
                    break 'lower;
                }
            }
            moved = true;
            brick_list[idx].set(&candidate);
        }
        if moved {
            move_count+=1;
        }
    }

    //println!("bricks: {:#?}", brick_list);
    bricks.extend(brick_list);
    move_count
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();

    let mut bricks = BTreeSet::new();

    for (idx, line) in input.lines().enumerate() {
        let tokens:Vec<usize> = line.split(['~',',']).map(|x|x.parse().unwrap()).collect();
        bricks.insert(Brick {
            id: idx,
            a: Point {x: tokens[0], y: tokens[1], z: tokens[2]},
            b: Point {x: tokens[3], y: tokens[4], z: tokens[5]}
        });
    }
    //for brick in &bricks {
    //    println!("{:?}", brick);
    //}

    settle(&mut bricks);
    //println!("-----");
    //for brick in &bricks {
    //    println!("{:?}", brick);
    //}
    //println!("-----");


    let candidates: Vec<Brick> = bricks.iter().cloned().collect();

    let mut sum1 = 0;
    let mut sum2 = 0;
    for candidate in candidates {
        bricks.remove(&candidate);

        print!("candidate: {:?}", candidate);

        let mut after = bricks.clone();
        let moved = settle(&mut after);

        if moved > 0 {
            println!("-disintegrate");
            sum1 += 1;
            sum2 += moved;
        } else {
            println!("-no disintegrate");
        }
        bricks.insert(candidate);
    }
    //for brick in bricks {
    //    println!("{:?}", brick);
    //}
    println!("q1: {:}", sum1);
    println!("q2: {:}", sum2);
}
