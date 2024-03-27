use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::convert::TryInto;

#[derive(Copy, Clone)]
struct Board {
    board: [[u32; 5]; 5],
    checked: [[bool; 5]; 5],
    finished: bool,
}

impl Board {
    fn new() -> Board {
        return Board { board: [[0; 5]; 5], checked: [[false; 5]; 5], finished: false };
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..5 {
            println!("{:?}", self.board[i]);
        }
        println!("");
    }

    #[allow(dead_code)]
    fn print_checked(&self) {
        for i in 0..5 {
            println!("{:?}", self.checked[i]);
        }
        println!("");
    }

    fn unmarked_sum(&self) -> u64 {
        let mut sum = 0;
        for i in 0..5 {
            for j in 0..5 {
                if !self.checked[i][j] {
                    sum += self.board[i][j];
                }
            }
        }
        return sum as u64;
    }

    fn draw_num(&mut self, num: u32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.board[i][j] == num {
                    self.checked[i][j] = true;
                    // self.print();
                    // self.print_checked();

                    // Check on the fly if board now fulfills wincondition
                    let mut win_x = 0;
                    let mut win_y = 0;
                    for k in 0..5 {
                        if self.checked[i][k] {
                            win_y += 1;
                        }
                        if self.checked[k][j] {
                            win_x += 1;
                        }
                    }

                    if win_x == 5 || win_y == 5 {
                        self.finished = true;
                        return self.finished;
                    }
                }
            }
        }
        return false;
    }
}

fn read_input(filename: &str) -> Result<(Vec<u32>, Vec<Board>), Error> {
    let mut line_reader = BufReader::new(File::open(filename)?);

    // Extract first line with order of drawn numbers
    let mut order_str = String::new();
    line_reader.read_line(&mut order_str)?;
    let order: Vec<u32> = order_str
        .split(',').map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();
    line_reader.read_line(&mut order_str)?;

    // Read in bingo boards
    let mut boards: Vec<Board> = vec!(Board::new());
    let mut index = 0_i32;
    for line in line_reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped.is_empty() {
            index = -1;
            boards.push(Board::new());
        } else {
            let len = boards.len() - 1;
            boards[len].board[index as usize] = unwrapped
                .split(' ').map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
        }
        index += 1;
    }

    return Ok((order, boards));
}

fn one(order: &Vec<u32>, mut boards: Vec<Board>) {
    let mut score = 0;
    'outer: for num in order.iter() {
        for board in boards.iter_mut() {
            if board.draw_num(*num) {
                // Calc sum of unmarked cells
                score = board.unmarked_sum() * *num as u64;
                break 'outer;
            }
        }
    }

    println!("ONE: Score = {}", score);
}

fn two(order: &Vec<u32>, mut boards: Vec<Board>) {
    let mut store: Vec<(u64, u32)> = vec![(0, 0); boards.len()];

    let mut latest_windex = 0;
    for num in order.iter() {
        for (i, board) in boards.iter_mut().enumerate() {
            if !board.finished && board.draw_num(*num) {
                store[i] = (board.unmarked_sum() * *num as u64, i as u32);
                latest_windex = i;
            }
        }
    }

    println!("TWO: Score = {}", store[latest_windex].0);
}

fn main() {
    let (order, boards) = read_input("in.txt")
        .expect("Failed to read file");

    one(&order, boards.clone());
    two(&order, boards.clone());
}
