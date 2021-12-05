use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  let filename = "../assets/input";
  let file = File::open(filename).expect("Something went wrong when opening file");
  let reader = BufReader::new(file);

  let mut drawn: Vec<i32> = Vec::new();
  let mut boards: Vec<[[i32; 5]; 5]> = Vec::new();
  let mut board_buffer: [[i32; 5]; 5] = [[0; 5]; 5];

  for (i, line) in reader.lines().enumerate() {
    let _line = line.expect("Invalid line");

    if i == 0 {
      drawn = _line.split(',').map(|s| s.parse().expect("Cannot parse")).collect();
      continue
    }
    if i == 1 {
      continue
    }

    let board_line = (i as i32 - 2) % 6;

    if board_line == 5 {
      // Flush the buffer
      boards.push(board_buffer);
      continue
    }

    for (i, n) in _line.split_ascii_whitespace().map(|s| s.parse().expect("Cannot parse")).enumerate() {
      board_buffer[board_line as usize][i] = n;
    }
  }

  let ro_boards = boards.clone();

  'find_winner: for num in drawn {
    for (b, board) in ro_boards.iter().enumerate() {
      for (i, line) in board.iter().enumerate() {
        for (j, n) in line.iter().enumerate() {
          if n == &num {
            boards[b][i][j] = -n;
          }
        }
      }

      if boards[b].iter().any(|&line| line.iter().all(|&x| x < 0)) ||
        (0..4).any(|i| boards[b].iter().all(|&line| line[i] < 0)) {
        let score = boards[b].iter().fold(0, |acc, x| acc + x.iter().fold(0, |acc, x| acc + if x > &0 { x } else { &0 }));

        println!("Winner! Score is {}", score*num);

        break 'find_winner;
      }
    }
  }
}
