use std::collections::HashMap;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut res = String::new();
    for i in 0..lines[0].len() {
        let mut column: HashMap<char, usize> = HashMap::new();
        for line in &lines {
            let c = line.chars().nth(i)?;
            *column.entry(c).or_insert(0) += 1;
        }
        res.push(*column.iter().max_by_key(|(_, v)| *v)?.0);
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut res = String::new();
    for i in 0..lines[0].len() {
        let mut column: HashMap<char, usize> = HashMap::new();
        for line in &lines {
            let c = line.chars().nth(i)?;
            *column.entry(c).or_insert(0) += 1;
        }
        res.push(*column.iter().min_by_key(|(_, v)| *v)?.0);
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
