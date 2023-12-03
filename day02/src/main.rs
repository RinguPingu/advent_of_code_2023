use lazy_regex::regex_find;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Color {
    None,
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cubes {
    color: Color,
    amount: u32,
}

impl Cubes {
    fn new(color: Color, amount: u32) -> Self {
        Cubes { color, amount }
    }
}

impl PartialEq for Cubes {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.amount == other.amount
    }
}

impl PartialOrd for Cubes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.amount.cmp(&other.amount))
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    shows: Vec<Vec<Cubes>>,
    possible: bool,
}

impl Game {
    fn new(id: u32) -> Self {
        Game {
            id,
            shows: Vec::new(),
            possible: true,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let red_constraint: u32 = 12;
    let green_constraint: u32 = 13;
    let blue_constraint: u32 = 14;

    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let mut split = line.split(':');
        let mut game = Game::new(
            regex_find!(r#"\d+"#, split.next().unwrap())
                .unwrap()
                .parse::<u32>()
                .unwrap(),
        );
        for group in split {
            for round in group.split(';') {
                let mut show: Vec<Cubes> = Vec::new();
                for cubes in round.split(',') {
                    let mut amount_and_color = cubes.split_whitespace();
                    let amount: u32 = amount_and_color.next().unwrap().parse().unwrap();
                    let color: Color = match amount_and_color.next().unwrap() {
                        "red" => Color::Red,
                        "blue" => Color::Blue,
                        "green" => Color::Green,
                        _ => Color::None,
                    };
                    show.push(Cubes::new(color, amount));
                }
                game.shows.push(show);
            }
        }
        games.push(game);
    }

    for game in &mut games {
        for show in &game.shows {
            if show.iter().any(|s| match s.color {
                Color::Red => s.amount > red_constraint,
                Color::Green => s.amount > green_constraint,
                Color::Blue => s.amount > blue_constraint,
                _ => false,
            }) {
                game.possible = false;
            }
        }
    }

    let part1_result: u32 = games.iter().filter(|g| g.possible).map(|g| g.id).sum();

    println!("Part 1 Result: {part1_result}");
}
