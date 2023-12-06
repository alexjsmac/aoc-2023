advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let times = get_times_or_distances(&input_lines[0]);
    let distances = get_times_or_distances(&input_lines[1]);
    let mut ways_to_win_vec: Vec<u32> = Vec::new();
    for i in 0..times.len() {
        let mut ways_to_win = 0;
        for speed in 0..times[i] {
            let distance = speed * (times[i] - speed);
            if distance > distances[i] {
                ways_to_win += 1;
            }
        }
        ways_to_win_vec.push(ways_to_win);
    }
    let sum = ways_to_win_vec.iter().product();
    Some(sum)
}

fn get_times_or_distances(input_line: &&str) -> Vec<u32> {
    let mut num_strs: Vec<&str> = input_line.split(' ').collect::<Vec<&str>>();
    num_strs.retain(|&x| !x.is_empty() && x != "Time:" && x != "Distance:");
    let nums: Vec<u32> = num_strs
        .iter()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    nums
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let mut num_strs: Vec<&str> = input_lines[0].split(' ').collect::<Vec<&str>>();
    num_strs.retain(|&x| !x.is_empty() && x != "Time:" && x != "Distance:");
    let num_strs = num_strs.join("");
    let time = num_strs.parse::<u64>().unwrap();
    let mut dist_strs = input_lines[1].split(' ').collect::<Vec<&str>>();
    dist_strs.retain(|&x| !x.is_empty() && x != "Time:" && x != "Distance:");
    let dist_strs = dist_strs.join("");
    let distance = dist_strs.parse::<u64>().unwrap();
    let mut ways_to_win = 0;
    for speed in 0..time {
        let possible_distance = speed * (time - speed);
        if possible_distance > distance {
            ways_to_win += 1;
        }
    }
    Some(ways_to_win)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
