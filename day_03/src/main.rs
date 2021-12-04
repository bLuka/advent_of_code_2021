use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("Hello, world!");

  let filename = "../assets/input";

  let file = File::open(filename)
    .expect("Something went wrong when opening file");

  let reader = BufReader::new(file);
  let mut first_line = true;
  let mut occ: Vec<i32> = Vec::new();

  for line in reader.lines() {
    let _line = line.expect("Invalid line");

    if first_line {
      for b in _line.bytes() {
        occ.push(if b == b'1' { 1 } else { -1 })
      }
      first_line = false
    } else {
      for (i, b) in _line.bytes().enumerate() {
        occ[i] += if b == b'1' { 1 } else { -1 }
      }
    }
  }
  println!("occ = {:?}", occ);

  let gamma = occ.iter().map(|x| if x <= &0 { 0 } else { 1 }).fold(0, |acc, b| (acc << 1) | b);
  let epsilon = occ.iter().map(|x| if x > &0 { 0 } else { 1 }).fold(0, |acc, b| (acc << 1) | b);
  println!("=> {}", gamma);
  println!("=> {}", epsilon);
  println!("=> {}", epsilon*gamma);
}
