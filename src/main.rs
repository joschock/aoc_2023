use std::fs::read_to_string;
use colored::Colorize;

fn main() {
  let lines = read_to_string(".\\src\\test.txt").unwrap();

  let grid: Vec<Vec<char>> = lines.lines()
    .map(|line|{
      line.chars().collect()
    })
    .collect();

  let mut sum = 0;
  for row in 0..grid.len() {
    let mut start = None;
    let mut is_part = false;
    for col in 0..grid[0].len() {
      if grid[row][col].is_digit(10) {
        if start.is_none() {
          start = Some(col);
          is_part = false;
        }
        //check to see if it is a part
        for i in [-1isize, 0, 1]{
          for j in [-1isize, 0, 1] {
            if let Some(check_row) = row.checked_add_signed(i) {
              if let Some(check_col) = col.checked_add_signed(j) {
                if check_row < grid.len() && check_col < grid[0].len() {
                  let c = grid[check_row][check_col];
                  if !(c.is_digit(10) || c == '.') {
                    //found an adjacent symbol
                    is_part = true;
                  }
                  if check_row != row && c.is_digit(10) {
                    is_part = true;
                  }
                }
              }
            }
          }
        }
      } else {
        if let Some(first_digit) = start {
          let part_num:usize = grid[row][first_digit..col].iter().collect::<String>().parse().unwrap();
          if is_part {
            print!("{:}", part_num.to_string().green());
            sum += part_num;
          } else {
            print!("{:}", part_num.to_string().red());
          }
          start = None;
        }
        print!("{:}", grid[row][col]);
      }
      if col == grid[0].len() - 1 {
        if let Some(first_digit) = start {
          let part_num:usize = grid[row][first_digit..].iter().collect::<String>().parse().unwrap();
          if is_part {
            print!("{:}", part_num.to_string().green());
            sum += part_num;
          } else {
            print!("{:}", part_num.to_string().red());
          }
          start = None;
        }
      }
    }
    println!();
  }
  println!("q1 sum: {:}", sum);
  println!();
  println!();

  let mut sum = 0;
  for row in 0..grid.len() {
    for col in 0..grid[0].len() {
      if grid[row][col] == '*' {
        let mut adjacent_parts = Vec::new();
        let mut visited = Vec::new();
        for n_row in -1..=1 {
          for n_col in -1..=1 {
            if let Some(check_row) = row.checked_add_signed(n_row) {
              if let Some(check_col) = col.checked_add_signed(n_col) {
                if check_row < grid.len() && check_col < grid[0].len() {
                  if !visited.contains(&(check_row, check_col)) {
                    //println!("row: {:}, col: {:}, char: {:}", check_row, check_col, grid[check_row][check_col]);
                    if grid[check_row][check_col].is_digit(10) {
                      let mut start = check_col;
                      loop {
                        visited.push((check_row, start));
                        if !(start > 0 && grid[check_row][start-1].is_digit(10)) {
                          break
                        }
                        start -= 1;
                      }
                      //println!("start: {:}", start);
                      let mut end = check_col;
                      loop {
                        visited.push((check_row, end));
                        if !(end < grid[0].len() - 1 && grid[check_row][end+1].is_digit(10)) {
                          break
                        }
                        end += 1;
                      }
                      //println!("end: {:}", end);
                      let part_num: usize = grid[check_row][start..=end].iter().collect::<String>().parse().unwrap();
                      //println!("part_num: {:}", part_num);
                      //println!("visited: {:?}", visited);
                      adjacent_parts.push(part_num);
                    }
                  }
                }
              }
            }
          }
        }
        if adjacent_parts.len() == 2 {
          print!("{:}", "*".green());
          sum += adjacent_parts.iter().product::<usize>();
        } else {
          print!("{:}", "*".red());
        }
      } else {
        print!("{:}", grid[row][col]);
      }
    }
    println!();
  }
  println!("q2 sum: {:}", sum);
}