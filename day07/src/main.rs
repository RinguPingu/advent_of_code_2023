static PART_1: bool = false;

#[derive(Debug, PartialOrd, PartialEq, Eq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq)]
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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type > other.hand_type {
            std::cmp::Ordering::Less
        } else if self.hand_type < other.hand_type {
            std::cmp::Ordering::Greater
        } else if self.hand_type == other.hand_type {
            for card_pair in self.cards.chars().zip(other.cards.chars()) {
                let self_value: u32 = determine_card_value(card_pair.0);
                let other_value: u32 = determine_card_value(card_pair.1);

                match self_value.cmp(&other_value) {
                    std::cmp::Ordering::Less => {
                        return std::cmp::Ordering::Less;
                    }
                    std::cmp::Ordering::Greater => {
                        return std::cmp::Ordering::Greater;
                    }
                    std::cmp::Ordering::Equal => continue,
                }
            }
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

fn determine_hand_type(hand: &Hand) -> HandType {
    let mut card_counts: Vec<(char, usize)> = Vec::new();

    for card in hand.cards.chars() {
        if let Some(x) = card_counts.iter_mut().find(|cc| cc.0 == card) {
            x.1 += 1;
        } else {
            card_counts.push((card, 1));
        }
    }

    card_counts.sort_by_key(|cc| cc.1);

    let j_count = if PART_1 {
        0
    } else {
        hand.cards.chars().filter(|c| *c == 'J').count()
    };

    let highest_count: &mut (char, usize) =
        if card_counts.iter().filter(|cc| cc.1 == j_count).count() > 1 {
            card_counts
                .iter_mut()
                .filter(|cc| cc.0 != 'J')
                .max_by_key(|x| x.1)
                .unwrap()
        } else {
            card_counts.iter_mut().max_by_key(|x| x.1).unwrap()
        };

    let hand_type = match highest_count.1 {
        5 => HandType::FiveKind,
        4 => HandType::FourKind,
        3 => {
            if card_counts.iter().any(|y| y.1 == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeKind
            }
        }
        2 => {
            if card_counts.iter().filter(|y| y.1 == 2).count() == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        1 => HandType::HighCard,
        other => panic!("Invalid number {} of the same card!", other),
    };

    if PART_1 || j_count == 0 {
        hand_type
    } else {
        match hand_type {
            HandType::HighCard => HandType::OnePair,
            HandType::OnePair => HandType::ThreeKind,
            HandType::TwoPair => {
                if j_count == 1 {
                    HandType::FullHouse
                } else {
                    HandType::FourKind
                }
            }
            HandType::ThreeKind => HandType::FourKind,
            HandType::FullHouse => HandType::FiveKind,
            HandType::FourKind => HandType::FiveKind,
            HandType::FiveKind => HandType::FiveKind,
        }
    }
}

fn determine_card_value(card: char) -> u32 {
    if card.is_numeric() {
        card.to_digit(10).unwrap()
    } else {
        match card {
            'T' => 10,
            'J' => {
                if PART_1 {
                    11
                } else {
                    1
                }
            }
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Card has invalid character"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("./src/input/input.txt").expect("Invalid File");

    let mut hands: Vec<Hand> = input
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|v| {
            Hand::new(
                v.first().unwrap().to_string(),
                v.last().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect();

    for hand in &mut hands {
        hand.hand_type = Some(determine_hand_type(hand));
    }

    hands.sort();

    let hands_ranks: Vec<(&Hand, usize)> = hands
        .iter()
        .map(|h| (h, hands.iter().filter(|x| *x < h).count() + 1))
        .collect();

    let winnings: usize = hands_ranks.iter().map(|hr| hr.0.bet as usize * hr.1).sum();

    for hand in hands_ranks
        .iter()
        .filter(|h| h.0.cards.chars().any(|c| c == 'J'))
    {
        println!("{:?}", hand);
    }

    println!("{}", winnings);
}
