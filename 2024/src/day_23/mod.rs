use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{BTreeSet, HashMap, HashSet};

fn parse_input(input: &str) -> Result<HashMap<String, BTreeSet<String>>> {
    let parsed = input
        .lines()
        .map(|l| {
            let re = Regex::new("(?<a>[a-z]+)-(?<b>[a-z]+)")?;
            let cap = re.captures(l).context("")?;
            let (_, [a, b]) = cap.extract();
            Ok((a.into(), b.into()))
        })
        .collect::<Result<Vec<(String, String)>>>()?;

    let mut connections = HashMap::<String, BTreeSet<String>>::new();

    for conn in parsed.into_iter() {
        connections
            .entry(conn.0.clone())
            .or_default()
            .insert(conn.1.clone());
        connections.entry(conn.1).or_default().insert(conn.0);
    }

    Ok(connections)
}

fn group_connections(
    connections: &HashMap<String, BTreeSet<String>>,
) -> Result<HashSet<BTreeSet<String>>> {
    let mut groups = HashSet::<BTreeSet<String>>::new();
    for (start, s_reacheable) in connections.iter() {
        for end in s_reacheable.iter().rev() {
            for e_reacheable in connections.get(end).context("")?.iter().rev() {
                if connections.get(e_reacheable).context("")?.contains(start) {
                    groups.insert(BTreeSet::from([
                        start.clone(),
                        end.clone(),
                        e_reacheable.clone(),
                    ]));
                }
            }
        }
    }
    Ok(groups)
}

fn largest_group(connections: &HashMap<String, BTreeSet<String>>) -> Result<BTreeSet<String>> {
    let mut groups = HashSet::<BTreeSet<String>>::new();
    for (start, s_reacheable) in connections.iter() {
        for end in s_reacheable.iter().rev() {
            for e_reacheable in connections.get(end).context("")?.iter().rev() {
                if connections.get(e_reacheable).context("")?.contains(start) {
                    let mut matches = groups
                        .iter()
                        .filter(|g| {
                            let all_reacheable = g
                                .iter()
                                .all(|gm| connections.get(gm).unwrap().contains(end));

                            all_reacheable && g.contains(start) && g.contains(e_reacheable)
                        })
                        .cloned()
                        .collect::<Vec<_>>();

                    for mut g in matches.drain(..) {
                        groups.remove(&g);
                        g.insert(end.clone());
                        groups.insert(g);
                    }

                    groups.insert(BTreeSet::from([
                        start.clone(),
                        end.clone(),
                        e_reacheable.clone(),
                    ]));
                }
            }
        }
    }

    let mut max_len = 0;
    let mut largest_set = None;

    for g in groups.iter() {
        if g.len() > max_len {
            max_len = g.len();
            largest_set = Some(g);
        }
    }

    largest_set.cloned().context("")
}

fn group_password(group: &BTreeSet<String>) -> String {
    group.iter().cloned().collect::<Vec<_>>().join(",")
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_23/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let connections = parse_input(&input)?;
    let groups = group_connections(&connections)?;

    let groups_containing_t = groups
        .iter()
        .filter(|g| g.len() >= 3 && g.iter().any(|n| n.starts_with('t')))
        .count();
    println!("Day 23, Part 1: Number of groups that contain 't': {groups_containing_t}");

    let largest_group = largest_group(&connections)?;
    let password = group_password(&largest_group);
    println!("Day 23, Part 2: Password of largest group: {password}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part_one() {
        let connections = parse_input(INPUT).unwrap();
        let groups = group_connections(&connections).unwrap();
        let groups_containing_t = groups
            .iter()
            .filter(|g| g.len() >= 3 && g.iter().any(|n| n.starts_with('t')))
            .count();
        assert_eq!(groups_containing_t, 7);
    }

    #[test]
    fn part_two() {
        let connections = parse_input(INPUT).unwrap();
        let largest_group = largest_group(&connections).unwrap();
        let password = group_password(&largest_group);
        assert_eq!(password, "co,de,ka,ta".to_string());
    }
}
