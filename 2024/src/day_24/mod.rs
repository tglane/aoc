use anyhow::{Context, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Gate {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Gate {
    fn new(a: &str, b: &str, op: &str, out: &str) -> Result<Self> {
        let this = match op {
            "AND" => Self::And(a.into(), b.into(), out.into()),
            "OR" => Self::Or(a.into(), b.into(), out.into()),
            "XOR" => Self::Xor(a.into(), b.into(), out.into()),
            _ => anyhow::bail!(""),
        };
        Ok(this)
    }

    fn exec(&self, a: usize, b: usize) -> (String, usize) {
        match self {
            Self::And(_, _, name) => {
                let val = a & b;
                (name.clone(), val)
            }
            Self::Or(_, _, name) => {
                let val = a | b;
                (name.clone(), val)
            }
            Self::Xor(_, _, name) => {
                let val = a ^ b;
                (name.clone(), val)
            }
        }
    }

    fn a_name(&self) -> &str {
        match self {
            Self::And(name, _, _) => name,
            Self::Or(name, _, _) => name,
            Self::Xor(name, _, _) => name,
        }
    }

    fn b_name(&self) -> &str {
        match self {
            Self::And(_, name, _) => name,
            Self::Or(_, name, _) => name,
            Self::Xor(_, name, _) => name,
        }
    }

    fn out_name(&self) -> &str {
        match self {
            Self::And(_, _, name) => name,
            Self::Or(_, _, name) => name,
            Self::Xor(_, _, name) => name,
        }
    }
}

fn exec_logic_gates(wires: &mut HashMap<String, usize>, gates: &HashSet<Gate>) {
    // Create new set to track which wires are already set to a value
    let mut set_wires = HashMap::<String, bool>::new();
    for gate in gates.iter() {
        set_wires.insert(gate.out_name().to_string(), false);
    }
    for (wire, _) in wires.iter() {
        set_wires.insert(wire.clone(), true);
    }

    // Continue until all registers contain a value
    while set_wires.iter().any(|(_, is_set)| !is_set) {
        for gate in gates.iter() {
            let out_set = set_wires.get_mut(gate.out_name()).unwrap();
            if !*out_set {
                if let (Some(val_a), Some(val_b)) =
                    (wires.get(gate.a_name()), wires.get(gate.b_name()))
                {
                    let (out_name, out_val) = gate.exec(*val_a, *val_b);
                    *wires.entry(out_name.clone()).or_default() = out_val;
                    *out_set = true;
                }
            }
        }
    }
}

fn calc_output_val(wires: &HashMap<String, usize>) -> usize {
    let mut out = 0;

    // Wire names only contain of exactly three characters so z can take names from z00 to z99
    for i in 0..100 {
        let name = format!("z{:02}", i);
        if let Some(out_val) = wires.get(&name) {
            out |= out_val << i;
        } else {
            break;
        }
    }

    out
}

fn detect_malformed_gates(gates: &HashSet<Gate>) -> Vec<String> {
    let mut malformed = HashSet::new();

    let mut connections = HashMap::<&str, Vec<&str>>::new();
    for g in gates {
        connections
            .entry(g.a_name())
            .or_default()
            .push(g.out_name());
        connections
            .entry(g.b_name())
            .or_default()
            .push(g.out_name());
    }

    // Circuit just implements a full adder:
    //
    //   Zi = (Xi XOR Yi) XOR Ci-1
    //   Ci = (Xi AND Yi) OR ((Xi XOR Yi) AND Ci-1)
    //
    // Try to find nodes that do not fulfill those requirements
    for g in gates {
        // z gates must be XOR
        if g.out_name().starts_with('z')
            && !matches!(g, Gate::Xor(_, _, _))
            && g.out_name() != "z45"
        {
            malformed.insert(g.out_name().to_string());
        }

        // z gates can not be input of other nodes
        if g.a_name().starts_with('z') {
            malformed.insert(g.a_name().to_string());
        }
        if g.b_name().starts_with('z') {
            malformed.insert(g.b_name().to_string());
        }

        // XOR gates must output to z or use x and y as input
        if matches!(g, Gate::Xor(_, _, _)) {
            if !g.out_name().starts_with('z')
                && !((g.a_name().starts_with('x') && g.b_name().starts_with('y'))
                    || (g.a_name().starts_with('y') && g.b_name().starts_with('x')))
            {
                malformed.insert(g.out_name().to_string());
            }
        }

        // XOR gates (except for z) must be input of exactly two other gates
        if matches!(g, Gate::Xor(_, _, _))
            && !g.out_name().starts_with('z')
            && connections[g.out_name()].len() != 2
        {
            malformed.insert(g.out_name().to_string());
        }

        // AND gates must be input of exactly one other gate
        if matches!(g, Gate::And(_, _, _))
            && !g.out_name().starts_with('z')
            && connections[g.out_name()].len() != 1
            && !((g.a_name() == "x00" && g.b_name() == "y00")
                || (g.a_name() == "y00" && g.b_name() == "x00"))
        {
            malformed.insert(g.out_name().to_string());
        }
    }

    let mut malformed = malformed.into_iter().collect::<Vec<_>>();
    malformed.sort();
    malformed
}

fn parse_input(input: &str) -> Result<(HashMap<String, usize>, HashSet<Gate>)> {
    let (init_lines, gate_lines) = input.split_once("\n\n").context("")?;

    let init_re = Regex::new("(?<name>.+): (?<val>.+)")?;
    let init = init_lines
        .lines()
        .map(|l| {
            let cap = init_re.captures(l).context("No init regex match")?;
            let (_, [name, val]) = cap.extract();
            Ok((name.into(), val.parse()?))
        })
        .collect::<Result<HashMap<_, _>>>()?;

    let gate_re = Regex::new("(?<a>.+) (?<gate>AND|OR|XOR) (?<b>.+) -> (?<out>.+)")?;
    let gates = gate_lines
        .lines()
        .map(|g| {
            let cap = gate_re.captures(g).context("No gate regex match")?;
            let (_, [a, op, b, out]) = cap.extract();
            Ok(Gate::new(a, b, op, out)?)
        })
        .collect::<Result<HashSet<_>>>()?;

    Ok((init, gates))
}

pub fn run() -> Result<()> {
    let input = std::fs::read_to_string(format!(
        "{}/src/day_24/input.txt",
        env!("CARGO_MANIFEST_DIR")
    ))?;
    let (mut wires, gates) = parse_input(&input).unwrap();

    exec_logic_gates(&mut wires, &gates);
    let z = calc_output_val(&wires);
    println!("Day 24, Part 1: Z register value: {z}");

    let malformed = detect_malformed_gates(&gates);
    let malformed_str = malformed.join(",");
    println!("Day 24, Part 2: Malformed gates: {malformed_str}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part_one() {
        let (mut wires, gates) = parse_input(INPUT).unwrap();
        println!("{}", gates.len());

        exec_logic_gates(&mut wires, &gates);
        println!("Test: {}", wires.len());
        for w in wires.iter() {
            if w.0.starts_with('z') {
                println!("{w:?}");
            }
        }

        let z = calc_output_val(&wires);
        println!("{z:0b}");
        assert_eq!(z, 2024);
    }
}
