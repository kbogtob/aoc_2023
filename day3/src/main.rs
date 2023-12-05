use std::{collections::HashMap, ops::RangeInclusive};

use regex::Regex;

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn get_at(&self, col_index: usize, row_index: usize) -> Option<&char> {
        self.data
            .get(row_index)
            .map(|row| row.get(col_index))
            .flatten()
    }

    fn near_symbol(&self, col_start: usize, col_end: usize, row_index: usize) -> bool {
        let row_range = RangeInclusive::new(
            if row_index == 0 {
                row_index
            } else {
                row_index - 1
            },
            row_index + 1,
        );

        for row in row_range {
            let col_range = RangeInclusive::new(
                if col_start == 0 {
                    col_start
                } else {
                    col_start - 1
                },
                col_end + 1,
            );

            for col in col_range {
                let have_found_symbol = self
                    .get_at(col, row)
                    .map(|char| !char.is_ascii_digit() && *char != '.')
                    .unwrap_or(false);

                if have_found_symbol {
                    return true;
                }
            }
        }

        false
    }

    fn surrounding_stars(
        &self,
        col_start: usize,
        col_end: usize,
        row_index: usize,
    ) -> Vec<(usize, usize)> {
        let mut surrounding_stars: Vec<(usize, usize)> = Vec::new();
        let row_range = RangeInclusive::new(
            if row_index == 0 {
                row_index
            } else {
                row_index - 1
            },
            row_index + 1,
        );

        for row in row_range {
            let col_range = RangeInclusive::new(
                if col_start == 0 {
                    col_start
                } else {
                    col_start - 1
                },
                col_end + 1,
            );

            for col in col_range {
                let have_found_star = self
                    .get_at(col, row)
                    .map(|char| *char == '*')
                    .unwrap_or(false);

                if have_found_star {
                    surrounding_stars.push((col, row));
                }
            }
        }

        surrounding_stars
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let data = input.lines().map(|line| line.chars().collect()).collect();

        Grid { data }
    }
}

fn ex1(input: &str) -> u32 {
    let number_re = Regex::new(r"((\d+)\D*)").unwrap();
    let grid: Grid = input.into();

    let mut sum = 0;

    for (row_index, row) in input.lines().enumerate() {
        for capture in number_re.captures_iter(row) {
            let number_match = capture
                .get(2)
                .expect("Should have captured a number with regex");

            if grid.near_symbol(number_match.start(), number_match.end() - 1, row_index) {
                sum += number_match
                    .as_str()
                    .parse::<u32>()
                    .expect("Number should be a number");
            }
        }
    }

    sum
}

fn ex2(input: &str) -> u32 {
    let number_re = Regex::new(r"((\d+)\D*)").unwrap();
    let grid: Grid = input.into();

    let mut numbers_by_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    // indexing all the part numbers by star coordinates
    for (row_index, row) in input.lines().enumerate() {
        for capture in number_re.captures_iter(row) {
            let number_match = capture
                .get(2)
                .expect("Should have captured a number with regex");
            let surrounding_stars =
                grid.surrounding_stars(number_match.start(), number_match.end() - 1, row_index);

            let part_number = number_match
                .as_str()
                .parse::<u32>()
                .expect("Number should be a number");

            for surrounding_star_coordinate in surrounding_stars {
                numbers_by_gears
                    .entry(surrounding_star_coordinate)
                    .and_modify(|related_part_numbers| related_part_numbers.push(part_number))
                    .or_insert(vec![part_number]);
            }
        }
    }

    // unrolling stars to multiply gear ratios
    let mut gear_ratios = 0;

    for (_, part_numbers) in numbers_by_gears.iter() {
        if part_numbers.len() > 1 {
            gear_ratios += part_numbers
                .iter()
                .fold(1, |accumulator, part_number| accumulator * part_number);
        }
        if part_numbers.len() > 2 {
            println!("Something is up");
        }
    }

    gear_ratios
}

fn main() {
    let input = include_str!("../etc/input");

    println!("{}", ex1(input));
    println!("{}", ex2(input));
}
