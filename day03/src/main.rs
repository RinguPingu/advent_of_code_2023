use grid::Grid;

#[derive(Debug)]
struct Digit {
    glyph: char,
    position: (usize, usize),
    valid: bool,
}

impl Digit {
    fn new(glyph: char, position: (usize, usize)) -> Self {
        Digit {
            glyph,
            position,
            valid: false,
        }
    }
}

#[derive(Debug)]
struct Gear {
    position: (usize, usize),
    part_numbers: Vec<u32>,
}

impl Gear {
    fn new(position: (usize, usize)) -> Self {
        Gear {
            position,
            part_numbers: Vec::new(),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();

    let mut grid: Grid<char> = Grid::new(rows, cols);

    for y in input.lines().enumerate() {
        for x in y.1.chars().enumerate() {
            grid[(y.0, x.0)] = x.1;
        }
    }

    println!("{:#?}", grid);

    let mut digits: Vec<Digit> = Vec::new();
    let mut gears: Vec<Gear> = Vec::new();

    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            let temp = grid.get(row, col).unwrap();
            if temp.is_numeric() {
                digits.push(Digit::new(*temp, (row, col)));
            } else if *temp == '*' {
                gears.push(Gear::new((row, col)));
            }
        }
    }

    for digit in &mut digits {
        let min_row = if digit.position.0 == 0 {
            0
        } else {
            digit.position.0 - 1
        };
        let max_row = if digit.position.0 == grid.rows() - 1 {
            digit.position.0
        } else {
            digit.position.0 + 1
        };

        let min_col = if digit.position.1 == 0 {
            0
        } else {
            digit.position.1 - 1
        };
        let max_col = if digit.position.1 == grid.cols() - 1 {
            digit.position.1
        } else {
            digit.position.1 + 1
        };

        for row in min_row..=max_row {
            for col in min_col..=max_col {
                let c = grid.get(row, col).unwrap();
                if !c.is_numeric() && *c != '.' {
                    digit.valid = true;
                }
            }
        }
    }

    let mut adjacent_digits: Vec<Vec<&Digit>> = Vec::new();

    let mut temp: Vec<&Digit> = Vec::new();

    for digit in digits.iter().enumerate() {
        let mut push_temp = false;
        if temp.is_empty() {
            temp.push(digit.1);
        } else {
            let d = temp.last().unwrap();
            if digit.1.position.0 == d.position.0 {
                if digit.1.position.1 - d.position.1 == 1 {
                    temp.push(digit.1);
                    if digit.0 == digits.len() - 1 {
                        push_temp = true;
                    }
                } else {
                    push_temp = true;
                }
            } else {
                push_temp = true;
            }
        }
        if push_temp {
            adjacent_digits.push(temp.clone());
            temp.clear();
            temp.push(digit.1);
        }
    }

    adjacent_digits.retain(|x| x.iter().any(|d| d.valid));

    let mut valid_part_numbers: Vec<u32> = Vec::new();

    for number in adjacent_digits.iter() {
        valid_part_numbers.push(
            number
                .iter()
                .map(|x| x.glyph)
                .collect::<String>()
                .parse::<u32>()
                .unwrap(),
        );
    }

    let part1_result: u32 = valid_part_numbers.iter().sum();
    println!("Part 1 Result: {}", part1_result);

    for gear in &mut gears {
        let min_row = if gear.position.0 == 0 {
            0
        } else {
            gear.position.0 - 1
        };
        let max_row = if gear.position.0 == grid.rows() - 1 {
            gear.position.0
        } else {
            gear.position.0 + 1
        };

        let min_col = if gear.position.1 == 0 {
            0
        } else {
            gear.position.1 - 1
        };
        let max_col = if gear.position.1 == grid.cols() - 1 {
            gear.position.1
        } else {
            gear.position.1 + 1
        };

        gear.part_numbers = adjacent_digits
            .iter()
            .filter(|x| {
                x.iter().any(|d| {
                    (min_row..=max_row).contains(&d.position.0)
                        && (min_col..=max_col).contains(&d.position.1)
                })
            })
            .map(|x| {
                x.iter()
                    .map(|x| x.glyph)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect();
    }

    gears.retain(|g| g.part_numbers.len() == 2);

    let part2_result: u32 = gears
        .iter()
        .map(|g| g.part_numbers.iter().product::<u32>())
        .sum();

    println!("Part 2 Result: {}", part2_result);
}
