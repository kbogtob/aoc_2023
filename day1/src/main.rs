fn ex1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| parse_first_two_digit_number(line))
        .sum()
}

fn parse_first_two_digit_number(line: &str) -> i32 {
    let digit_chars: Vec<char> = line.chars().filter(|s| s.is_ascii_digit()).collect();

    let first_digit = digit_chars
        .first()
        .expect("Expect at least one number digit in line");
    let last_digit = digit_chars
        .last()
        .expect("Expect at least one number digit in line");

    format!("{}{}", first_digit, last_digit)
        .parse::<i32>()
        .expect("Line should have a two digit number")
}

fn ex2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| hack_replace_parse_first_two_digit_number_with_letters(line))
        .sum()
}

fn hack_replace_parse_first_two_digit_number_with_letters(line: &str) -> i32 {
    let replaced = line
        .to_lowercase()
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "th3ree")
        .replace("four", "fo4r")
        .replace("five", "f5ve")
        .replace("six", "s6x")
        .replace("seven", "se7en")
        .replace("eight", "ei8ht")
        .replace("nine", "n9ne");

    parse_first_two_digit_number(&replaced)
}

fn main() {
    let input = include_str!("../etc/input");

    println!("ex1: {}", ex1(input));
    println!("ex2: {}", ex2(input));
}
