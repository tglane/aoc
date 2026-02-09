use crate::intcode::{IntcodeComputer, RunStatus};
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fmt::Display;

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_17/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let mut ascii = ASCIISystem::new(&input)?;
    ascii.run()?;
    let initial_frame = ascii
        .take_last_frame()
        .context("No frame rendered to completion")?;
    let alignment_parameters = initial_frame.alignment_parameter_sum();
    println!("Day 17, Part 1: Number of intersections in first frame: {alignment_parameters}");

    let routines = generate_movement_routines(initial_frame)?;

    let mut ascii = ASCIISystem::new(&input)?;
    ascii.override_movement_logic(routines.0, routines.1, false)?;
    ascii.run()?;
    let dust_collected = ascii.dust_collected().context("No dust collected")?;
    println!("Day 17, Part 2: Dust collected by the robot on its path: {dust_collected}");

    Ok(())
}

fn generate_movement_routines(frame: Frame) -> Result<(Vec<isize>, [Routine; 3])> {
    // Find shortest path to mark all
    let (mut robot_pos, mut robot_dir, scaffolds) = frame.positions();
    let mut path = Vec::<Segment>::new();
    loop {
        // Try straight path first until we are at the end of one path
        let next_straight = robot_dir.next(robot_pos);
        if scaffolds.contains(&next_straight) {
            // Path is ok, no need to turn
            path.last_mut().context("Path empty but it shouldn't")?.len += 1;
            robot_pos = next_straight;
        } else {
            // Check left and right from the direction
            let (left, left_new_dir) = robot_dir.left(robot_pos);
            let (right, right_new_dir) = robot_dir.right(robot_pos);
            if scaffolds.contains(&left) {
                robot_dir = left_new_dir;
                path.push(Segment {
                    dir: Direction::Left,
                    len: 0,
                });
            } else if scaffolds.contains(&right) {
                robot_dir = right_new_dir;
                path.push(Segment {
                    dir: Direction::Right,
                    len: 0,
                });
            } else {
                // Target reached
                break;
            }
        }
    }

    // Create three groups that cover the complete path with arbitary upper bound of 10
    let movement_routines = generate_subroutines_from_path(&path, 10)
        .context("Path could not be grouped into subroutines")?;

    Ok(movement_routines)
}

fn generate_subroutines_from_path(
    path: &[Segment],
    max_pattern_len: usize,
) -> Option<(Vec<isize>, [Routine; 3])> {
    // Generate subroutines via backtracking
    let mut patterns: [Routine; 3] = std::array::from_fn(|_| Routine::empty());

    backtrack_path(path, 0, &mut patterns, 0, max_pattern_len)
        .map(|main_routine| (main_routine, patterns))
}

fn backtrack_path(
    path: &[Segment],
    start: usize,
    patterns: &mut [Routine; 3],
    pat_idx: usize,
    max_len: usize,
) -> Option<Vec<isize>> {
    if pat_idx == 3 {
        return can_cover_path(path, patterns);
    }

    for len in 1..=max_len.min(path.len() - start) {
        let candidate = Routine {
            content: path[start..start + len].to_vec(),
        };

        if patterns[..pat_idx].contains(&candidate)
            && let Some(main_routine) =
                backtrack_path(path, start + len, patterns, pat_idx, max_len)
        {
            return Some(main_routine);
        }

        patterns[pat_idx] = candidate;

        if let Some(main_routine) =
            backtrack_path(path, start + len, patterns, pat_idx + 1, max_len)
        {
            return Some(main_routine);
        }
    }
    None
}

fn can_cover_path(path: &[Segment], patterns: &[Routine]) -> Option<Vec<isize>> {
    let n = path.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;

    let mut main_routine = Vec::new();

    for i in 0..n {
        if !dp[i] {
            continue;
        }
        for (idx, p) in patterns.iter().enumerate() {
            if i + p.len() <= n && path[i..i + p.len()] == p.content[..] {
                dp[i + p.len()] = true;
                main_routine.push(idx as isize + 65);
            }
        }
    }

    if dp[n] { Some(main_routine) } else { None }
}

struct ASCIISystem {
    computer: IntcodeComputer,
    frame_buffer: Vec<Vec<CameraField>>,
    last_frame: Option<Frame>,
    dust_collected: Option<isize>,
}

impl ASCIISystem {
    const ASCII_INPUT_SEPARATOR: isize = 44; // ,
    const ASCII_NEWLINE: isize = 10; // n

    fn new(program: &str) -> Result<Self> {
        Ok(Self {
            computer: IntcodeComputer::new(program, [])?,
            frame_buffer: vec![Vec::new()],
            last_frame: None,
            dust_collected: None,
        })
    }

    fn take_last_frame(&mut self) -> Option<Frame> {
        self.last_frame.take()
    }

    fn dust_collected(&self) -> Option<isize> {
        self.dust_collected
    }

    fn run(&mut self) -> Result<()> {
        loop {
            match self.computer.run()? {
                RunStatus::Halt => {
                    return Ok(());
                }
                RunStatus::WaitForInput => anyhow::bail!("No input command expected"),
                RunStatus::OutputValue => {
                    self.process_next_pixel()?;
                }
            }
        }
    }

    fn override_movement_logic(
        &mut self,
        main_routine: Vec<isize>,
        sub_routines: [Routine; 3],
        video_feed: bool,
    ) -> Result<()> {
        // Setting program memory at address 0 to 2 sets the system into manual movement mode.
        *self.computer.at_mut(0)? = 2;

        // Set main routine
        let main_routine_element_len = main_routine.len();
        for (idx, main_routine_element) in main_routine.into_iter().enumerate() {
            self.computer.push_input(main_routine_element);
            if idx == main_routine_element_len - 1 {
                self.computer.push_input(Self::ASCII_NEWLINE);
            } else {
                self.computer.push_input(Self::ASCII_INPUT_SEPARATOR);
            }
        }

        // Upload all routines to the robot.
        for routine in sub_routines {
            self.upload_routine(routine);
        }

        // Computer awaits input wether to activate video feed or not
        if video_feed {
            self.computer.push_input('y' as isize);
        } else {
            self.computer.push_input('n' as isize);
        }
        self.computer.push_input(Self::ASCII_NEWLINE);

        // This marks the end of the configuration procedure.
        // After this we can resume running the robot system.
        Ok(())
    }

    fn upload_routine(&mut self, routine: Routine) {
        let routine_len = routine.content.len();
        for (idx, segment) in routine.content.into_iter().enumerate() {
            match segment.dir {
                Direction::Left => {
                    self.computer.push_input('L' as isize);
                }
                Direction::Right => {
                    self.computer.push_input('R' as isize);
                }
                _ => (),
            }

            self.computer.push_input(Self::ASCII_INPUT_SEPARATOR);

            let s = segment.len.to_string();
            for c in s.as_bytes() {
                self.computer.push_input(*c as isize);
            }

            if idx == routine_len - 1 {
                self.computer.push_input(Self::ASCII_NEWLINE);
            } else {
                self.computer.push_input(Self::ASCII_INPUT_SEPARATOR);
            }
        }
    }

    fn process_next_pixel(&mut self) -> Result<()> {
        let pixel_val = self.computer.next_output().context("No output available")?;
        match pixel_val {
            Self::ASCII_NEWLINE
                if self
                    .frame_buffer
                    .last()
                    .map(|line| line.is_empty())
                    .unwrap_or(true) =>
            {
                // End of frame is indicated by double new line
                // New frame
                let mut frame = Frame(std::mem::replace(&mut self.frame_buffer, vec![Vec::new()]));
                frame.0.resize(frame.0.len() - 1, Vec::default());
                frame.render();
                self.last_frame = Some(frame);
            }
            Self::ASCII_NEWLINE => self.frame_buffer.push(Vec::new()),
            val => {
                if let Ok(pixel) = CameraField::try_from(val) {
                    self.frame_buffer
                        .last_mut()
                        .context("Frame buffer is in invalid state")?
                        .push(pixel);
                } else {
                    self.dust_collected = Some(val);
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Frame(Vec<Vec<CameraField>>);

impl Frame {
    fn alignment_parameter_sum(&self) -> i64 {
        self.intersections()
            .iter()
            .fold(0_i64, |sum, (x, y)| sum + (*x * *y))
    }

    fn render(&self) {
        for line in self.0.iter() {
            for pixel in line.iter() {
                print!("{}", pixel);
            }
            println!();
        }
    }

    fn intersections(&self) -> HashSet<(i64, i64)> {
        let mut intersections = HashSet::new();
        for y in 1..self.0.len() - 1 {
            for x in 1..self.0[y].len() - 1 {
                // Check for intersection
                if self.0[y - 1][x] == CameraField::Scaffold
                    && self.0[y + 1][x] == CameraField::Scaffold
                    && self.0[y][x - 1] == CameraField::Scaffold
                    && self.0[y][x + 1] == CameraField::Scaffold
                {
                    intersections.insert((x as i64, y as i64));
                }
            }
        }
        intersections
    }

    fn positions(&self) -> ((i64, i64), Direction, HashSet<(i64, i64)>) {
        let mut robot = (0, 0);
        let mut robot_dir = Direction::Up;
        let mut positions = HashSet::new();
        for y in 0..self.0.len() {
            for x in 0..self.0[y].len() {
                match &self.0[y][x] {
                    CameraField::Scaffold => {
                        positions.insert((x as i64, y as i64));
                    }
                    CameraField::Robot(dir) => {
                        positions.insert((x as i64, y as i64));
                        robot = (x as i64, y as i64);
                        robot_dir = *dir;
                    }
                    _ => (),
                }
            }
        }
        (robot, robot_dir, positions)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Segment {
    dir: Direction,
    len: isize,
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.dir {
            Direction::Left => write!(f, "L,{},", self.len),
            Direction::Right => write!(f, "R,{},", self.len),
            Direction::Up => write!(f, "U,{},", self.len),
            Direction::Down => write!(f, "D,{},", self.len),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Routine {
    content: Vec<Segment>,
}

impl Routine {
    fn empty() -> Self {
        Self {
            content: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.content.len()
    }

    #[allow(dead_code)]
    fn to_ascii(&self) -> Vec<isize> {
        let mut buff =
            self.content
                .iter()
                .fold(Vec::new(), |mut buff: Vec<isize>, segment: &Segment| {
                    match segment.dir {
                        Direction::Left => {
                            buff.push('L' as isize);
                        }
                        Direction::Right => {
                            buff.push('R' as isize);
                        }
                        _ => (),
                    }

                    buff.push(',' as isize);

                    let s = segment.len.to_string();
                    for c in s.as_bytes() {
                        buff.push(*c as isize);
                    }

                    buff.push(',' as isize);

                    buff
                });
        *buff.last_mut().unwrap() = 10;
        buff
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            Self::Up => (pos.0, pos.1 - 1),
            Self::Right => (pos.0 + 1, pos.1),
            Self::Down => (pos.0, pos.1 + 1),
            Self::Left => (pos.0 - 1, pos.1),
        }
    }

    fn left(&self, pos: (i64, i64)) -> ((i64, i64), Direction) {
        match self {
            Self::Up => ((pos.0 - 1, pos.1), Direction::Left),
            Self::Right => ((pos.0, pos.1 - 1), Direction::Up),
            Self::Down => ((pos.0 + 1, pos.1), Direction::Right),
            Self::Left => ((pos.0, pos.1 + 1), Direction::Down),
        }
    }

    fn right(&self, pos: (i64, i64)) -> ((i64, i64), Direction) {
        match self {
            Self::Up => ((pos.0 + 1, pos.1), Direction::Right),
            Self::Right => ((pos.0, pos.1 + 1), Direction::Down),
            Self::Down => ((pos.0 - 1, pos.1), Direction::Left),
            Self::Left => ((pos.0, pos.1 - 1), Direction::Up),
        }
    }
}

impl TryFrom<isize> for Direction {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            94 => Ok(Self::Up),    // ^
            62 => Ok(Self::Right), // >
            118 => Ok(Self::Down), // v
            60 => Ok(Self::Left),  // <
            n => anyhow::bail!("Invalid direction encoding: {n}"),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Up => "^",
            Self::Right => ">",
            Self::Down => "v",
            Self::Left => "<",
        };
        write!(f, "{val}")
    }
}

impl From<Direction> for isize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 94,
            Direction::Right => 62,
            Direction::Down => 118,
            Direction::Left => 60,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CameraField {
    Empty,
    Scaffold,
    Robot(Direction),
}

impl TryFrom<isize> for CameraField {
    type Error = anyhow::Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            46 => Ok(Self::Empty),    // .
            35 => Ok(Self::Scaffold), // #
            n => {
                Ok(Self::Robot(Direction::try_from(n).map_err(|_| {
                    anyhow::Error::msg("Invalid field encoding")
                })?))
            }
        }
    }
}

impl Display for CameraField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Scaffold => write!(f, "#"),
            Self::Robot(dir) => dir.fmt(f),
        }
    }
}
