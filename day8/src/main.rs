use rayon::prelude::*;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Milestone<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> From<&'a str> for Milestone<'a> {
    fn from(line: &'a str) -> Self {
        let path_re = Regex::new(r"^(\w+) = [(](\w+), (\w+)[)]$").unwrap();

        let capture = path_re
            .captures(line)
            .expect("line should match Milestone re");

        let name = capture
            .get(1)
            .expect("expect to have milestone name")
            .as_str();

        let left = capture.get(2).expect("expect to have left path").as_str();

        let right = capture.get(3).expect("expect to have right path").as_str();

        Milestone { name, left, right }
    }
}

#[derive(Debug)]
struct Paths<'a> {
    milestones: HashMap<&'a str, Milestone<'a>>,
}

impl<'a> From<&'a str> for Paths<'a> {
    fn from(input: &'a str) -> Self {
        let mut milestones: HashMap<&str, Milestone> = HashMap::new();

        for milestone in input.lines().map(|line| Milestone::from(line)) {
            milestones.insert(milestone.name, milestone);
        }

        Paths { milestones }
    }
}

#[derive(Debug)]
struct DesertMap<'a> {
    operations: &'a str,
    paths: Paths<'a>,
}

impl<'a> From<&'a str> for DesertMap<'a> {
    fn from(input: &'a str) -> Self {
        let (operations, input) = input
            .split_once("\n\n")
            .expect("expect input to have lines");

        let paths = Paths::from(input);

        DesertMap { operations, paths }
    }
}

fn ex1(input: &str) -> u64 {
    let desert_map = DesertMap::from(input);
    let milestones = &desert_map.paths.milestones;

    let mut op_count = 0;

    let mut position = "AAA";

    'traversal: loop {
        for op in desert_map.operations.chars() {
            if position == "ZZZ" {
                break 'traversal;
            }

            op_count += 1;
            let current_milestone = milestones.get(position).expect("Unknown position");

            position = match op {
                'L' => current_milestone.left,
                'R' => current_milestone.right,
                _ => unreachable!("Unknown operation {}", op),
            }
        }
    }

    op_count
}

fn find_op_count_to_z_suffix(
    operations: &str,
    milestones: &HashMap<&str, Milestone>,
    start: &str,
) -> u64 {
    let mut position = start;
    let mut op_count = 0u64;

    'traversal: loop {
        for op in operations.chars() {
            if position.ends_with("Z") {
                break 'traversal;
            }

            op_count += 1;

            let current_milestone = milestones.get(position).expect("Unknown position");

            position = match op {
                'L' => current_milestone.left,
                'R' => current_milestone.right,
                _ => unreachable!("Unknown operation {}", op),
            }
        }
    }

    op_count
}

fn gcd(x: u64, y: u64) -> u64 {
    if y != 0 {
        gcd(y, x % y)
    } else {
        x
    }
}

fn lcm(x: u64, y: u64) -> u64 {
    x * y / gcd(x, y)
}

fn ex2(input: &str) -> u64 {
    let desert_map = DesertMap::from(input);
    let milestones = &desert_map.paths.milestones;

    // find starting positions
    let mut starting_positions: Vec<&str> = milestones
        .keys()
        .filter(|name| name.ends_with("A"))
        .map(|name| *name)
        .collect();

    // find minimum operation counts for each starting position
    let min_op_counts = starting_positions
        .par_iter()
        .map(|position| {
            find_op_count_to_z_suffix(
                &desert_map.operations,
                &desert_map.paths.milestones,
                &position,
            )
        })
        .collect::<Vec<u64>>();

    // result is the Least Common Multiple for all the operation counts to
    // simulate loops of each positions until they synchronize
    min_op_counts
        .iter()
        .fold(*min_op_counts.first().unwrap(), |a, b| lcm(a, *b))
}

fn main() {
    let input = include_str!("../etc/input");

    println!("{}", ex1(input));
    println!("{}", ex2(input));
}
