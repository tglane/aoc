use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{VecDeque, HashMap};

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
    input_idx: usize,
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
        Self { registers: regs, ops: operations, pc: 0, input_idx: 0 }
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

    fn run(&mut self, input: &VecDeque<i64>) {
        for _ in self.pc..self.ops.len() {
            self.exec(&input);
        }
    }

    fn exec_many(&mut self, input: &VecDeque<i64>, steps: usize) {
        for _ in 0..steps {
            self.exec(&input);
        }
    }

    fn exec(&mut self, input: &VecDeque<i64>) {
        let op = &self.ops[self.pc];
        let a = reg_to_idx(&op.op_a);

        match op.inst {
            Instruction::INP => {
                self.registers[a] = input[self.input_idx];
                self.input_idx += 1;
            },
            Instruction::ADD => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] + self.registers[reg_to_idx(&op_b)]
                } else {
                    self.registers[a] + op.op_b_num.unwrap()
                };
            },
            Instruction::MUL => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] * self.registers[reg_to_idx(&op_b)]
                } else {
                    self.registers[a] * op.op_b_num.unwrap()
                };
            },
            Instruction::DIV => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] / self.registers[reg_to_idx(&op_b)]
                } else {
                    self.registers[a] / op.op_b_num.unwrap()
                };
            },
            Instruction::MOD => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    self.registers[a] % self.registers[reg_to_idx(&op_b)]
                } else {
                    self.registers[a] % op.op_b_num.unwrap()
                };
            },
            Instruction::EQL => {
                self.registers[a] = if let Some(op_b) = &op.op_b {
                    (self.registers[a] == self.registers[reg_to_idx(&op_b)]) as i64
                } else {
                    (self.registers[a] == op.op_b_num.unwrap()) as i64
                };
            },
            _ => {},
        }

        self.pc += 1;
    }

    fn finished(&self) -> bool {
        self.pc == self.ops.len()
    }
}

fn reg_to_idx(reg: &Register) -> usize {
    match reg {
        Register::W => 0,
        Register::X => 1,
        Register::Y => 2,
        Register::Z => 3,
    }
}

fn num_to_vec(mut num: i64) -> VecDeque<i64> {
    let mut vec_repr = VecDeque::new();
    while num > 0 {
        vec_repr.push_front(num % 10);
        num /= 10;
    }
    vec_repr
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

fn brute_force_big(program: &Program) -> i64 {
    // Got bound through trying
    for num in (0..99300000000000_i64).rev() {
        let mut p = program.clone();
        p.run(&num_to_vec(num));
        if p.z() == 0 {
            return num;
        }
    }
    -1
}

fn brute_force_small(program: &Program) -> i64 {
    // Got bound through trying
    for num in 70000000000000..99300000000000_i64 {
        let mut p = program.clone();
        p.run(&num_to_vec(num));
        if p.z() == 0 {
            return num;
        }
    }
    -1
}

fn biggest_valid(program: &Program, visited: &mut HashMap<Program, i64>) -> i64 {
    if let Some(solution) = visited.get(&program) {
        return *solution;
    }

    let range = [9,8,7,6,5,4,3,2,1];
    'input: for input in range {
        let mut p = program.clone();

        while !p.finished() {
            if p.peek_op() == Instruction::INP {

            } else {
                continue 'input;
            }
        }

        if p.z() == 0 {
            visited.insert(p, input);
            return input;
        }
    }
    0
}

fn main() {
    let filename = std::env::args().nth(1).expect("No filename given");
    let input = parse_input(&filename).expect("Failed to parse input");

    let biggest_valid_input = brute_force_big(&input);
    println!("ONE: Biggest valid input = {}", biggest_valid_input);

    let smallest_valid_input = brute_force_small(&input);
    println!("ONE: Biggest valid input = {}", smallest_valid_input);
}
