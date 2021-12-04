use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("Hello, world!");

  let filename = "../assets/input";

  let file = File::open(filename)
    .expect("Something went wrong when opening file");

  let reader = BufReader::new(file);
  let mut buff: Vec<Vec<i32>> = Vec::new();

  for line in reader.lines() {
    let _line = line.expect("Invalid line");
    let mut number: Vec<i32> = Vec::new();

    for (i, b) in _line.bytes().enumerate() {
      number.push(if b == b'1' { 1 } else { 0 });
    }
    buff.push(number);
  }


  let mut ox: Vec<&Vec<i32>> = buff.iter().collect();
  for (i, _) in buff[0].iter().enumerate() {
    let mut freq = 0;

    for line in ox.iter() {
      freq += if line[i] == 1 { 1 } else { -1 };
    }

    ox.retain(|x| x[i] == (if freq >= 0 { 1 } else { 0 } ));

    if ox.len() == 1 {
      break;
    }
  }

  let mut co: Vec<&Vec<i32>> = buff.iter().collect();
  for (i, _) in buff[0].iter().enumerate() {
    let mut freq = 0;

    for line in co.iter() {
      freq += if line[i] == 1 { 1 } else { -1 };
    }

    co.retain(|x| x[i] == (if freq < 0 { 1 } else { 0 } ));

    if co.len() == 1 {
      break;
    }
  }

  let oxygen = ox[0].iter().fold(0, |acc, b| (acc << 1) | b);
  let co2 = co[0].iter().fold(0, |acc, b| (acc * 2) + b);
  println!("co2 {}, oxygène {}", co2, oxygen);
  println!("=> {:?}", oxygen*co2);
}
