use std::fs::File;
use std::str;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("Hello, world!");

  let filename = "../assets/input";

  let file = File::open(filename)
    .expect("Something went wrong when opening file");
    let reader = BufReader::new(file);

    let  (mut x, mut y): (i64, i64) = (0, 0);

    for line in reader.lines() {
      let _line = line.expect("Invalid line");
      let mut iter = _line.split_whitespace();

      let operation = iter.next().expect("Invalid line");
      let value: i64 = iter.next().expect("Invalid line").parse().expect("Not a number");

      match operation {
        "forward" => x += value,
        "down" => y += value,
        "up" => y -= value,
        _ => println!("Invalid line: {}", _line),
      }
    }

    println!("=> {}×{} = {}", x, y, x*y);
}
