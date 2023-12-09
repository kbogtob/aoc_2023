use regex::Regex;

fn differentiate(numbers: &Vec<i64>) -> Vec<i64> {
    numbers.windows(2).map(|pair| pair[1] - pair[0]).collect()
}

fn predict_next(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }

    let differences = differentiate(&numbers);

    numbers.last().unwrap() + predict_next(&differences)
}

fn predict_prev(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        return 0;
    }

    let differences = differentiate(&numbers);

    numbers.first().unwrap() - predict_prev(&differences)
}

#[derive(Debug)]
struct Sequence {
    measures: Vec<i64>,
}

impl Sequence {
    fn next(&self) -> i64 {
        predict_next(&self.measures)
    }

    fn prev(&self) -> i64 {
        predict_prev(&self.measures)
    }
}

impl From<&str> for Sequence {
    fn from(line: &str) -> Self {
        let numbers_re = Regex::new(r"(-?\d+)").unwrap();

        let measures = numbers_re
            .captures_iter(line)
            .map(|capture| capture.get(1).expect("Line should have a number"))
            .map(|group| group.as_str())
            .map(|number| number.parse::<i64>().expect("Numbers should be numbers"))
            .collect();

        Sequence { measures }
    }
}

fn ex1(input: &str) -> i64 {
    let sequences = input
        .lines()
        .map(|line| Sequence::from(line))
        .collect::<Vec<Sequence>>();

    sequences
        .iter()
        .fold(0i64, |total, sequence| total + sequence.next())
}

fn ex2(input: &str) -> i64 {
    let sequences = input
        .lines()
        .map(|line| Sequence::from(line))
        .collect::<Vec<Sequence>>();

    sequences
        .iter()
        .fold(0i64, |total, sequence| total + sequence.prev())
}

fn main() {
    let input = include_str!("../etc/input");

    println!("{}", ex1(input));
    println!("{}", ex2(input));
}
