use grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn new(y: usize, x: usize) -> Self {
        Position { y, x }
    }
}

#[derive(Debug)]
struct Node {
    glyph: char,
    position: Option<Position>,
    connections: Vec<Option<Position>>,
}

impl Node {
    fn new(glyph: char) -> Self {
        Node {
            glyph,
            position: None,
            connections: Vec::new(),
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.glyph)
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/example_input.txt").expect("Invalid File");

    let mut grid: Grid<Node> = Grid::from_vec(
        input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Node::new)
            .collect(),
        input.lines().count(),
    );

    let mut connections: Vec<Vec<Option<Position>>> = Vec::new();

    // Set the position of each node (added *after* the stuff below, for convenience)
    for y in 0..grid.rows() {
        for x in 0..grid.cols() {
            grid.get_mut(y, x).unwrap().position = Some(Position::new(y, x));
        }
    }

    // Determine all the connections
    for node in grid.indexed_iter() {
        connections.push(match node.1.glyph {
            '.' => vec![None, None],
            'S' => vec![None, None],
            '|' => vec![
                Some(Position::new(node.0 .0 + 1, node.0 .1)),
                node.0
                     .0
                    .checked_sub(1)
                    .map(|pos| Position::new(pos, node.0 .1)),
            ],
            '-' => vec![
                Some(Position::new(node.0 .0, node.0 .1 + 1)),
                node.0
                     .1
                    .checked_sub(1)
                    .map(|pos| Position::new(node.0 .0, pos)),
            ],
            'L' => vec![
                node.0
                     .0
                    .checked_sub(1)
                    .map(|pos| Position::new(pos, node.0 .1)),
                Some(Position::new(node.0 .0, node.0 .1 + 1)),
            ],
            'J' => vec![
                node.0
                     .0
                    .checked_sub(1)
                    .map(|pos| Position::new(pos, node.0 .1)),
                node.0
                     .1
                    .checked_sub(1)
                    .map(|pos| Position::new(node.0 .0, pos)),
            ],
            '7' => vec![
                Some(Position::new(node.0 .0 + 1, node.0 .1)),
                node.0
                     .1
                    .checked_sub(1)
                    .map(|pos| Position::new(node.0 .0, pos)),
            ],
            'F' => vec![
                Some(Position::new(node.0 .0 + 1, node.0 .1)),
                Some(Position::new(node.0 .0, node.0 .1 + 1)),
            ],
            other => panic!("Invalid Node: {}", other),
        })
    }

    // Now set them separately because the stupid compiler won't let me do it all in one go
    for node in grid.iter_mut().zip(connections.iter().cloned()) {
        node.0.connections = node.1;
    }

    for row in grid.iter_rows() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }

    // Determine the connections of 'S'
    let s_position = grid
        .indexed_iter()
        .find(|n| n.1.glyph == 'S')
        .map(|o| Position::new(o.0 .0, o.0 .1));

    let s_connections = grid
        .iter()
        .filter(|n| n.connections.iter().any(|c| *c == s_position))
        .map(|x| x.position)
        .collect::<Vec<Option<Position>>>();

    let mut current_node = grid.iter_mut().find(|n| n.glyph == 'S').unwrap();

    current_node.connections = s_connections;

    let mut all_steps: Vec<u32> = Vec::new();

    let mut steps: u32 = 1;

    let starting_nodes: Vec<Position> = current_node
        .connections
        .iter()
        .map(|x| x.unwrap())
        .collect();

    let mut prev_position = current_node.position.unwrap();

    for node in starting_nodes.iter().enumerate() {
        println!("NODE: {:?}", node);
        let mut node_steps: Vec<u32> = Vec::new();

        current_node = grid.get_mut(node.1.y, node.1.x).unwrap();

        while current_node.glyph != 'S' {
            node_steps.push(steps);
            println!(
                "{}\t({},{})",
                steps,
                current_node.position.unwrap().y,
                current_node.position.unwrap().x
            );
            steps += 1;
            let next = current_node
                .connections
                .iter()
                .find(|p| p.unwrap() != prev_position)
                .unwrap()
                .unwrap();

            prev_position = current_node.position.unwrap();
            current_node = grid.get_mut(next.y, next.x).unwrap();
        }
        node_steps.push(steps);
        all_steps.push(*node_steps.iter().max().unwrap());
        steps = 1;
    }

    println!("Max Steps: {:?}", all_steps);

    let part1_result = all_steps.iter().max().unwrap();
    println!("Part 1 Result: {}", part1_result / 2);
}
