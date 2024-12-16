use anyhow::{bail, Context, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    Robot,
    Wall,
    Box,
    BigBoxLeft,
    BigBoxRight,
}

impl TryFrom<char> for Tile {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '@' => Ok(Self::Robot),
            '#' => Ok(Self::Wall),
            'O' => Ok(Self::Box),
            '[' => Ok(Self::BigBoxLeft),
            ']' => Ok(Self::BigBoxRight),
            c => bail!("Invalid tile char {c}"),
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Robot => '@',
            Self::Wall => '#',
            Self::Box => 'O',
            Self::BigBoxLeft => '[',
            Self::BigBoxRight => ']',
        };
        write!(f, "{c}")?;
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct PositionUpdate {
    pos: (usize, usize),
    val: Tile,
    old_pos: (usize, usize),
    robot: bool,
}

#[derive(Clone, Debug)]
struct Warehouse {
    map: Vec<Vec<Tile>>,
    robot: (usize, usize),
}

impl Warehouse {
    fn expand(&self) -> Result<Self> {
        let mut new_map = Vec::with_capacity(self.map.len());
        let mut new_robot = self.robot;

        for (y, line) in self.map.iter().enumerate() {
            let mut new_line = Vec::with_capacity(line.len() * 2);

            for (x, tile) in line.iter().enumerate() {
                match tile {
                    Tile::Empty => {
                        new_line.push(Tile::Empty);
                        new_line.push(Tile::Empty);
                    }
                    Tile::Robot => {
                        new_robot = (x * 2, y);
                        new_line.push(Tile::Robot);
                        new_line.push(Tile::Empty);
                    }
                    Tile::Box => {
                        new_line.push(Tile::BigBoxLeft);
                        new_line.push(Tile::BigBoxRight);
                    }
                    Tile::Wall => {
                        new_line.push(Tile::Wall);
                        new_line.push(Tile::Wall);
                    }
                    _ => bail!("Can not expand big boxes any further"),
                }
            }

            new_map.push(new_line);
        }

        Ok(Self {
            map: new_map,
            robot: new_robot,
        })
    }

    fn simulate(&mut self, commands: &[(isize, isize)]) -> Result<()> {
        for command in commands {
            let mut modified = std::collections::HashSet::new();
            let pos = self.robot;
            if let Ok(updates) = self.step(&pos, &command) {
                for update in updates.into_iter().rev() {
                    if !modified.insert(update.clone()) {
                        continue;
                    }
                    self.apply_position_update(update);
                }
            }
            // self.print();
        }

        Ok(())
    }

    fn gps_sum(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .map(|(x, c)| {
                        if *c == Tile::Box || *c == Tile::BigBoxLeft {
                            100 * y + x
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn apply_position_update(&mut self, update: PositionUpdate) {
        self.map[update.pos.1][update.pos.0] = update.val;
        self.map[update.old_pos.1][update.old_pos.0] = Tile::Empty;
        if update.robot {
            self.robot = update.pos;
        }
    }

    fn step(&mut self, pos: &(usize, usize), vel: &(isize, isize)) -> Result<Vec<PositionUpdate>> {
        let mut updates = Vec::new();

        let x = TryInto::<isize>::try_into(pos.0)?;
        let y = TryInto::<isize>::try_into(pos.1)?;

        let new_x = TryInto::<usize>::try_into(x + vel.0)?;
        let new_y = TryInto::<usize>::try_into(y + vel.1)?;
        if new_x >= self.map[0].len() || new_y >= self.map.len() {
            bail!("Out of bounds: ({new_x}, {new_y})");
        }
        let new_pos = (new_x, new_y);

        let new_field = self.map[new_pos.1][new_pos.0];
        if new_field == Tile::Wall {
            // Do nothing since we can not move into wall
            // Stop update with error
            bail!("Encountered wall at ({}, {})", new_pos.0, new_pos.1);
        } else if new_field == Tile::Empty {
            // Ok to update position
            updates.push(PositionUpdate {
                pos: new_pos,
                val: self.map[pos.1][pos.0],
                old_pos: *pos,
                robot: self.map[pos.1][pos.0] == Tile::Robot,
            });
        } else if new_field == Tile::Box
            || (vel.1 == 0 && (new_field == Tile::BigBoxLeft || new_field == Tile::BigBoxRight))
        {
            // Recurse since we need to update the next field as well
            if let Ok(mut other_updates) = self.step(&new_pos, &vel) {
                updates.push(PositionUpdate {
                    pos: new_pos,
                    val: self.map[pos.1][pos.0],
                    old_pos: *pos,
                    robot: self.map[pos.1][pos.0] == Tile::Robot,
                });
                updates.append(&mut other_updates);
            } else {
                bail!("Encountered wall at ({}, {})", new_pos.0, new_pos.1);
            }
        } else if new_field == Tile::BigBoxLeft {
            // Only up and downwards need to be handled separately
            let left_update = self.step(&new_pos, &vel);
            let right_update = self.step(&(new_pos.0 + 1, new_pos.1), &vel);

            if let (Ok(mut left), Ok(mut right)) = (left_update, right_update) {
                updates.push(PositionUpdate {
                    pos: new_pos,
                    val: self.map[pos.1][pos.0],
                    old_pos: *pos,
                    robot: self.map[pos.1][pos.0] == Tile::Robot,
                });
                updates.append(&mut left);
                updates.append(&mut right);
            } else {
                bail!("");
            }
        } else if new_field == Tile::BigBoxRight {
            // Only up and downwards need to be handled separately
            let right_update = self.step(&new_pos, &vel);
            let left_update = self.step(&(new_pos.0 - 1, new_pos.1), &vel);

            if let (Ok(mut left), Ok(mut right)) = (left_update, right_update) {
                updates.push(PositionUpdate {
                    pos: new_pos,
                    val: self.map[pos.1][pos.0],
                    old_pos: *pos,
                    robot: self.map[pos.1][pos.0] == Tile::Robot,
                });
                updates.append(&mut left);
                updates.append(&mut right);
            } else {
                bail!("");
            }
        }

        Ok(updates)
    }

    #[allow(unused)]
    fn print(&self) {
        for line in self.map.iter() {
            for c in line {
                print!("{c}");
            }
            println!();
        }
    }
}

fn parse_input(input: &str) -> Result<(Warehouse, Vec<(isize, isize)>)> {
    let (map, commands) = input.split_once("\n\n").context("")?;

    let mut robot = (0, 0);
    let map = map
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '@' {
                        robot = (x, y);
                    }
                    c.try_into().unwrap()
                })
                .collect()
        })
        .collect();

    let commands = commands
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| match c {
            '^' => Ok((0, -1)),
            '>' => Ok((1, 0)),
            'v' => Ok((0, 1)),
            '<' => Ok((-1, 0)),
            c => bail!("Invalid command {c}"),
        })
        .collect::<Result<Vec<(isize, isize)>>>();

    Ok((Warehouse { map, robot }, commands?))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_15/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (mut warehouse, commands) = parse_input(&input).unwrap();
    let mut expanded_warehouse = warehouse.expand()?;

    warehouse.simulate(&commands).unwrap();
    let gps_sum = warehouse.gps_sum();
    println!("Day 14, Part 1: Total sum of GPS coordinates: {gps_sum}");

    expanded_warehouse.simulate(&commands).unwrap();
    let expanded_gps_sum = expanded_warehouse.gps_sum();
    println!(
        "Day 14, Part 2: Total sum of GPS coordinates in expanded warehouse: {expanded_gps_sum}"
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_S: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const INPUT_M: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn part_one_s() {
        let (mut warehouse, commands) = parse_input(INPUT_S).unwrap();
        warehouse.simulate(&commands).unwrap();
        assert_eq!(warehouse.gps_sum(), 2028);
    }

    #[test]
    fn part_one() {
        let (mut warehouse, commands) = parse_input(INPUT).unwrap();
        warehouse.simulate(&commands).unwrap();
        assert_eq!(warehouse.gps_sum(), 10092);
    }

    #[test]
    fn part_two_m() {
        let (mut warehouse, commands) = parse_input(INPUT_M).unwrap();
        warehouse = warehouse.expand().unwrap();
        warehouse.print();
        warehouse.simulate(&commands).unwrap();
        warehouse.print();
        assert_eq!(warehouse.gps_sum(), 618);
    }

    #[test]
    fn part_two() {
        let (mut warehouse, commands) = parse_input(INPUT).unwrap();
        warehouse = warehouse.expand().unwrap();
        warehouse.print();
        warehouse.simulate(&commands).unwrap();
        warehouse.print();
        assert_eq!(warehouse.gps_sum(), 9021);
    }
}
