use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq)]
enum Instruction {
    INP,
    ADD,
    MUL,
    DIV,
    MOD,
    EQL,
    NoOp,
}

#[derive(Clone, Eq, PartialEq)]
enum Register {
    W,
    X,
    Y,
    Z,
}

impl Register {
    fn to_index(&self) -> usize {
        match self {
            Register::W => 0,
            Register::X => 1,
            Register::Y => 2,
            Register::Z => 3,
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Op {
    inst: Instruction,
    op_a: Register,
    op_b: Option<Register>,
    op_b_num: Option<i64>,
}

impl Op {
    fn from(op_str: &str) -> Self {
        let mut split = op_str.split(' ');

        // Parse instruction
        let inst = match split.next().unwrap() {
            "inp" => Instruction::INP,
            "add" => Instruction::ADD,
            "mul" => Instruction::MUL,
            "div" => Instruction::DIV,
            "mod" => Instruction::MOD,
            "eql" => Instruction::EQL,
            _ => Instruction::NoOp,
        };

        // Parse first operand
        let op_a = match split.next().unwrap() {
            "w" => Register::W,
            "x" => Register::X,
            "y" => Register::Y,
            "z" => Register::Z,
            _ => Register::W, // Should not appear
        };

        // Parse second operand (optional)
        let mut op_b = None;
        let mut op_b_num = None;
        if let Some(sec) = split.next() {
            match sec {
                "w" => {
                    op_b = Some(Register::W);
                },
                "x" => {
                    op_b = Some(Register::X);
                },
                "y" => {
                    op_b = Some(Register::Y);
                },
                "z" => {
                    op_b = Some(Register::Z);
                },
                _ => {
                    // Parse num
                    op_b_num = Some(sec.parse::<i64>().unwrap());
                },
            };
        }

        Op { inst, op_a, op_b, op_b_num }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Program {
    registers: [i64; 4],
    ops: Vec<Op>,
    pc: usize,
}

impl std::hash::Hash for Program {
    fn hash<H>(&self, state: &mut H)
        where H: std::hash::Hasher
    {
        self.registers.hash(state);
        state.finish();
    }
}

#[allow(dead_code)]
impl Program {
    fn from(regs: [i64; 4], operations: Vec<Op>) -> Self {
        Self { registers: regs, ops: operations, pc: 0 }
    }

    fn w(&self) -> i64 {
        self.registers[0]
    }

    fn x(&self) -> i64 {
        self.registers[1]
    }

    fn y(&self) -> i64 {
        self.registers[2]
    }

    fn z(&self) -> i64 {
        self.registers[3]
    }

    fn peek_op(&self) -> Instruction {
        self.ops[self.pc].inst.clone()
    }

    fn run(&mut self, input: &[i64]) {
        let mut input_idx = 0_usize;
        for _ in self.pc..self.ops.len() {
            if self.peek_op() == Instruction::INP {
                self.exec(Some(input[input_idx]));
                input_idx += 1;
            } else {
                self.exec(None);
            }
        }
    }

    fn exec_many(&mut self, input: &[i64], steps: usize) {
        let mut input_idx = 0_usize;
        for _ in 0..steps {
            if self.peek_op() == Instruction::INP {
                self.exec(Some(input[input_idx]));
                input_idx += 1;
            } else {
                self.exec(None);
            }
        }
    }

    fn exec(&mut self, input: Option<i64>) {
        let op = &self.ops[self.pc];
        let a = op.op_a.to_index();

        match op.inst {
            Instruction::INP => {
                if let Some(in_num) = input {
                self.registers[a] = in_num;
                } else {
                    panic!("Missing input!");
                }
            },
            Instruction::ADD => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] + self.registers[op_b.to_index()]
                } else {
                    self.registers[a] + op.op_b_num.unwrap()
                };
            },
            Instruction::MUL => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] * self.registers[op_b.to_index()]
                } else {
                    self.registers[a] * op.op_b_num.unwrap()
                };
            },
            Instruction::DIV => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] / self.registers[op_b.to_index()]
                } else {
                    self.registers[a] / op.op_b_num.unwrap()
                };
            },
            Instruction::MOD => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] % self.registers[op_b.to_index()]
                } else {
                    self.registers[a] % op.op_b_num.unwrap()
                };
            },
            Instruction::EQL => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    (self.registers[a] == self.registers[op_b.to_index()]) as i64
                } else {
                    (self.registers[a] == op.op_b_num.unwrap()) as i64
                };
            },
            _ => {
                panic!("Invalid instruction");
            },
        }

        self.pc += 1;
    }

    fn finished(&self) -> bool {
        self.pc == self.ops.len()
    }
}

fn parse_input(filename: &str) -> Result<Program, std::io::Error> {
    let reader = BufReader::new(File::open(filename)?);

    let ops = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| Op::from(&line))
        .collect::<Vec<Op>>();

    Ok(Program::from([0; 4], ops))
}

fn get_valid_internal(program: &Program, visited: &mut HashMap<Program, Option<i64>>, number_range: &[i64]) -> Option<i64> {
    if let Some(solution) = visited.get(&program) {
        return *solution;
    }

    'input: for input in number_range {
        let mut p = program.clone();
        p.exec(Some(*input));

        while !p.finished() {
            if p.peek_op() == Instruction::INP {
                if let Some(old_smallest) = get_valid_internal(&p, visited, number_range) {
                    visited.insert(p.clone(), Some(old_smallest * 10 + *input));
                    return Some(old_smallest * 10 + *input);
                } else {
                    continue 'input;
                }
            } else {
                p.exec(None);
            }
        }

        if p.z() == 0 {
            visited.insert(p, Some(*input));
            return Some(*input);
        }
    }

    visited.insert(program.clone(), None);
    None
}

fn biggest_valid(program: &Program) -> i64 {
    let number_range = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut tmp = get_valid_internal(&program, &mut HashMap::new(), &number_range).unwrap();
    let mut reversed = 0;
    while tmp != 0 {
        reversed = reversed * 10 + tmp % 10;
        tmp /= 10;
    }
    reversed
}

fn smallest_valid(program: &Program) -> i64 {
    let number_range = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut tmp = get_valid_internal(&program, &mut HashMap::new(), &number_range).unwrap();
    let mut reversed = 0;
    while tmp != 0 {
        reversed = reversed * 10 + tmp % 10;
        tmp /= 10;
    }
    reversed
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let input = parse_input(&filename).expect("Failed to parse input");

    let biggest_valid_input = biggest_valid(&input);
    println!("ONE: Biggest valid input = {}", biggest_valid_input);

    let smallest_valid_input = smallest_valid(&input);
    println!("TWO: Smallest valid input = {}", smallest_valid_input);
}
