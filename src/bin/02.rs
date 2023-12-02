advent_of_code::solution!(2);

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn check_valid_colour_value(colour_value: i32, colour: &str) -> bool {
    if colour == "red" {
        colour_value <= MAX_RED
    } else if colour == "green" {
        colour_value <= MAX_GREEN
    } else if colour == "blue" {
        colour_value <= MAX_BLUE
    } else {
        false
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        let line_split: Vec<&str> = line.split(':').collect();
        let game_info = line_split[1].trim();
        let sets: Vec<&str> = game_info.split(';').collect();
        let mut invalid_game = false;
        for set in sets {
            let set = set.trim();
            let cube_numbers: Vec<&str> = set.split(',').collect();
            for mut cube_count in cube_numbers {
                cube_count = cube_count.trim();
                let cube_count_split: Vec<&str> = cube_count.split(' ').collect();
                let colour = cube_count_split[1];
                let colour_value = cube_count_split[0].parse::<i32>().unwrap();
                if !check_valid_colour_value(colour_value, colour) {
                    invalid_game = true;
                    break;
                }
            }
            if invalid_game {
                break;
            }
        }
        if !invalid_game {
            sum += (index + 1) as u32;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let sets = collect_sets(line);
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;
        for set in sets {
            let set = set.trim();
            let cube_numbers: Vec<&str> = set.split(',').collect();
            for mut cube_count in cube_numbers {
                cube_count = cube_count.trim();
                let cube_count_split: Vec<&str> = cube_count.split(' ').collect();
                let colour = cube_count_split[1];
                let colour_value: u32 = cube_count_split[0].parse::<u32>().unwrap();
                if colour == "red" && colour_value > min_red {
                    min_red = colour_value;
                } else if colour == "green" && colour_value > min_green {
                    min_green = colour_value;
                } else if colour == "blue" && colour_value > min_blue {
                    min_blue = colour_value;
                }
            }
        }
        let power = min_red * min_green * min_blue;
        sum += power;
    }
    Some(sum)
}

fn collect_sets(line: &str) -> Vec<&str> {
    let line_split: Vec<&str> = line.split(':').collect();
    let game_info = line_split[1].trim();
    let sets: Vec<&str> = game_info.split(';').collect();
    sets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
