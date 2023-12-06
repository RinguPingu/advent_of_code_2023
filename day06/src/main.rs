fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut lines = input.lines();

    let mut times: Vec<usize> = if let Some(line) = lines.next() {
        line.split(':')
            .flat_map(|x| {
                x.split_whitespace()
                    .filter_map(|y| y.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<usize>>()
    } else {
        Vec::new()
    };
    let mut distances: Vec<usize> = if let Some(line) = lines.next() {
        line.split(':')
            .flat_map(|x| {
                x.split_whitespace()
                    .filter_map(|y| y.parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<usize>>()
    } else {
        Vec::new()
    };

    let part1 = false;

    if !part1 {
        times = vec![times
            .iter()
            .map(|x| x.to_string())
            .reduce(|a, b| a + &b)
            .unwrap()
            .parse::<usize>()
            .unwrap()];
        distances = vec![distances
            .iter()
            .map(|x| x.to_string())
            .reduce(|a, b| a + &b)
            .unwrap()
            .parse::<usize>()
            .unwrap()];
    }

    let races: Vec<(&usize, &usize)> = times.iter().zip(distances.iter()).collect();

    let mut race_solutions: Vec<Vec<(usize, usize)>> = Vec::new();

    for race in &races {
        let mut solutions: Vec<(usize, usize)> = Vec::new();

        for ms in 0..=*race.0 {
            let distance = ms * (race.0 - ms);
            if distance > *race.1 {
                solutions.push((ms, distance));
            }
        }
        race_solutions.push(solutions);
    }

    let result: usize = race_solutions.iter().map(|x| x.len()).product();
    println!("Result: {}", result);
}
