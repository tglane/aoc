use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct Mark {
    x: u64,
    y: u64,
}

impl Mark {
    fn with(set_x: u64, set_y: u64) -> Mark {
        Mark { x: set_x, y: set_y }
    }
}

struct Fold {
    axis: char,
    height: u64,
}

impl Fold {
    fn with(set_axis: char, set_height: u64) -> Fold {
        Fold { axis: set_axis, height: set_height }
    }
}

struct Board {
    field: Vec<Vec<char>>,
    x_len: usize,
    y_len: usize,
}

impl Board {
    fn from(marks: &Vec<Mark>) -> Board {
        let mut x_max = 0_usize;
        let mut y_max = 0_usize;
        for mark in marks.iter() {
            if mark.x > x_max as u64 {
                x_max = mark.x as usize;
            }
            if mark.y > y_max as u64 {
                y_max = mark.y as usize;
            }
        }

        let mut board = Board { field: vec![vec!['.'; y_max + 1]; x_max + 1], x_len: x_max + 1, y_len: y_max + 1 };
        for mark in marks.iter() {
            board.field[mark.x as usize][mark.y as usize] = '#';
        }

        board
    }

    fn fold(&mut self, axis: char, height: u64) {
        if axis == 'x' {
            for x in height..self.x_len as u64 {
                for y in 0..self.y_len as u64 {
                    if self.field[x as usize][y as usize] == '#' {
                        let dist = x - height;
                        self.field[(height - dist) as usize][y as usize] = '#';
                    }
                }
            }
            self.x_len = height as usize;
        } else if axis == 'y' {
            for x in 0..self.x_len as u64 {
                for y in height..self.y_len as u64 {
                    if self.field[x as usize][y as usize] == '#' {
                        let dist = y - height;
                        self.field[x as usize][(height - dist) as usize] = '#';
                    }
                }
            }
            self.y_len = height as usize;
        }
    }

    fn marked(&self) -> u64 {
        let mut counter = 0_u64;
        for x in 0..self.x_len as u64 {
            for y in 0..self.y_len as u64 {
                if self.field[x as usize][y as usize] == '#' {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn print(&self) {
        for x in 0..self.x_len {
            for y in 0..self.y_len {
                print!("{}", self.field[x as usize][y as usize]);
            }
            println!("");
        }
        println!("---------------");
    }
}

fn parse_input(filename: &str) -> Result<(Vec<Mark>, Vec<Fold>), Error> {
    let reader = BufReader::new(File::open(filename)?);

    let mut marks = Vec::<Mark>::new();
    let mut folds = Vec::<Fold>::new();

    let mut empty_line_reached = false;
    for line in reader.lines() {
        let unwrapped = line.unwrap();
        if unwrapped == "" {
            empty_line_reached = true;
        } else if !empty_line_reached {
            // Coordinates of marks
            let mut splitted = unwrapped.split(',');
            marks.push(Mark::with(splitted.next().unwrap().parse::<u64>().unwrap(), splitted.next().unwrap().parse::<u64>().unwrap()));
        } else {
            // Fold expressions
            let splitted = unwrapped.split(' ');
            let imp_part = splitted.last().unwrap();
            let mut splitted_imp = imp_part.split('=');
            folds.push(Fold::with(splitted_imp.next().unwrap().parse::<char>().unwrap(), splitted_imp.next().unwrap().parse::<u64>().unwrap()));
        }
    }

    Ok((marks, folds))
}

fn main() {
    let (marks, folds) = parse_input("in.txt")
        .expect("Failed to parse input");

    let mut field = Board::from(&marks);

    for (i, fold) in folds.iter().enumerate() {
        field.fold(fold.axis, fold.height);
        println!("Marked after {} folds: {}", i + 1, field.marked());
    }
    field.print();
}
