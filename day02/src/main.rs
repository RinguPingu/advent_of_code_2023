use lazy_regex::regex_find;

enum Color {
    Red,
    Green,
    Blue,
}

struct Cubes {
    color: Color,
    amount: u32,
}

impl Cubes {
    fn new(color: Color, amount: u32) -> Self {
        Cubes { color, amount }
    }
}

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
                        _ => panic!("INVALID COLOR"),
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
            }) {
                game.possible = false;
            }
        }
    }

    let part1_result: u32 = games.iter().filter(|g| g.possible).map(|g| g.id).sum();
    println!("Part 1 Result: {part1_result}");

    let mut powers: Vec<u32> = Vec::new();

    for game in &games {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for show in &game.shows {
            for cubes in show {
                match cubes.color {
                    Color::Red => {
                        if cubes.amount > red {
                            red = cubes.amount;
                        }
                    }
                    Color::Green => {
                        if cubes.amount > green {
                            green = cubes.amount;
                        }
                    }
                    Color::Blue => {
                        if cubes.amount > blue {
                            blue = cubes.amount;
                        }
                    }
                }
            }
        }
        powers.push(red * green * blue);
    }

    let part2_result: u32 = powers.iter().sum();
    println!("Part 2 Result: {part2_result}");
}
