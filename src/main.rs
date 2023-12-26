use std::fs::read_to_string;

struct Stone {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64)
}

impl Stone {
    fn xy_collision(&self, other: &Stone) -> Option<(f64, f64)> {
        let self_pt0 = self.pos;
        let self_pt1 = (self.pos.0 + self.vel.0, self.pos.1 + self.vel.1, self.pos.2 + self.vel.2);

        let other_pt0 = other.pos;
        let other_pt1 = (other.pos.0 + other.vel.0, other.pos.1 + other.vel.1, other.pos.2 + other.vel.2);

        let a1 = self_pt1.1 - self_pt0.1;
        let b1 = self_pt0.0 - self_pt1.0;
        let c1 = a1 * self_pt0.0 + b1 * self_pt0.1;

        let a2 = other_pt1.1 - other_pt0.1;
        let b2 = other_pt0.0 - other_pt1.0;
        let c2 = a2 * other_pt0.0 + b2 * other_pt0.1;

        let delta = a1 * b2 - a2 * b1;
        if delta == 0.0 {
            return None;
        }

        Some((
            (b2 * c1 - b1 * c2) / delta,
            (a1 * c2 - a2 * c1) / delta
        ))
    }
}

fn main() {
    let input = read_to_string(".\\src\\test.txt").unwrap();
    let mut stones = Vec::new();
    for line in input.lines() {
        let mut split = line.split(" @ ");
        let pos: Vec<isize> = split.next().unwrap().split(",").map(|x|x.trim().parse().unwrap()).collect();
        let vel: Vec<isize> = split.next().unwrap().split(",").map(|x|x.trim().parse().unwrap()).collect();
        stones.push(Stone {
            pos: (pos[0] as f64, pos[1] as f64, pos[2] as f64),
            vel: (vel[0] as f64, vel[1] as f64, vel[2] as f64)
        });
    }

    let x_range = 200000000000000f64..400000000000000f64;
    let y_range = 200000000000000f64..400000000000000f64;

    let mut count_in_range = 0;
    for stone1 in 0..stones.len() {
        for stone2 in stone1..stones.len() {
            if stone1 != stone2 {
                let intercept = stones[stone1].xy_collision(&stones[stone2]);
                //println!("{:?} - {:?} = {:?}", stone1, stone2, intercept);
                if let Some(point) = intercept {
                    let mut in_past = false;
                    if point.0 < stones[stone1].pos.0 && stones[stone1].vel.0.is_sign_positive() ||
                       point.0 > stones[stone1].pos.0 && stones[stone1].vel.0.is_sign_negative()
                    {
                        //println!("  past for A");
                        in_past = true;
                    }
                    if point.0 < stones[stone2].pos.0 && stones[stone2].vel.0.is_sign_positive() ||
                       point.0 > stones[stone2].pos.0 && stones[stone2].vel.0.is_sign_negative()
                    {
                        //println!("  past for B");
                        in_past = true;
                    }
                    if !in_past {
                        if x_range.contains(&point.0) && y_range.contains(&point.1) {
                            //println!("  in range");
                            count_in_range +=1;
                        }
                    }
                }

            }
        }
    }
    println!("q1: {:}", count_in_range);
}
