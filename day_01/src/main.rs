use std::fs::File;
use std::str;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("Hello, world!");

  let filename = "../assets/input";

  let file = File::open(filename)
    .expect("Something went wrong when opening file");
    let reader = BufReader::new(file);

    let mut result: u32 = 0;
    let mut first_line = true;
    let mut previous_line: i64 = 0;

    for line in reader.lines() {
      let parsed_line: i64 =line.expect("Invalid line")
        .parse().expect("Not a number");

        if !first_line {
          if previous_line < parsed_line {
            result += 1;
          }
        }

        first_line = false;
        previous_line = parsed_line;
    }

    println!("=> {}", result);
}
