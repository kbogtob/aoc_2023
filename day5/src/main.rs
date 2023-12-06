use std::ops::RangeInclusive;

#[derive(Debug)]
struct Mapper {
    src_start: u64,
    src_end: u64,
    dest_start: u64,
}

impl From<&str> for Mapper {
    fn from(line: &str) -> Self {
        let mut line_it = line.split(" ");

        let dest_start = line_it
            .next()
            .expect("Expecting a destination start")
            .parse::<u64>()
            .expect("Expect destination start to be a number");
        let src_start = line_it
            .next()
            .expect("Expecting a source start")
            .parse::<u64>()
            .expect("Expect source start to be a number");

        let range_width = line_it
            .next()
            .expect("Expecting a range width")
            .parse::<u64>()
            .expect("Expect range width to be a number");

        Mapper {
            src_start,
            src_end: src_start + range_width - 1,
            dest_start,
        }
    }
}

impl Mapper {
    fn map(&self, source: u64) -> u64 {
        source - self.src_start + self.dest_start
    }

    fn is_covering(&self, number: u64) -> bool {
        (self.src_start..=self.src_end).contains(&number)
    }

    fn is_overlapping(&self, number_range: &RangeInclusive<u64>) -> bool {
        let src_range = &self.src_start..&self.src_end;

        number_range.contains(&self.src_start)
            || number_range.contains(&self.src_end)
            || src_range.contains(&number_range.start())
            || src_range.contains(&number_range.end())
    }
}

#[derive(Debug)]
struct Operation {
    mappers: Vec<Mapper>,
}

impl From<&str> for Operation {
    fn from(operation_description: &str) -> Self {
        let lines = operation_description.split("\n");

        // drop first line as operation name are not important
        let mut mappers = lines
            .skip(1)
            .map(|mapper_description| Mapper::from(mapper_description))
            .collect::<Vec<Mapper>>();

        mappers.sort_by_key(|mapper| mapper.src_start);

        Operation { mappers }
    }
}

impl Operation {
    fn map(&self, number: u64) -> u64 {
        self.closest_mapper(number)
            .map(|mapper| mapper.map(number))
            .unwrap_or(number)
    }

    fn closest_mapper(&self, number: u64) -> Option<&Mapper> {
        let closest_mapper_id = self
            .mappers
            .binary_search_by_key(&number, |mapper| mapper.src_start);

        if let Ok(mapper_id) = closest_mapper_id {
            return self.mappers.get(mapper_id);
        }

        let previous_id = closest_mapper_id.err().unwrap();

        if previous_id == 0 {
            return None;
        }

        return self
            .mappers
            .get(previous_id - 1)
            .filter(|previous_mapper| previous_mapper.is_covering(number));
    }

    fn map_range(&self, number_range: &RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let matching_mappers = self.matching_mappers(number_range);

        let mut mapped_ranges: Vec<RangeInclusive<u64>> = Vec::new();

        // set end of previous mapper as start to fill gap if necessary with a 1:1 range
        let mut previous_mapper_end = *number_range.start();

        for matching_mapper in matching_mappers.iter() {
            // if previous mapper finished before remaining range start
            if previous_mapper_end < matching_mapper.src_start {
                // add a 1:1 mapping in between matching mappers
                mapped_ranges.push(RangeInclusive::new(
                    previous_mapper_end,
                    matching_mapper.src_start - 1,
                ));
                // mimick as if the previous mapper finished at the next mapper start
                previous_mapper_end = matching_mapper.src_start;
            }

            let mapped_start = matching_mapper.map(previous_mapper_end);

            let mapped_end = if *number_range.end() > matching_mapper.src_end {
                previous_mapper_end = matching_mapper.src_end + 1;
                matching_mapper.map(matching_mapper.src_end)
            } else {
                previous_mapper_end = *number_range.end() + 1;
                matching_mapper.map(*number_range.end())
            };

            mapped_ranges.push(RangeInclusive::new(mapped_start, mapped_end));
        }

        // if the last mapper finished before the end, fill gap with 1:1 range
        if previous_mapper_end <= *number_range.end() {
            mapped_ranges.push(RangeInclusive::new(
                previous_mapper_end,
                *number_range.end(),
            ))
        }

        mapped_ranges
    }

    fn matching_mappers(&self, range: &RangeInclusive<u64>) -> Vec<&Mapper> {
        self.mappers
            .iter()
            .filter(|mapper| mapper.is_overlapping(&range))
            .collect()
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    operations: Vec<Operation>,
}

impl From<&str> for Almanac {
    fn from(input: &str) -> Self {
        // reading seeds
        let (seeds, input) = input
            .split_once("\n")
            .expect("Expecting multiple lines in file");
        let (_, seeds) = seeds.split_once(": ").expect("Expecting seeds after :");
        let seeds = seeds
            .split(" ")
            .map(|seed_number| {
                seed_number
                    .parse::<u64>()
                    .expect("Expecting seed number to be a number")
            })
            .collect();

        // reading operations
        let operations = input[1..]
            .split("\n\n")
            .map(|operation_description| Operation::from(operation_description))
            .collect();

        Almanac { seeds, operations }
    }
}

impl Almanac {
    fn map(&self, seed_number: u64) -> u64 {
        self.operations
            .iter()
            .fold(seed_number, |number_to_map, operation| {
                operation.map(number_to_map)
            })
    }

    fn seed_ranges(&self) -> Vec<RangeInclusive<u64>> {
        self.seeds
            .chunks(2)
            .map(|pair| {
                let start = pair[0];
                let end = pair[0] + pair[1] - 1;
                RangeInclusive::new(start, end)
            })
            .collect()
    }

    fn map_range(&self, seed_range: &RangeInclusive<u64>) -> Vec<RangeInclusive<u64>> {
        let mut ranges = vec![seed_range.clone()];

        for operation in self.operations.iter() {
            ranges = ranges
                .iter()
                .map(|range| operation.map_range(range))
                .flatten()
                .collect::<Vec<RangeInclusive<u64>>>();
        }

        ranges
    }
}

fn ex1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|seed_number| almanac.map(*seed_number))
        .min()
        .expect("Expect at least one seed location")
}

fn ex2(almanac: &Almanac) -> u64 {
    almanac
        .seed_ranges()
        .iter()
        .map(|seed_number| almanac.map_range(seed_number))
        .flatten()
        .map(|range| *range.start())
        .min()
        .expect("Expect at least one seed location")
}

fn main() {
    let input = include_str!("../etc/input");

    let almanac = Almanac::from(input);

    println!("{}", ex1(&almanac));
    println!("{}", ex2(&almanac));
}
