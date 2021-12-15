use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    cost: u32,
    x: usize,
    y: usize,
}

impl Node {
    fn new(cost: u32, x: usize, y: usize) -> Self {
        Node { cost, x, y }
    }

    fn neighbour_coords(&self) -> Vec<(i64, i64)> {
        let x = self.x as i64;
        let y = self.y as i64;
        vec![
            (x, y + 1),
            (x, y - 1),
            (x + 1, y),
            (x - 1, y),
        ]
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::hash::Hash for Node {
    fn hash<H>(&self, state: &mut H)
        where H: std::hash::Hasher,
    {
        state.write_i64((self.x * self.y) as i64);
        state.finish();
    }
}

fn parse_input(filename: &str) -> Result<Vec<Vec<u32>>, Error> {
    let reader = BufReader::new(File::open(filename)?);

    let out: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| -> Vec<u32> {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    Ok(out)
}

fn dijkstra(field: &Vec<Vec<u32>>) -> Option<u32> {

    let mut open_nodes = BinaryHeap::<Node>::from([Node::new(0, 0, 0)]);
    let mut visited = HashSet::<(usize, usize)>::new();

    let rows = field.len();
    let cols = field.first().unwrap().len();

    let end_node = Node::new(field[rows-1][cols-1], rows-1, cols-1);
    while let Some(node) = open_nodes.pop() {
        if node.x == end_node.x && node.y == end_node.y {
            // Target reached
            return Some(node.cost);
        }

        for (x, y) in node.neighbour_coords().iter() {
            if *x >= 0 && *y >= 0 && *x < rows as i64 && *y < cols as i64 {
                let new_node = Node::new(node.cost + field[*x as usize][*y as usize], *x as usize, *y as usize);
                if !visited.contains(&(new_node.x, new_node.y)) {
                    open_nodes.push(new_node.clone());
                    visited.insert((new_node.x, new_node.y));
                }
            }
        }
    }

    None
}

fn expand_field(field: &Vec<Vec<u32>>, expansion: i64) -> Vec<Vec<u32>> {
    let mut new_field = vec![
        vec![0; field[0].len() * expansion as usize]; field.len() * expansion as usize
    ];

    let mut offset = 0_usize;
    for _ in 0..(expansion * expansion) {
        let row_offset = offset / 5;
        let col_offset = offset % 5;
        let row_offset_e = row_offset * field.len();
        let col_offset_e = col_offset * field.first().unwrap().len();

        for x in 0..field.len() {
            for y in 0..field.first().unwrap().len() {
                let v = new_field.get_mut(x + row_offset_e).unwrap().get_mut(y + col_offset_e).unwrap();
                *v = field[x][y] + row_offset as u32 + col_offset as u32;
                if *v > 9 {
                    *v = *v % 9;
                    if *v == 0 {
                        *v = 1;
                    }
                }
            }
        }
        offset += 1;
    }

    new_field
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");
    let big_input = expand_field(&input, 5);

    if let Some(cost) = dijkstra(&input) {
        println!("ONE: Cost = {}", cost);
    }

    if let Some(cost) = dijkstra(&big_input) {
        println!("TWO: Cost = {}", cost);
    }
}
