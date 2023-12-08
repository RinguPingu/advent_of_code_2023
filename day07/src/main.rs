#[derive(Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bet: u32,
    hand_type: Option<HandType>,
}

impl Hand {
    fn new(cards: String, bet: u32) -> Self {
        Hand {
            cards,
            bet,
            hand_type: None,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/example_input.txt").expect("Invalid File");

    let hands: Vec<Hand> = input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|v| {
            Hand::new(
                v.first().unwrap().to_string(),
                v.last().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    for hand in hands {
        println!("{:?}", hand);
    }
}
