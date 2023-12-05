#[derive(Debug)]
struct AlmanacRange {
    destination_index: usize,
    source_index: usize,
    range_length: usize,
}

impl AlmanacRange {
    fn new(destination_index: usize, source_index: usize, range_length: usize) -> Self {
        AlmanacRange {
            destination_index,
            source_index,
            range_length,
        }
    }
}
fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut lines = input.lines();

    let part2 = true;

    let mut seeds: Vec<Vec<usize>> = if !part2 {
        lines
            .next()
            .unwrap()
            .split(':')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|x| vec![x.parse::<usize>().unwrap()])
            .collect()
    } else {
        lines
            .next()
            .unwrap()
            .split(':')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        
            .chunks(2)
            .map(|c| (*c.first().unwrap()..*c.last().unwrap()).collect::<Vec<usize>>())
            .collect()
    };

    for seed in &seeds {
        println!("{:?}", seed);
    }

    lines.next();

    let mut maps: Vec<Vec<&str>> = Vec::new();

    let mut temp: Vec<&str> = Vec::new();
    for line in lines {
        if !line.is_empty() {
            temp.push(line);
        } else {
            maps.push(temp.clone());
            temp.clear();
        }
    }
    maps.push(temp);

    let almanac: Vec<Vec<AlmanacRange>> = maps
        .iter()
        .map(|x| {
            x.iter()
                .skip(1)
                .map(|s| {
                    s.split_whitespace()
                        .map(|d| d.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .map(|v| {
                    AlmanacRange::new(*v.get(0).unwrap(), *v.get(1).unwrap(), *v.get(2).unwrap())
                })
                .collect::<Vec<AlmanacRange>>()
        })
        .collect();

    for seed in &mut seeds {
        for map in &almanac {
            if let Some(x) = map.iter().find(|r| {
                (r.source_index..r.source_index + r.range_length).contains(seed.last().unwrap())
            }) {
                let number = if x.source_index < x.destination_index {
                    seed.last().unwrap() + usize::abs_diff(x.destination_index, x.source_index)
                } else {
                    seed.last().unwrap() - usize::abs_diff(x.destination_index, x.source_index)
                };
                seed.push(number);
            } else {
                seed.push(*seed.last().unwrap());
            }
        }
    }

    let part1_result = seeds.iter().map(|s| s.last().unwrap()).min().unwrap();

    println!("Part 1 Result: {}", part1_result);
}
