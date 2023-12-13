use core::panic;

use itertools::Itertools;
use regex::Regex;

fn validate_arrangement(arr: &str, key: Vec<usize>) -> bool {
    let re = Regex::new(r"(#+)").unwrap();
    let captures: Vec<&str> = re.find_iter(arr).map(|s| s.as_str()).collect();

    if captures.len() != key.len() {
        return false;
    }

    let lengths: Vec<usize> = captures.iter().map(|c| c.len()).collect();

    lengths == key
}

fn main() {
    let input = std::fs::read_to_string("./src/input/example_input.txt").expect("Invalid File");

    let spring_rows: Vec<(&str, Vec<usize>)> = input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|v| {
            (
                *v.first().unwrap(),
                v.last()
                    .unwrap()
                    .split(',')
                    .map(|d| d.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .collect();

    for row in spring_rows {
        let mut consecutive_springs: usize = 0;
        for group in row.1.iter().rev() {
            let mut springs = row.0.chars().rev().peekable();
            while let Some(mut spring) = springs.next() {
                match spring {
                    '#' => continue,
                    '?' => {
                        spring = '.';
                        consecutive_springs += 1
                    }
                    '.' => consecutive_springs += 1,
                    other => panic!("Spring was {}", other),
                }

                if consecutive_springs == *group {
                    if let Some(next) = springs.peek_mut() {
                        if *next == '?' {
                            *next = '#';
                        }
                        if *next != '.' {
                            consecutive_springs = 0;
                            continue;
                        }
                    }
                }
            }
        }
    }

    // let unknowns_pattern = Regex::new(r"\?+").unwrap();

    // for row in &spring_rows {
    //     println!("Row: {:?}", row);

    //     let mut replacements: Vec<Vec<Vec<char>>> = Vec::new();

    //     for unknowns in unknowns_pattern.find_iter(row.0).map(|c| c.as_str()) {
    //         let temp: Vec<Vec<char>> = ".##..#"
    //             .chars()
    //             .combinations_with_replacement(unknowns.len())
    //             .sorted()
    //             .dedup()
    //             .collect_vec();

    //         replacements.push(temp);
    //     }

    //     for foo in replacements {
    //         println!("{:?}", foo);
    //     }
    // }
}
