use anyhow::Result;
use std::collections::HashMap;

static DIRECTIONS: [fn(row: usize, col: usize) -> Option<(usize, usize)>; 4] = [
    |row, col| Some((row, col + 1)),
    |row, col| Some((row + 1, col)),
    |row, col| if col == 0 { None } else { Some((row, col - 1)) },
    |row, col| if row == 0 { None } else { Some((row - 1, col)) },
];

fn parse_input(input: &str) -> HashMap<char, Vec<Vec<(usize, usize)>>> {
    let mut areas = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            areas
                .entry(cell)
                .and_modify(|a| {
                    insert_and_merge(a, (row, col));
                })
                .or_insert(vec![vec![(row, col)]]);
        }
    }

    areas
}

fn insert_and_merge(areas: &mut Vec<Vec<(usize, usize)>>, (row, col): (usize, usize)) {
    let mut inserted_areas = Vec::new();
    for (i, area) in areas.iter().enumerate() {
        'positions: for pos in area.iter() {
            for dir in DIRECTIONS {
                if let Some((next_row, next_col)) = dir(row, col) {
                    if pos.0 == next_row && pos.1 == next_col {
                        inserted_areas.push(i);
                        break 'positions;
                    }
                }
            }
        }
    }

    if inserted_areas.len() == 0 {
        // Push new area
        areas.push(vec![(row, col)]);
    } else if inserted_areas.len() == 1 {
        let target_idx = inserted_areas.first().unwrap();
        areas[*target_idx].push((row, col));
    } else {
        // Merge areas
        let target_idx = inserted_areas.remove(0);
        areas[target_idx].push((row, col));
        for idx in inserted_areas.iter().rev() {
            let mut tmp = areas.remove(*idx);
            areas[target_idx].append(&mut tmp);
        }
    }
}

fn fencing_price(area: &Vec<(usize, usize)>) -> usize {
    let mut perimeter = 0;
    for pos in area {
        // Each position has 4 neighbours so we set the initial neighbour count to 4
        let mut perimeter_edge: usize = 4;
        for other in area {
            for dir in DIRECTIONS {
                if let Some((other_row, other_col)) = dir(pos.0, pos.1) {
                    if other_row == other.0 && other_col == other.1 {
                        if let Some(p) = perimeter_edge.checked_sub(1) {
                            perimeter_edge = p;
                        }
                    }
                }
            }
        }
        perimeter += perimeter_edge;
    }

    // Fencing price equals the perimeter multiplied by the area size
    perimeter * area.len()
}

fn fencing_price_discounted(area: &Vec<(usize, usize)>) -> usize {
    if area.len() == 1 {
        return 4;
    }

    let (top_left, bottom_right) = bounding_rectangle(area);
    if top_left.0 == bottom_right.0 || top_left.1 == bottom_right.1 {
        // Line shape
        return 4 * area.len();
    }

    let rows = (bottom_right.0 - top_left.0) + 3;
    let cols = (bottom_right.1 - top_left.1) + 3;
    let mut shape = Vec::with_capacity(rows);

    for i in 0..rows {
        let mut line_shape = Vec::with_capacity(cols);
        for j in 0..cols {
            if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
                line_shape.push(false);
            } else if is_within(area, &(top_left.0 + i - 1, top_left.1 + j - 1)) {
                line_shape.push(true);
            } else {
                line_shape.push(false);
            }
        }
        shape.push(line_shape);
    }

    let mut edges = 0;

    for i in 0..rows - 1 {
        let mut continuous = false;

        for j in 1..cols {
            if shape[i][j] != shape[i][j - 1] {
                continuous = false;
            }

            if shape[i][j] != shape[i + 1][j] {
                if !continuous {
                    edges += 1;
                }
                continuous = true;
            } else {
                continuous = false;
            }
        }
    }

    for j in 0..cols - 1 {
        let mut contiuous = false;

        for i in 1..rows {
            if shape[i][j] != shape[i - 1][j] {
                contiuous = false;
            }

            if shape[i][j] != shape[i][j + 1] {
                if !contiuous {
                    edges += 1;
                }
                contiuous = true;
            } else {
                contiuous = false;
            }
        }
    }

    edges * area.len()
}

fn bounding_rectangle(area: &Vec<(usize, usize)>) -> ((usize, usize), (usize, usize)) {
    let mut top_left = (area[0].0, area[0].1);
    let mut bottom_right = (area[0].0, area[0].1);

    for i in 1..area.len() {
        if area[i].0 < top_left.0 {
            top_left.0 = area[i].0;
        }

        if area[i].1 < top_left.1 {
            top_left.1 = area[i].1
        }

        if area[i].0 > bottom_right.0 {
            bottom_right.0 = area[i].0
        }

        if area[i].1 > bottom_right.1 {
            bottom_right.1 = area[i].1
        }
    }

    (top_left, bottom_right)
}

fn is_within(area: &Vec<(usize, usize)>, pos: &(usize, usize)) -> bool {
    for inside_pos in area.iter() {
        if inside_pos == pos {
            return true;
        }
    }
    false
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_12/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let areas = parse_input(&input);

    let fencing_price = areas
        .values()
        .flatten()
        .map(|a| fencing_price(a))
        .sum::<usize>();
    println!("Day 12, Part 1: Fencing price: {fencing_price}",);

    let fencing_price_discounted = areas
        .values()
        .flatten()
        .map(|a| fencing_price_discounted(a))
        .sum::<usize>();
    println!("Day 12, Part 2: Fencing price discounted: {fencing_price_discounted}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part_one() {
        let areas = parse_input(INPUT);
        let fencing_price = areas
            .values()
            .flatten()
            .map(|a| fencing_price(a))
            .sum::<usize>();
        assert_eq!(fencing_price, 1930);
    }

    #[test]
    fn part_two() {
        let areas = parse_input(INPUT);
        let fencing_price_discounted = areas
            .values()
            .flatten()
            .map(|a| fencing_price_discounted(a))
            .sum::<usize>();
        assert_eq!(fencing_price_discounted, 1206);
    }
}
