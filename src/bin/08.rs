advent_of_code::solution!(8);

use std::collections::HashMap;

fn parse_line(line: &str) -> [String; 3] {
    let mut result = [String::new(), String::new(), String::new()];
    let parts: Vec<&str> = line.split('=').collect();
    result[0] = parts[0].trim().to_string();
    let sub_parts: Vec<&str> = parts[1].split(',').collect();
    for i in 0..sub_parts.len() {
        let part = sub_parts[i].replace(['(', ')'], "");
        let trimmed = part.trim();
        result[i + 1] = trimmed.to_string();
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let instructions = input_lines[0];

    let mut map = HashMap::new();
    for line in input_lines[2..].iter() {
        let [from, left, right] = parse_line(line);
        map.insert(from, (left, right));
    }

    let mut count = 0;
    let mut current = "AAA";
    for direction in instructions.chars().cycle() {
        if let Some((left, right)) = map.get(current) {
            if &direction.to_string() == "R" {
                current = right;
            } else if &direction.to_string() == "L" {
                current = left;
            }
            count += 1;
            if current == "ZZZ" {
                break;
            }
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
