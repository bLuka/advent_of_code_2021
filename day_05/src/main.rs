use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::fmt;
use std::collections::HashMap;

struct Point {
  x: i32,
  y: i32,
}
struct Line {
  a: Point,
  b: Point,
}

fn main() {
  let filename = "../assets/input";
  let file = File::open(filename).expect("Something went wrong when opening file");
  let reader = BufReader::new(file);

  let mut lines: Vec<Line> = Vec::new();

  for entry in reader.lines() {
    let line = Line::new(&entry.expect("Invalid line"));

    lines.push(line);
  }

  let mut board = HashMap::new();

  for line in lines {
    for point in line.list_points().iter() {
      let n = *board.get(&point.hash()).unwrap_or(&0);

      board.insert(point.hash(), n + 1);
    }
  }

  let mut res = 0;
  for (_, value) in board.into_iter() {
    if value > 1 {
      res += 1;
    }
  }

  println!("Res => {}", res);
}

impl Point {
  fn new(coords_str: &str) -> Self {
    let coords: Vec<i32> = coords_str.split(",").map(|s| s.parse().expect("Not a number")).collect();

    Point {
      x: coords[0],
      y: coords[1]
    }
  }

  fn hash(&self) -> String {
    format!("{} {}", self.x, self.y)
  }
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Line")
      .field("x", &self.x)
      .field("y", &self.y)
      .finish()
  }
}

impl Line {
  fn new(coords: &String) -> Self {
    let points: Vec<&str> = coords.split(" -> ").collect();

    Line {
      a: Point::new(points[0]),
      b: Point::new(points[1]),
    }
  }

  fn list_points(&self) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();

    if &self.a.x == &self.b.x {
      let mut n: i32;
      let max: i32;

      if &self.a.y < &self.b.y {
        n = self.a.y;
        max = self.b.y;
      } else {
        n = self.b.y;
        max = self.a.y;
      }

      while n <= max {
        points.push(Point { x: self.a.x, y: n });
        n += 1;
      }
    } else if &self.a.y == &self.b.y {
      let mut n: i32;
      let max: i32;

      if &self.a.x < &self.b.x {
        n = self.a.x;
        max = self.b.x;
      } else {
        n = self.b.x;
        max = self.a.x;
      }

      while n <= max {
        points.push(Point { x: n, y: self.a.y });
        n += 1;
      }
    }

    points
  }
}

impl fmt::Debug for Line {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Line")
      .field("a", &self.a)
      .field("b", &self.b)
      .finish()
  }
}
