use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_20/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;

    let maze = Maze::try_from(input.as_str())?;
    let path_len = maze.find_path().context("No path found")?;
    println!("Day 20, Part 1: Length of shortest path: {path_len}");

    let recursive_path_len = maze
        .find_path_on_recursive_maze()
        .context("No path found")?;
    println!("Day 20, Part 2: Length of shortest path in recursive maze: {recursive_path_len}");

    Ok(())
}

#[derive(Debug)]
struct Maze {
    tiles: HashMap<(i64, i64), Tile>,
}

impl Maze {
    const DIRS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    fn find_path(&self) -> Option<usize> {
        let start = self
            .tiles
            .iter()
            .find(|(_, tile)| matches!(tile, Tile::Start))
            .map(|(c, t)| (*c, *t))?;

        let mut queue = VecDeque::from([(start.0, 0)]);
        let mut seen = HashSet::new();

        while let Some((pos, steps)) = queue.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);

            match self.tiles.get(&pos) {
                None => continue,
                Some(Tile::Destination) => return Some(steps),
                Some(Tile::Start) | Some(Tile::Passage) => {
                    for dir in Self::DIRS {
                        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                        queue.push_back((next_pos, steps + 1));
                    }
                }
                Some(Tile::Portal(portal)) => {
                    for dir in Self::DIRS {
                        let port_to = portal.port_to;
                        let next_pos_after_portal = (port_to.0 + dir.0, port_to.1 + dir.1);
                        queue.push_back((next_pos_after_portal, steps + 2));
                    }
                }
            }
        }

        None
    }

    fn find_path_on_recursive_maze(&self) -> Option<usize> {
        let start = self
            .tiles
            .iter()
            .find(|(_, tile)| matches!(tile, Tile::Start))
            .map(|(c, t)| (*c, *t))?;

        let mut queue = VecDeque::from([(start.0, 0, 0)]);
        let mut seen = HashSet::<((i64, i64), usize)>::new();

        while let Some((pos, steps, depth)) = queue.pop_front() {
            if seen.contains(&(pos, depth)) {
                continue;
            }
            seen.insert((pos, depth));

            match self.tiles.get(&pos) {
                None => continue,
                Some(Tile::Destination) if depth == 0 => return Some(steps),
                Some(Tile::Start) | Some(Tile::Destination) | Some(Tile::Passage) => {
                    for dir in Self::DIRS {
                        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);
                        queue.push_back((next_pos, steps + 1, depth));
                    }
                }
                Some(Tile::Portal(portal)) => {
                    for dir in Self::DIRS {
                        let port_to = portal.port_to;
                        let next_pos_after_portal = (port_to.0 + dir.0, port_to.1 + dir.1);
                        // How to know if we get a level deeper or higher?
                        match portal.direction {
                            PortalDirection::Inner => {
                                queue.push_back((next_pos_after_portal, steps + 2, depth + 1))
                            }
                            PortalDirection::Outer if depth > 0 => {
                                queue.push_back((next_pos_after_portal, steps + 2, depth - 1))
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        None
    }
}

impl TryFrom<&str> for Maze {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut tiles = HashMap::new();

        let mut unmatched_portals = Vec::<((i64, i64), PortalIdentifier)>::new();

        let field = value
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for y in 0..field.len() {
            for x in 0..field[y].len() {
                match field[y][x] {
                    '.' => {
                        tiles.insert((x as i64, y as i64), Tile::Passage);
                    }
                    'A'..='Z' => {
                        let mut other_label_part = None;
                        let mut portal_coords = None;
                        for (x_delta, y_delta) in Self::DIRS {
                            let neighbour_coord = (x as i64 + x_delta, y as i64 + y_delta);
                            if neighbour_coord.0 < 0
                                || neighbour_coord.0 >= field[y].len() as i64
                                || neighbour_coord.1 < 0
                                || neighbour_coord.1 >= field.len() as i64
                            {
                                continue;
                            }

                            let neighbour =
                                field[neighbour_coord.1 as usize][neighbour_coord.0 as usize];
                            if neighbour == '.' {
                                portal_coords = Some(neighbour_coord);
                            } else if neighbour.is_ascii_uppercase() {
                                other_label_part = Some(neighbour);
                            }
                        }

                        if let Some(portal_coords) = portal_coords
                            && let Some(other_label_part) = other_label_part
                        {
                            unmatched_portals.push((
                                portal_coords,
                                PortalIdentifier {
                                    label: (other_label_part, field[y][x]),
                                    port_to: (0, 0), // This is just temporary
                                    direction: PortalDirection::guess_direction(
                                        portal_coords,
                                        (
                                            field[portal_coords.1 as usize].len() as i64,
                                            field.len() as i64,
                                        ),
                                    ),
                                },
                            ));
                        }
                    }
                    _ => (),
                }
            }
        }

        for i in 0..unmatched_portals.len() {
            let (coordinates, ident) = unmatched_portals[i];
            if ident == PortalIdentifier::new(('A', 'A'), (0, 0), PortalDirection::Inner) {
                tiles.insert(coordinates, Tile::Start);
            } else if ident == PortalIdentifier::new(('Z', 'Z'), (0, 0), PortalDirection::Inner) {
                tiles.insert(coordinates, Tile::Destination);
            } else {
                for j in i + 1..unmatched_portals.len() {
                    if ident == unmatched_portals[j].1 {
                        // Found match
                        unmatched_portals[j].1.port_to = coordinates;
                        unmatched_portals[i].1.port_to = unmatched_portals[j].0;

                        tiles.insert(unmatched_portals[j].0, Tile::Portal(unmatched_portals[j].1));
                        tiles.insert(coordinates, Tile::Portal(unmatched_portals[i].1));
                    }
                }
            }
        }

        Ok(Self { tiles })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Passage,
    Portal(PortalIdentifier),
    Start, // AA
    Destination, // ZZ
           // Ignore empty space and walls because we can not traverse them
}

#[derive(Copy, Clone, Debug, Eq)]
struct PortalIdentifier {
    label: (char, char),
    port_to: (i64, i64),
    direction: PortalDirection,
}

impl PortalIdentifier {
    fn new(label: (char, char), port_to: (i64, i64), direction: PortalDirection) -> Self {
        Self {
            label,
            port_to,
            direction,
        }
    }
}

impl PartialEq for PortalIdentifier {
    fn eq(&self, other: &Self) -> bool {
        if self.label.0 == other.label.0 {
            return self.label.1 == other.label.1;
        } else if self.label.1 == other.label.0 {
            return self.label.0 == other.label.1;
        }
        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum PortalDirection {
    Inner,
    Outer,
}

impl PortalDirection {
    fn guess_direction(coordinates: (i64, i64), max_coordinates: (i64, i64)) -> Self {
        if coordinates.0 == 2
            || coordinates.1 == 2
            || coordinates.0 == max_coordinates.0 - 3
            || coordinates.1 == max_coordinates.1 - 3
        {
            Self::Outer
        } else {
            Self::Inner
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_S: &str = r#"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       
"#;

    const INPUT_M: &str = r#"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     
"#;

    #[test]
    fn part_one() {
        let maze = Maze::try_from(INPUT_S).unwrap();
        let path_len = maze.find_path().unwrap();
        assert_eq!(path_len, 23);
    }

    #[test]
    fn part_two() {
        let maze = Maze::try_from(INPUT_M).unwrap();
        let path_len = maze.find_path_on_recursive_maze().unwrap();
        assert_eq!(path_len, 396);
    }
}
