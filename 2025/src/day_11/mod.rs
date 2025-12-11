use crate::Day;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::path::Path;

pub(crate) struct DayEleven {
    input: String,
}

impl Day for DayEleven {
    fn new<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            input: std::fs::read_to_string(path)?,
        })
    }

    fn part_one(&self) -> Result<()> {
        let connections = parse_input(&self.input)?;
        let paths = count_paths(&connections, "you".into(), "out".into());
        println!("Day 11 - Part 1: Number of paths: {paths}");
        Ok(())
    }

    fn part_two(&self) -> Result<()> {
        let connections = parse_input(&self.input)?;
        let paths = count_paths_that_include(
            &connections,
            "svr".into(),
            "out".into(),
            HashSet::from(["fft".into(), "dac".into()]),
        );
        println!("Day 11 - Part 2: Number of paths that include 'fft' and 'dac': {paths}");
        Ok(())
    }
}

fn parse_input(input: &str) -> Result<HashMap<String, Vec<String>>> {
    let connections = input
        .lines()
        .map(|line| {
            let (from, to_list) = line.split_once(": ").context("")?;
            let to = to_list
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<_>>();
            Ok((from.to_string(), to))
        })
        .collect::<Result<HashMap<String, Vec<String>>>>()?;

    Ok(connections)
}

fn count_paths(connections: &HashMap<String, Vec<String>>, start: String, target: String) -> usize {
    let mut queue = VecDeque::from([start]);
    let mut paths_to_target = 0;
    while let Some(next) = queue.pop_front() {
        if next == target {
            paths_to_target += 1;
            continue;
        }
        if let Some(to) = connections.get(&next) {
            for node in to {
                queue.push_back(node.clone());
            }
        }
    }
    paths_to_target
}

fn count_paths_that_include(
    connections: &HashMap<String, Vec<String>>,
    start: String,
    target: String,
    must_include: HashSet<String>,
) -> usize {
    let must_include = must_include
        .into_iter()
        .map(|node| (node, 0))
        .collect::<HashMap<String, usize>>();

    count_paths_that_include_inner(
        connections,
        start,
        &target,
        must_include,
        &mut HashMap::new(),
    )
}

fn count_paths_that_include_inner(
    connections: &HashMap<String, Vec<String>>,
    curr: String,
    target: &String,
    mut must_include: HashMap<String, usize>,
    cache: &mut HashMap<CachedState, usize>,
) -> usize {
    if let Some(counter) = must_include.get_mut(&curr) {
        *counter += 1;
    }

    if curr == *target && must_include.iter().all(|(_, counter)| *counter > 0) {
        return 1;
    }

    let cached_state = CachedState {
        node: curr.clone(),
        included: must_include.clone(),
    };
    if let Some(cached) = cache.get(&cached_state) {
        return *cached;
    }

    let mut path_sum = 0;
    if let Some(to) = connections.get(&curr) {
        for node in to {
            path_sum += count_paths_that_include_inner(
                connections,
                node.clone(),
                target,
                must_include.clone(),
                cache,
            );
        }
    }

    cache.insert(cached_state, path_sum);

    path_sum
}

#[derive(Debug, PartialEq, Eq)]
struct CachedState {
    node: String,
    included: HashMap<String, usize>,
}

impl Hash for CachedState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.node.as_bytes());
        for i in self.included.iter() {
            state.write(i.0.as_bytes());
            state.write_usize(*i.1);
        }
        let _ = state.finish();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const INPUT_TWO: &str = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#;

    #[test]
    fn part_one() {
        let connections = parse_input(INPUT).unwrap();
        let paths = count_paths(&connections, "you".into(), "out".into());
        assert_eq!(paths, 5);
    }

    #[test]
    fn part_two() {
        let connections = parse_input(INPUT_TWO).unwrap();
        let paths = count_paths_that_include(
            &connections,
            "svr".into(),
            "out".into(),
            HashSet::from(["fft".into(), "dac".into()]),
        );
        assert_eq!(paths, 2);
    }
}
