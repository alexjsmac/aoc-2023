advent_of_code::solution!(1);

const DIGIT_STRINGS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct Digit {
    value: u32,
    position: u32,
}

impl Digit {
    fn new(value: u32, position: u32) -> Self {
        Self { value, position }
    }
}

fn word_to_digit(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in lines {
        let mut numbers = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c);
            }
        }
        let number_string = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        let number = number_string.parse::<u32>().unwrap();
        sum += number;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in lines {
        let mut numbers = Vec::new();
        for digit_string in DIGIT_STRINGS {
            if line.contains(digit_string) {
                line.match_indices(digit_string).for_each(|(m, _)| {
                    let digit = Digit::new(word_to_digit(digit_string).unwrap(), m as u32);
                    numbers.push(digit);
                });
            }
        }
        for (idx, c) in line.chars().enumerate() {
            if c.is_numeric() {
                numbers.push(Digit::new(c.to_digit(10).unwrap(), idx as u32));
            }
        }
        numbers.sort_by(|a, b| a.position.cmp(&b.position));
        let number_string = format!("{}{}", numbers[0].value, numbers[numbers.len() - 1].value);
        let number = number_string.parse::<u32>().unwrap();
        sum += number;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
