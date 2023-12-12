use grid::Grid;

static PART_1: bool = true;
struct Galaxy {
    position: (usize, usize),
}

impl Galaxy {
    fn new(position: (usize, usize)) -> Self {
        Galaxy { position }
    }
}

fn determine_distance(start: (usize, usize), end: (usize, usize)) -> usize {
    let mut pos = start;
    let mut steps = 0;
    while pos != end {
        while pos.0 != end.0 {
            match pos.0.cmp(&end.0) {
                std::cmp::Ordering::Less => pos.0 += 1,
                std::cmp::Ordering::Greater => pos.0 -= 1,
                std::cmp::Ordering::Equal => (),
            }
            steps += 1;
        }
        while pos.1 != end.1 {
            match pos.1.cmp(&end.1) {
                std::cmp::Ordering::Less => pos.1 += 1,
                std::cmp::Ordering::Greater => pos.1 -= 1,
                std::cmp::Ordering::Equal => (),
            }
            steps += 1;
        }
    }
    steps
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut grid: Grid<char> = Grid::from_vec(
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

    if PART_1 {
        for row in empty_rows.iter().enumerate() {
            grid.insert_row(row.1 + row.0, vec!['.'; grid.cols()]);
        }
        for col in empty_cols.iter().enumerate() {
            grid.insert_col(col.1 + col.0, vec!['.'; grid.rows()]);
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
            distances.push((
                vec![galaxies[a].position, galaxies[b].position],
                determine_distance(galaxies[a].position, galaxies[b].position),
            ));
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
