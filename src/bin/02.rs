use std::{panic, str::FromStr};

advent_of_code::solution!(2);

enum Dir {
    Up,
    Right,
    Down,
    Left,
}
#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;
impl FromStr for Dir {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().unwrap() {
            'U' => Self::Up,
            'R' => Self::Right,
            'D' => Self::Down,
            'L' => Self::Left,
            _ => return Err(ParsePointError),
        })
    }
}

fn next_button(b: char, d: Dir) -> char {
    match (b, d) {
        ('1', Dir::Up) => '1',
        ('1', Dir::Right) => '2',
        ('1', Dir::Down) => '4',
        ('1', Dir::Left) => '1',
        ('2', Dir::Up) => '2',
        ('2', Dir::Right) => '3',
        ('2', Dir::Down) => '5',
        ('2', Dir::Left) => '1',
        ('3', Dir::Up) => '3',
        ('3', Dir::Right) => '3',
        ('3', Dir::Down) => '6',
        ('3', Dir::Left) => '2',
        ('4', Dir::Up) => '1',
        ('4', Dir::Right) => '5',
        ('4', Dir::Down) => '7',
        ('4', Dir::Left) => '4',
        ('5', Dir::Up) => '2',
        ('5', Dir::Right) => '6',
        ('5', Dir::Down) => '8',
        ('5', Dir::Left) => '4',
        ('6', Dir::Up) => '3',
        ('6', Dir::Right) => '6',
        ('6', Dir::Down) => '9',
        ('6', Dir::Left) => '5',
        ('7', Dir::Up) => '4',
        ('7', Dir::Right) => '8',
        ('7', Dir::Down) => '7',
        ('7', Dir::Left) => '7',
        ('8', Dir::Up) => '5',
        ('8', Dir::Right) => '9',
        ('8', Dir::Down) => '8',
        ('8', Dir::Left) => '7',
        ('9', Dir::Up) => '6',
        ('9', Dir::Right) => '9',
        ('9', Dir::Down) => '9',
        ('9', Dir::Left) => '8',
        _ => panic!(),
    }
}

fn next_button_2(b: char, d: Dir) -> char {
    match (b, d) {
        ('1', Dir::Up) => '1',
        ('1', Dir::Right) => '1',
        ('1', Dir::Down) => '3',
        ('1', Dir::Left) => '1',
        ('2', Dir::Up) => '2',
        ('2', Dir::Right) => '3',
        ('2', Dir::Down) => '6',
        ('2', Dir::Left) => '2',
        ('3', Dir::Up) => '1',
        ('3', Dir::Right) => '4',
        ('3', Dir::Down) => '7',
        ('3', Dir::Left) => '2',
        ('4', Dir::Up) => '4',
        ('4', Dir::Right) => '4',
        ('4', Dir::Down) => '8',
        ('4', Dir::Left) => '3',
        ('5', Dir::Up) => '5',
        ('5', Dir::Right) => '6',
        ('5', Dir::Down) => '5',
        ('5', Dir::Left) => '5',
        ('6', Dir::Up) => '2',
        ('6', Dir::Right) => '7',
        ('6', Dir::Down) => 'A',
        ('6', Dir::Left) => '5',
        ('7', Dir::Up) => '3',
        ('7', Dir::Right) => '8',
        ('7', Dir::Down) => 'B',
        ('7', Dir::Left) => '6',
        ('8', Dir::Up) => '4',
        ('8', Dir::Right) => '9',
        ('8', Dir::Down) => 'C',
        ('8', Dir::Left) => '7',
        ('9', Dir::Up) => '9',
        ('9', Dir::Right) => '9',
        ('9', Dir::Down) => '9',
        ('9', Dir::Left) => '8',
        ('A', Dir::Up) => '6',
        ('A', Dir::Right) => 'B',
        ('A', Dir::Down) => 'A',
        ('A', Dir::Left) => 'A',
        ('B', Dir::Up) => '7',
        ('B', Dir::Right) => 'C',
        ('B', Dir::Down) => 'D',
        ('B', Dir::Left) => 'A',
        ('C', Dir::Up) => '8',
        ('C', Dir::Right) => 'C',
        ('C', Dir::Down) => 'C',
        ('C', Dir::Left) => 'B',
        ('D', Dir::Up) => 'B',
        ('D', Dir::Right) => 'D',
        ('D', Dir::Down) => 'D',
        ('D', Dir::Left) => 'D',
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut res = String::from("");
    let mut current_button = '5';
    for c in input.chars() {
        if let Ok(d) = c.to_string().parse::<Dir>() {
            current_button = next_button(current_button, d);
        } else {
            res.push(current_button);
        }
    }
    res.push(current_button);

    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut res = String::from("");
    let mut current_button = '5';
    for c in input.chars() {
        if let Ok(d) = c.to_string().parse::<Dir>() {
            current_button = next_button_2(current_button, d);
        } else {
            res.push(current_button);
        }
    }
    res.push(current_button);

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("None")));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("None")));
    }
}
