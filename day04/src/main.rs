#[derive(Debug)]
struct Card {
    winners: Vec<u32>,
    numbers: Vec<u32>,
    score: u32,
}

impl Card {
    fn new(winners: Vec<u32>, numbers: Vec<u32>) -> Self {
        Card {
            winners,
            numbers,
            score: 0,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let id_and_numbers = line.split(':').collect::<Vec<&str>>();
        let nums: Vec<Vec<u32>> = id_and_numbers
            .get(1)
            .unwrap()
            .split('|')
            .map(|x| {
                x.split_whitespace()
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        cards.push(Card::new(
            nums.first().unwrap().to_vec(),
            nums.last().unwrap().to_vec(),
        ));
    }

    let mut copies: Vec<u32> = vec![1; cards.len()];

    let part2 = true;

    for card in &mut cards.iter_mut().enumerate() {
        for _ in 0..*copies.get(card.0).unwrap() {
            let mut matches: u32 = 0;
            for num in &card.1.numbers {
                if card.1.winners.contains(num) {
                    matches += 1;
                    if !part2 {
                        if card.1.score == 0 {
                            card.1.score = 1;
                        } else {
                            card.1.score *= 2;
                        }
                    }
                }
            }
            for i in 0..matches {
                if let Some(num) = copies.get_mut(card.0 + i as usize + 1) {
                    *num += 1;
                }
            }
        }
    }

    let part1_result: u32 = cards.iter().map(|c| c.score).sum();
    println!("Part 1 Result: {}", part1_result);

    println!("{:#?}", copies);

    let part2_result: u32 = copies.iter().sum();
    println!("Part 2 Result: {}", part2_result);
}
