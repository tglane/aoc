use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::collections::HashMap;

#[derive(Clone)]
struct Rule {
    from: String,
    to: String,
}

fn parse_input(filename: &str) -> Result<Vec<Rule>, Error> {
    let reader = BufReader::new(File::open(filename)?);

    let out: Vec<Rule> = reader
        .lines()
        .map(|line| -> Rule {
            let line_str = line.unwrap();
            let mut line_split = line_str.split('-');
            Rule { from: line_split.next().unwrap().to_owned(), to: line_split.next().unwrap().to_owned() }
        })
        .collect();
    Ok(out)
}

fn build_map(rules: &[Rule]) -> HashMap<&str, Vec<String>> {
    let mut map = HashMap::<&str, Vec<String>>::new();

    for rule in rules.iter() {
        if map.contains_key(&rule.from[..]) {
            map.entry(&rule.from[..]).and_modify(|str_vec| str_vec.push(rule.to.clone()));
        } else {
            map.insert(&rule.from[..], vec![rule.to.clone(); 1]);
        }

        if map.contains_key(&rule.to[..]) {
            map.entry(&rule.to[..]).and_modify(|str_vec| str_vec.push(rule.from.clone()));
        } else {
            map.insert(&rule.to[..], vec![rule.from.clone(); 1]);
        }
    }

    map
}

fn advanced_step(curr_cave: &str, map: &HashMap<&str, Vec<String>>, recent: HashMap<&str, u64>, double_pass_used: bool) -> u64 {
    if curr_cave == "end" {
        return 1;
    } else {
        let mut new_routes = 0;
        for next in map[curr_cave].iter() {
            if next != "start" {
                if next.chars().all(char::is_uppercase) || !recent.contains_key(&next as &str) {
                    let mut recent_clone = recent.clone();
                    *recent_clone.entry(next).or_insert(0) += 1;
                    new_routes += advanced_step(&next, &map, recent_clone, double_pass_used);
                } else if !double_pass_used {
                    new_routes += advanced_step(&next, &map, recent.clone(), true);
                }
            }
        }
        return new_routes;
    }
}

fn one(map: &HashMap<&str, Vec<String>>) {
    let route_count = advanced_step("start", &map, HashMap::<&str, u64>::new(), true);
    println!("ONE: Routes from start to end: {}", route_count);
}

fn two(map: &HashMap<&str, Vec<String>>) {
    let route_count = advanced_step("start", &map, HashMap::<&str, u64>::new(), false);
    println!("TWO: Routes from start to end: {}", route_count);
}

fn main() {
    let input = parse_input("in.txt")
        .expect("Failed to parse input");
    let map = build_map(&input);

    one(&map);
    two(&map);
}
