use grid::Grid;

static PART_1: bool = false;
static OFFSET: usize = 1000000;

struct Galaxy {
    position: (usize, usize),
}

impl Galaxy {
    fn new(position: (usize, usize)) -> Self {
        Galaxy { position }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let grid: Grid<char> = Grid::from_vec(
        input.chars().filter(|c| !c.is_whitespace()).collect(),
        input.lines().count(),
    );

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();

    for row in 0..grid.rows() {
        if grid.iter_row(row).all(|c| *c == '.') {
            empty_rows.push(row);
        }
    }

    for col in 0..grid.cols() {
        if grid.iter_col(col).all(|c| *c == '.') {
            empty_cols.push(col);
        }
    }

    let galaxies: Vec<Galaxy> = grid
        .indexed_iter()
        .filter(|c| *c.1 == '#')
        .map(|g| Galaxy::new(g.0))
        .collect();

    let mut distances = Vec::new();
    for a in 0..galaxies.len() {
        for b in a + 1..galaxies.len() {
            let mut pos = galaxies[a].position;
            let end = galaxies[b].position;
            let mut steps = 0;
            while pos != end {
                match pos.0.cmp(&end.0) {
                    std::cmp::Ordering::Less => {
                        if empty_rows.contains(&pos.0) {
                            if PART_1 {
                                steps += 2;
                            } else {
                                steps += OFFSET;
                            }
                        } else {
                            steps += 1;
                        }
                        pos.0 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        if empty_rows.contains(&pos.0) {
                            if PART_1 {
                                steps += 2;
                            } else {
                                steps += OFFSET;
                            }
                        } else {
                            steps += 1;
                        }
                        pos.0 -= 1;
                    }
                    std::cmp::Ordering::Equal => (),
                }

                match pos.1.cmp(&end.1) {
                    std::cmp::Ordering::Less => {
                        if empty_cols.contains(&pos.1) {
                            if PART_1 {
                                steps += 2;
                            } else {
                                steps += OFFSET;
                            }
                        } else {
                            steps += 1;
                        }
                        pos.1 += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        if empty_cols.contains(&pos.1) {
                            if PART_1 {
                                steps += 2;
                            } else {
                                steps += OFFSET;
                            }
                        } else {
                            steps += 1;
                        }
                        pos.1 -= 1;
                    }
                    std::cmp::Ordering::Equal => (),
                }
            }
            distances.push((vec![galaxies[a].position, galaxies[b].position], steps));
        }
    }

    distances.dedup_by(|a, b| {
        a.0.sort();
        b.0.sort();
        a.0 == b.0
    });

    let result: usize = distances.iter().map(|d| d.1).sum();
    println!("Result: {}", result);
}
