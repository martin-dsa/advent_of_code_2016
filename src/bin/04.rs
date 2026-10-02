use std::{collections::HashMap, convert::Infallible, str::FromStr};

advent_of_code::solution!(4);

struct Room {
    name: String,
    id: usize,
    checksum: String,
}

impl FromStr for Room {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (other, checksum) = s.split_once("[").unwrap();

        let (name, id) = other.rsplit_once("-").unwrap();
        let name = name.chars().collect::<String>();
        let id = id.parse::<usize>().unwrap();
        let checksum = checksum.chars().take(5).collect::<String>();
        Ok(Self { name, id, checksum })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let id_sum = input
        .lines()
        .map(|l| l.parse::<Room>().unwrap())
        .filter(|r| {
            let mut map = HashMap::<char, usize>::new();
            for c in r.name.chars().filter(|x| char::is_alphabetic(*x)) {
                *map.entry(c).or_default() += 1;
            }
            let mut a = map.iter().collect::<Vec<_>>();
            a.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            let x = a.iter().take(5).map(|x| x.0).collect::<String>();

            x == r.checksum
        })
        .map(|x| x.id)
        .sum::<usize>();
    Some(id_sum as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let north_pole_id = input
        .lines()
        .map(|l| l.parse::<Room>().unwrap())
        .map(|x| {
            let shift = (x.id % 26) as u8;
            let name = x
                .name
                .chars()
                .map(|c| match c {
                    'a'..='z' => ((c as u8 - b'a' + shift) % 26 + b'a') as char,
                    _ => c,
                })
                .collect();
            Room {
                name,
                id: x.id,
                checksum: x.checksum,
            }
        })
        .find(|x| x.name.starts_with("northpole"))
        .unwrap()
        .id;

    Some(north_pole_id as u64)
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
