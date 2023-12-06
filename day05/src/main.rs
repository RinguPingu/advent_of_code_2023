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

    let mut seeds: Vec<u32> = Vec::new();
    if !part2 {
        seeds = lines
            .next()
            .unwrap()
            .split(':')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split_whitespace()
            .flat_map(|x| vec![x.parse::<u32>().unwrap()])
            .collect()
    } else {
        let chunks: Vec<u32> = lines
            .next()
            .unwrap()
            .split(':')
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for chunk in chunks.chunks(2) {
            let first = chunk.first().unwrap();
            let last = first + chunk.last().unwrap();

            for seed in *first..last {
                seeds.push(seed);
            }
        }
    };

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
                (r.source_index..r.source_index + r.range_length).contains(&(*seed as usize))
            }) {
                let number = if x.source_index < x.destination_index {
                    *seed + u32::abs_diff(x.destination_index as u32, x.source_index as u32)
                } else {
                    *seed - u32::abs_diff(x.destination_index as u32, x.source_index as u32)
                };
                *seed = number;
            }
        }
    }

    let result = seeds.iter().min().unwrap();

    println!("Result: {}", result);
}
