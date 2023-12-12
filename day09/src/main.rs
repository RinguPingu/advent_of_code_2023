fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut histories: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|v| vec![v])
        .collect();

    for history in &mut histories {
        loop {
            let mut temp: Vec<i32> = Vec::new();
            {
                let mut sequence = history.last().unwrap().iter().peekable();

                while let Some(x) = sequence.next() {
                    if let Some(y) = sequence.peek() {
                        temp.push(*y - x);
                    }
                }
            }

            if temp.iter().all(|i| *i == 0) {
                history.push(temp);
                break;
            } else {
                history.push(temp);
            }
        }
    }

    for history in &mut histories {
        let mut end_diff: i32 = 0;
        let mut begin_diff: i32 = 0;
        for sequence in history.iter_mut().rev() {
            sequence.push(sequence.last().unwrap() + end_diff);
            end_diff = *sequence.last().unwrap();

            sequence.insert(0, sequence.first().unwrap() - begin_diff);
            begin_diff = *sequence.first().unwrap();
        }
    }

    let part1_result: i32 = histories
        .iter()
        .map(|h| h.first().unwrap())
        .map(|v| v.last().unwrap())
        .sum();

    println!("Part 1 Result: {}", part1_result);

    let part2_result: i32 = histories
        .iter()
        .map(|h| h.first().unwrap())
        .map(|v| v.first().unwrap())
        .sum();

    println!("Part 2 Result: {}", part2_result);
}
