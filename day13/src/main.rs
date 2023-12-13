use grid::Grid;

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut patterns: Vec<Grid<char>> = Vec::new();
    let cols = input.lines().next().unwrap().len();

    let mut temp: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Grid::from_vec(
                temp.iter().flat_map(|s| s.chars()).collect(),
                cols,
            ));
            temp.clear();
            continue;
        }

        temp.push(line);
    }
    patterns.push(Grid::from_vec(
        temp.iter().flat_map(|s| s.chars()).collect(),
        cols,
    ));

    for pat in &patterns {
        println!("{:#?}", pat);
    }

    let mut vertical: Vec<usize> = Vec::new();
    let mut horizontal: Vec<usize> = Vec::new();

    for pattern in patterns.iter() {
        for col in 0..pattern.cols() {
            if col == pattern.cols() - 1 {
                continue;
            }
            let a = pattern.iter_col(col).cloned().collect::<Vec<char>>();
            let b = pattern.iter_col(col + 1).cloned().collect::<Vec<char>>();

            if a == b {
                let mut x = col;
                let mut y = col + 1;

                let mut reflection = true;

                while x > 0 && y < pattern.cols() - 1 {
                    x -= 1;
                    y += 1;

                    let x_col = pattern.iter_col(x).cloned().collect::<Vec<char>>();
                    let y_col = pattern.iter_col(y).cloned().collect::<Vec<char>>();

                    if x_col != y_col {
                        reflection = false;
                        break;
                    }
                }

                if reflection {
                    vertical.push(col + 1);
                }
            }
        }
        for row in 0..pattern.rows() {
            if row == pattern.rows() - 1 {
                continue;
            }
            let a = pattern.iter_row(row).cloned().collect::<Vec<char>>();
            let b = pattern.iter_row(row + 1).cloned().collect::<Vec<char>>();

            if a == b {
                let mut x = row;
                let mut y = row + 1;

                let mut reflection = true;

                while x > 0 && y < pattern.rows() - 1 {
                    x -= 1;
                    y += 1;

                    let x_row = pattern.iter_row(x).cloned().collect::<Vec<char>>();
                    let y_row = pattern.iter_row(y).cloned().collect::<Vec<char>>();

                    if x_row != y_row {
                        reflection = false;
                        break;
                    }
                }

                if reflection {
                    horizontal.push(row + 1);
                }
            }
        }
    }

    let vertical_sum: usize = vertical.iter().sum();
    let horizontal_sum: usize = 100 * horizontal.iter().sum::<usize>();

    println!("Result: {}", vertical_sum + horizontal_sum);
}
