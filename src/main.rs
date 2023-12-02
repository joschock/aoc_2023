use std::{fs::read_to_string, collections::HashMap};
use std::cmp::max;
#[derive(Debug)]
struct BagPull {
  red: usize,
  green: usize,
  blue: usize,
}

const MAX_RED:usize = 12;
const MAX_GREEN:usize = 13;
const MAX_BLUE:usize = 14;

fn main() {
  let mut games: HashMap<usize, Vec<BagPull>> = HashMap::new();
  let lines = read_to_string(".\\src\\test.txt").unwrap();
  for line in lines.lines() {
    let line: Vec<_> = line.split(":").collect();
    let id:usize = line[0].split(" ").last().unwrap().parse().unwrap();

    let mut pulls: Vec<BagPull> = Vec::new();

    for set in line[1].split(";") {
      let mut red=0;
      let mut green=0;
      let mut blue=0;
      for item in set.split(",") {
        let count: usize = item.trim().split(" ").next().unwrap().parse().unwrap();
        match item.trim().split(" ").last() {
          Some("red") => red = count,
          Some("green") => green = count,
          Some("blue") => blue = count,
          _=> panic!("eh?")
        }
      }
      pulls.push(BagPull { red, green, blue});
    }
    games.insert(id, pulls);
  }
  //println!("{:#?}", games);

  let mut sum = 0;
  'outer: for (id, pulls) in &games {
    for pull in pulls {
      if pull.red > MAX_RED || pull.green > MAX_GREEN || pull.blue > MAX_BLUE {
        continue 'outer;
      }
    }
    sum += id;
  }

  println!("q1 {:}", sum);


  let mut sum = 0;
  for (id, pulls) in &games {
    let mut red_power = 0;
    let mut blue_power = 0;
    let mut green_power = 0;
    for pull in pulls {
      red_power = max(red_power, pull.red);
      blue_power = max(blue_power, pull.blue);
      green_power = max(green_power, pull.green);
    }
    sum += red_power * blue_power * green_power;
  }

  println!("q2 {:}", sum);

}
