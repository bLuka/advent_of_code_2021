use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("Hello, world!");

  let filename = "../assets/input";

  let file = File::open(filename)
    .expect("Something went wrong when opening file");
    let reader = BufReader::new(file);

    let mut result: u32 = 0;
    let mut previous_lines: [i64; 3] = [0; 3];
    let mut previous_count: i64 = 0;

    for (i, line) in reader.lines().enumerate() {

      let parsed_line: i64 = line.expect("Invalid line")
        .parse().expect("Not a number");

      if i >= 2 {
        let count: i64 = previous_lines.iter().sum();

        if i > 2 && count > previous_count {
          result += 1;
        }
        previous_count = count;
      }

      previous_lines[i % 3] = parsed_line;
  }

  println!("=> {}", result);
}
