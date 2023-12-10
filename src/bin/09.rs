advent_of_code::solution!(9);

fn differences(numbers: &[i32]) -> Vec<i32> {
    let mut diffs = Vec::new();
    for i in 0..numbers.len() - 1 {
        diffs.push(numbers[i + 1] - numbers[i]);
    }
    diffs
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in input_lines.iter() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut diffs: Vec<Vec<i32>> = Vec::new();
        diffs.push(differences(&numbers));
        while diffs[diffs.len() - 1].iter().any(|x| *x != 0) {
            diffs.push(differences(&diffs[diffs.len() - 1]));
        }
        let mut value = diffs[0][diffs[0].len() - 1];
        for diff in diffs.iter().skip(1) {
            value += diff[diff.len() - 1];
        }
        sum += numbers[numbers.len() - 1] + value;
    }
    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    for line in input_lines.iter() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut diffs: Vec<Vec<i32>> = Vec::new();
        diffs.push(differences(&numbers));
        while diffs[diffs.len() - 1].iter().any(|x| *x != 0) {
            diffs.push(differences(&diffs[diffs.len() - 1]));
        }
        let mut value = diffs[0][0];
        for (i, diff) in diffs.iter().skip(1).enumerate() {
            if i % 2 == 0 {
                value -= diff[0];
            } else {
                value += diff[0];
            }
        }
        sum += numbers[0] - value;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
