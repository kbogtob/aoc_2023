use std::ops::RangeInclusive;

use regex::Regex;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn pressing_times_beating_distance(&self) -> Option<RangeInclusive<u64>> {
        match self.equation_polynom().solve_root() {
            PolynomSolution::NoSolution => None,
            PolynomSolution::Solution(_) => None,
            PolynomSolution::TwoSolution(first_sol, second_sol) => Some(RangeInclusive::new(
                (first_sol + 1f64).floor() as u64,
                (second_sol - 1f64).ceil() as u64,
            )),
        }
    }

    fn equation_polynom(&self) -> Polynom {
        // distance formula is: (race_time - time_we_press) * time_we_press
        // distance of beat is race_record
        // equation is then (race_time - time_we_press) * time_we_press > race_record
        // equation is then -time_we_press^2 + time_we_press*race_time - race_record > 0
        // if what we want to solve is the time_we_press, then polynom becomes
        // -x^2 + race_time*x - race_record > 0
        Polynom {
            a: -1f64,
            b: self.time as f64,
            c: -(self.distance as f64),
        }
    }
}

#[derive(Debug)]
struct Polynom {
    a: f64,
    b: f64,
    c: f64,
}

#[derive(Debug)]
enum PolynomSolution {
    NoSolution,
    Solution(f64),
    TwoSolution(f64, f64),
}

impl Polynom {
    fn solve_root(&self) -> PolynomSolution {
        let discriminant = self.b * self.b - 4f64 * self.a * self.c;

        match discriminant.total_cmp(&0f64) {
            std::cmp::Ordering::Less => PolynomSolution::NoSolution,
            std::cmp::Ordering::Equal => PolynomSolution::Solution(-self.b / (2f64 * self.a)),
            std::cmp::Ordering::Greater => PolynomSolution::TwoSolution(
                (-self.b + discriminant.sqrt()) / (2f64 * self.a),
                (-self.b - discriminant.sqrt()) / (2f64 * self.a),
            ),
        }
    }
}

fn parse_numbers(line: &str) -> Vec<u64> {
    let number_re = Regex::new(r"((\d+)\D*)").unwrap();

    number_re
        .captures_iter(line)
        .map(|capture| {
            let capture_match = capture.get(2).expect("Should have inner number");
            capture_match
                .as_str()
                .parse::<u64>()
                .expect("Number should be a number")
        })
        .collect()
}

fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();

    let times_line = lines.next().expect("expect a time description");
    let times = parse_numbers(times_line);

    let distances_line = lines.next().expect("expect a distance description");
    let distances = parse_numbers(distances_line);

    if distances.len() != times.len() {
        panic!("There should have been the same number of distances and times");
    }

    (0..distances.len())
        .map(|index| {
            let distance = distances[index];
            let time = times[index];

            Race { distance, time }
        })
        .collect()
}

fn ex1() -> u64 {
    let input = include_str!("../etc/input");

    let races = parse(input);
    races.iter().fold(1, |accumulator, race| {
        let multiplier = match race.pressing_times_beating_distance() {
            Some(range) => range.end() - range.start() + 1,
            None => 1,
        };

        accumulator * multiplier
    })
}

fn ex2() -> u64 {
    let input = include_str!("../etc/input_fixed");

    let races = parse(input);
    races.iter().fold(1, |accumulator, race| {
        let multiplier = match race.pressing_times_beating_distance() {
            Some(range) => range.end() - range.start() + 1,
            None => 1,
        };

        accumulator * multiplier
    })
}

fn main() {
    let input = include_str!("../etc/input_fixed");

    let races = parse(input);

    println!("{}", ex1());
    println!("{}", ex2());
}
