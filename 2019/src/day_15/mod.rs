use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_15/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let robot = RepairDroid::new(&input)?;

    let min_steps = steps_to_oxygen_system(robot.clone())?;
    println!("Day 15, Part 1: Minimum steps to oxygen system: {min_steps}");

    let oxygen_fill_steps = determine_oxygen_fill_time(robot)?;
    println!("Day 15, Part 2: Steps to fill area with oxygen: {oxygen_fill_steps}");

    Ok(())
}

fn steps_to_oxygen_system(robot: RepairDroid) -> Result<usize> {
    let mut queue = VecDeque::from([(robot, 0_usize)]); // List of positions to check next
    let mut cache = HashSet::<(i64, i64)>::new();

    while let Some((robot, steps)) = queue.pop_front() {
        // Test all 4 moves, if valid, append new positions to the  queue

        if cache.contains(&robot.pos) {
            continue;
        }
        cache.insert(robot.pos);

        for next_move in [
            Movement::North,
            Movement::East,
            Movement::South,
            Movement::West,
        ] {
            let mut robot = robot.clone();

            match robot.movement(next_move)? {
                MovementStatus::HitWall => (),
                MovementStatus::MoveSuccessful => {
                    queue.push_back((robot, steps + 1));
                }
                MovementStatus::MovedToOxygenSystem => return Ok(steps + 1),
            }
        }
    }

    anyhow::bail!("No path to oxygen system found")
}

fn explore_area(robot: RepairDroid) -> Result<HashMap<(i64, i64), MovementStatus>> {
    let mut queue = VecDeque::from([(robot, 0_usize)]); // List of positions to check next
    let mut cache = HashSet::<(i64, i64)>::new();
    let mut map = HashMap::new();

    while let Some((robot, steps)) = queue.pop_front() {
        // Test all 4 moves, if valid, append new positions to the  queue

        if cache.contains(&robot.pos) {
            continue;
        }
        cache.insert(robot.pos);

        for next_move in [
            Movement::North,
            Movement::East,
            Movement::South,
            Movement::West,
        ] {
            let mut robot = robot.clone();
            let move_status = robot.movement(next_move)?;
            if !matches!(move_status, MovementStatus::HitWall) {
                map.insert(robot.pos, move_status);
                queue.push_back((robot, steps + 1));
            }
        }
    }

    Ok(map)
}

fn determine_oxygen_fill_time(robot: RepairDroid) -> Result<usize> {
    let map = explore_area(robot)?;

    let oxygen_system_pos = *map
        .iter()
        .find(|(_pos, field_val)| matches!(field_val, MovementStatus::MovedToOxygenSystem))
        .context("No oxygen system found")?
        .0;

    let mut queue = VecDeque::from([vec![oxygen_system_pos]]);
    let mut seen = HashSet::<(i64, i64)>::new();

    let mut steps = 0;
    while let Some(positions) = queue.pop_front() {
        steps += 1;

        let mut next_positions = Vec::new();

        for position in positions {
            // Check neighbouring

            for next_move in [
                Movement::North,
                Movement::East,
                Movement::South,
                Movement::West,
            ] {
                let next_pos = next_move.apply(position);
                if let Some(next_pos_in_map) = map.get(&next_pos)
                    && !seen.contains(&next_pos)
                    && !matches!(next_pos_in_map, MovementStatus::HitWall)
                {
                    seen.insert(next_pos);
                    next_positions.push(next_pos);
                }
            }
        }

        if !next_positions.is_empty() {
            queue.push_back(next_positions);
        }
    }

    Ok(steps - 1)
}

#[derive(Copy, Clone)]
enum Movement {
    North,
    East,
    South,
    West,
}

impl Movement {
    fn apply(&self, mut pos: (i64, i64)) -> (i64, i64) {
        match self {
            Movement::North => pos.1 += 1,
            Movement::East => pos.0 += 1,
            Movement::South => pos.1 -= 1,
            Movement::West => pos.0 -= 1,
        }
        pos
    }
}

impl From<Movement> for isize {
    fn from(value: Movement) -> Self {
        match value {
            Movement::North => 1,
            Movement::East => 4,
            Movement::South => 2,
            Movement::West => 3,
        }
    }
}

impl TryFrom<isize> for Movement {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Movement::North),
            2 => Ok(Movement::South),
            3 => Ok(Movement::West),
            4 => Ok(Movement::East),
            v => anyhow::bail!("Invalid movement encoding: {v}"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum MovementStatus {
    HitWall,
    MoveSuccessful,
    MovedToOxygenSystem,
}

impl TryFrom<isize> for MovementStatus {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MovementStatus::HitWall),
            1 => Ok(MovementStatus::MoveSuccessful),
            2 => Ok(MovementStatus::MovedToOxygenSystem),
            v => anyhow::bail!("Invalid movement status encoding: {v}"),
        }
    }
}

#[derive(Clone)]
struct RepairDroid {
    computer: IntcodeComputer,
    pos: (i64, i64),
    tiles_visited: HashMap<(i64, i64), MovementStatus>,
}

impl RepairDroid {
    fn new(program: &str) -> Result<Self> {
        Ok(Self {
            computer: IntcodeComputer::new(program, [])?,
            pos: (0, 0),
            tiles_visited: HashMap::from([((0, 0), MovementStatus::MoveSuccessful)]),
        })
    }

    fn movement(&mut self, move_instr: Movement) -> Result<MovementStatus> {
        self.computer.push_input(move_instr.into());
        match self.computer.run()? {
            RunStatus::Halt => anyhow::bail!("Intcode computer halted unexpectedly"),
            RunStatus::WaitForInput => anyhow::bail!("Intcode computer is expecting input"),
            RunStatus::OutputValue => {
                let status = MovementStatus::try_from(
                    self.computer
                        .next_output()
                        .context("Intcode computer did not provide output")?,
                )?;

                let next_pos = move_instr.apply(self.pos);
                self.tiles_visited.insert(next_pos, status);
                if matches!(status, MovementStatus::MoveSuccessful)
                    || matches!(status, MovementStatus::MovedToOxygenSystem)
                {
                    self.pos = next_pos;
                }

                Ok(status)
            }
        }
    }
}
