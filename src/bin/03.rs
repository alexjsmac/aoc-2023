advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (line_idx, line) in lines.iter().enumerate() {
        let mut char_idx = 0;
        while char_idx < line.len() {
            let c = line.chars().nth(char_idx).unwrap();
            if c.is_numeric() {
                let (start_idx, number_length, number_digits) = build_number(line, char_idx, c);
                char_idx += number_length as usize;
                if check_for_symbol(line_idx, start_idx, number_length, &lines) {
                    sum += number_digits.parse::<u32>().unwrap();
                }
            }
            char_idx += 1;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut numbers: Vec<PartNumber> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for (line_idx, line) in lines.iter().enumerate() {
        let mut char_idx = 0;
        while char_idx < line.len() {
            let c = line.chars().nth(char_idx).unwrap();
            if c.is_numeric() {
                let (start_idx, number_length, number_digits) = build_number(line, char_idx, c);
                let mut positions = vec![start_idx as u32];
                for i in 1..number_length {
                    positions.push((start_idx + i as usize) as u32);
                }
                let part_number = PartNumber::new(
                    number_digits.parse::<u32>().unwrap(),
                    line_idx as u32,
                    positions,
                );
                numbers.push(part_number);
                char_idx += number_length as usize;
            }
            char_idx += 1;
        }
    }
    for (line_idx, line) in lines.iter().enumerate() {
        for (char_idx, c) in line.chars().enumerate() {
            if c == '*' {
                let mut adjacent_numbers =
                    find_adjacent_numbers(char_idx, line_idx, line.len(), &numbers);
                if adjacent_numbers.len() > 1 {
                    let mut adjacent_numbers_product = adjacent_numbers.pop().unwrap().value;
                    for adjacent_number in adjacent_numbers {
                        adjacent_numbers_product *= adjacent_number.value;
                    }
                    sum += adjacent_numbers_product;
                }
            }
        }
    }
    Some(sum)
}

struct PartNumber {
    value: u32,
    line: u32,
    positions: Vec<u32>,
}

impl PartNumber {
    fn new(value: u32, line: u32, positions: Vec<u32>) -> Self {
        Self {
            value,
            line,
            positions,
        }
    }
}

fn build_number(line: &&str, mut char_idx: usize, c: char) -> (usize, i32, String) {
    let start_idx = char_idx;
    let mut number_length = 1;
    let mut number_digits = c.to_string();
    char_idx += 1;
    let mut finding_next_number = true;
    while finding_next_number {
        if let Some(next_c) = line.chars().nth(char_idx) {
            if next_c.is_numeric() {
                number_length += 1;
                number_digits.push(next_c);
                char_idx += 1;
            } else {
                finding_next_number = false;
            }
        } else {
            finding_next_number = false;
        }
    }
    (start_idx, number_length, number_digits)
}

fn check_for_symbol(line_idx: usize, char_idx: usize, number_length: i32, lines: &[&str]) -> bool {
    // Get the line above
    if line_idx > 0 {
        let line_above = lines[line_idx - 1];
        if let Some(value) = check_characters(char_idx, number_length, line_above) {
            return value;
        }
    }
    // Get the line below
    if line_idx < lines.len() - 1 {
        let line_below = lines[line_idx + 1];
        if let Some(value) = check_characters(char_idx, number_length, line_below) {
            return value;
        }
    }
    // Check the characters on the same line
    let line = lines[line_idx];
    if char_idx > 0 {
        let c_before = line.chars().nth(char_idx - 1);
        if let Some(c_before) = c_before {
            if !c_before.is_numeric() && c_before != '.' {
                return true;
            }
        }
    }
    if (char_idx + number_length as usize) < line.len() {
        let c_after = line.chars().nth(char_idx + number_length as usize);
        if let Some(c_after) = c_after {
            if !c_after.is_numeric() && c_after != '.' {
                return true;
            }
        }
    }
    false
}

fn check_characters(char_idx: usize, number_length: i32, line_below: &str) -> Option<bool> {
    let mut start_char_idx = char_idx;
    start_char_idx = start_char_idx.saturating_sub(1);
    let mut end_char_idx = char_idx + number_length as usize;
    if (char_idx + number_length as usize) < line_below.len() - 1 {
        end_char_idx += 1;
    }
    for i in 0..end_char_idx - start_char_idx {
        let c = line_below.chars().nth(start_char_idx + i);
        if let Some(c) = c {
            // If the character is not a number or a period return true
            if !c.is_numeric() && c != '.' {
                return Some(true);
            }
        }
    }
    None
}

fn find_adjacent_numbers(
    char_idx: usize,
    line_idx: usize,
    line_length: usize,
    part_numbers: &Vec<PartNumber>,
) -> Vec<&PartNumber> {
    let mut adjacent_numbers: Vec<&PartNumber> = Vec::new();
    for part_number in part_numbers {
        if part_number.line > line_idx as u32
            && part_number.positions[0] as i32 - char_idx as i32 > 1
        {
            break;
        }
        if part_number.line == line_idx as u32 - 1
            || part_number.line == line_idx as u32
            || part_number.line == line_idx as u32 + 1
        {
            check_for_nearby_number(char_idx, line_length, &mut adjacent_numbers, part_number);
        }
    }
    adjacent_numbers
}

fn check_for_nearby_number<'a>(
    char_idx: usize,
    line_length: usize,
    adjacent_numbers: &mut Vec<&'a PartNumber>,
    part_number: &'a PartNumber,
) {
    let start_char_idx = char_idx.saturating_sub(1);
    let mut end_char_idx = char_idx;
    if char_idx < line_length - 1 {
        end_char_idx += 1;
    }
    for i in start_char_idx..=end_char_idx {
        if part_number.positions.contains(&(i as u32)) {
            adjacent_numbers.push(part_number);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
