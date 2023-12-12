advent_of_code::solution!(11);

use pathfinding::prelude::bfs;

#[cfg(not(test))]
const LARGER_EXPANSION: usize = 1000000;

#[cfg(test)]
const LARGER_EXPANSION: usize = 100;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self) -> Vec<Pos> {
        let &Pos(x, y) = self;
        vec![Pos(x + 1, y), Pos(x, y + 1), Pos(x - 1, y), Pos(x, y - 1)]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let expanded_map = expand_universe(input_lines, 2);
    let galaxy_locations = get_positions(expanded_map);
    let sum = calculate_distances(&galaxy_locations);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_lines: Vec<&str> = input.lines().collect();
    let expanded_map = expand_universe(input_lines, LARGER_EXPANSION);
    println!(
        "Expanded map size: {}x{}",
        expanded_map.len(),
        expanded_map[0].len()
    );
    let galaxy_locations = get_positions(expanded_map);
    let sum = calculate_distances(&galaxy_locations);
    Some(sum)
}

fn calculate_distances(galaxy_locations: &Vec<Pos>) -> u32 {
    let mut sum = 0;
    for i in 0..galaxy_locations.len() {
        for j in i + 1..galaxy_locations.len() {
            let result = bfs(
                &galaxy_locations[i],
                |p| p.successors(),
                |p| *p == galaxy_locations[j],
            );
            if result.is_some() {
                sum += result.unwrap().len() as u32 - 1;
            }
        }
    }
    sum
}

fn get_positions(expanded_map: Vec<Vec<char>>) -> Vec<Pos> {
    let mut galaxy_locations: Vec<Pos> = Vec::new();
    for i in 0..expanded_map.len() {
        for j in 0..expanded_map[0].len() {
            if expanded_map[i][j] == '#' {
                let pos = Pos(i as i32, j as i32);
                galaxy_locations.push(pos);
            }
        }
    }
    galaxy_locations
}

fn expand_universe(input_lines: Vec<&str>, extra_amount: usize) -> Vec<Vec<char>> {
    let mut expanded_map: Vec<Vec<char>> = Vec::new();
    for line in input_lines {
        if !line.contains('#') {
            for _ in 0..extra_amount {
                expanded_map.push(vec!['.'; line.len()]);
            }
        } else {
            expanded_map.push(line.chars().collect());
        }
    }
    let mut empty_columns: Vec<usize> = Vec::new();
    for col in 0..expanded_map[0].len() - 1 {
        let mut column_empty = true;
        for row in expanded_map.iter() {
            if row[col] == '#' {
                column_empty = false;
                break;
            }
        }
        if column_empty {
            empty_columns.push(col);
        }
    }

    for i in 0..empty_columns.len() {
        // Increment the positions of the other empty columns
        for empty_column in empty_columns.iter_mut().skip(i + 1) {
            *empty_column += extra_amount - 1;
        }
        for row in expanded_map.iter_mut() {
            for _ in 0..extra_amount - 1 {
                row.insert(empty_columns[i], '.');
            }
        }
    }

    expanded_map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
