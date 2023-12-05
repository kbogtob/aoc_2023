use std::collections::HashMap;

#[derive(Debug)]
struct Game<'a> {
    id: i32,
    max_used_by_color: HashMap<&'a str, u32>,
}

impl<'a> From<&'a str> for Game<'a> {
    fn from(line: &'a str) -> Self {
        let game_info: Vec<&'a str> = line.split(":").collect();
        let id: i32 = game_info
            .get(0)
            .map(|introduction| introduction.split(" ").last())
            .flatten()
            .expect("game description should have game ID before :")
            .parse()
            .expect("game ID should be a number");

        let mut max_used_by_color = HashMap::new();

        let game_results = *game_info.get(1).expect("Game should have results");

        let reveals: Vec<&str> = game_results.split(";").collect();

        for reveal in reveals {
            let cube_descriptions: Vec<&str> = reveal
                .split(",")
                .map(|cube_desc| cube_desc.trim())
                .collect();

            for cube_description in cube_descriptions {
                let cube_description: Vec<&str> = cube_description.split(" ").collect();

                let cube_amount = *cube_description.get(0).expect("cube should have an amount");
                let cube_amount: u32 = cube_amount.parse().expect("cube amount should be a number");
                let cube_color = *cube_description.get(1).expect("cube should have a color");

                max_used_by_color
                    .entry(cube_color)
                    .and_modify(|current_max| {
                        if (*current_max < cube_amount) {
                            *current_max = cube_amount;
                        }
                    })
                    .or_insert(cube_amount);
            }
        }

        Game {
            id,
            max_used_by_color,
        }
    }
}

impl Game<'_> {
    fn validates(&self, max_cubes: &HashMap<&str, u32>) -> bool {
        for (color, max_amount) in max_cubes.iter() {
            match self.max_used_by_color.get(*color) {
                Some(amount) => {
                    if amount > max_amount {
                        return false;
                    }
                }
                None => return false,
            }
        }

        true
    }

    fn minimum_set_power(&self) -> u32 {
        self.max_used_by_color
            .values()
            .fold(1u32, |power, used_color| power * used_color)
    }
}

fn ex1(games: &Vec<Game>) -> i32 {
    let max_cubes: HashMap<&str, u32> = {
        let mut m = HashMap::new();
        m.insert("red", 12);
        m.insert("green", 13);
        m.insert("blue", 14);
        m
    };

    let invalid_games = games
        .iter()
        .filter(|game| game.validates(&max_cubes))
        .map(|game| game.id)
        .sum();

    invalid_games
}

fn ex2(games: &Vec<Game>) -> u32 {
    games.iter().map(|game| game.minimum_set_power()).sum()
}

fn main() {
    let input = include_str!("../etc/input");
    let games: Vec<Game> = input.lines().map(|line| Game::from(line)).collect();

    println!("{}", ex1(&games));
    println!("{}", ex2(&games));
}
