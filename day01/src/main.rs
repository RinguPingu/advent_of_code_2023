fn main() {
    // let input = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    #[rustfmt::skip]
    let input = ["two1nine","eightwothree","abcone2threexyz","xtwone3four","4nineeightseven2","zoneight234","7pqrstsixteen"];

    let mut strings: Vec<String> = input.iter().map(|s| s.to_string()).collect();

    let digit_replacements = [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ];

    let part2 = true;

    if part2 {
        for string in &mut strings {
            for s in digit_replacements {
                *string = string.replace(s.0, s.1);
            }
        }
    }

    let mut digits: Vec<String> = Vec::new();
    for string in strings {
        digits.push(string.chars().filter(|c| c.is_numeric()).collect());
    }

    let mut numbers: Vec<u32> = Vec::new();

    for number in &mut digits {
        let temp: u32 = match number.len() {
            1 => {
                number.push(number.chars().next().unwrap());
                number.parse().unwrap()
            }
            2 => number.parse().unwrap(),
            _ => {
                let first = number.chars().next().unwrap();
                let last = number.chars().last().unwrap();
                format!("{first}{last}").parse().unwrap()
            }
        };
        numbers.push(temp);
    }

    let output: u32 = numbers.iter().sum();

    println!("Result: {output}");
}
