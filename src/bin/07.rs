use std::collections::HashSet;

advent_of_code::solution!(7);

fn is_abba(s: &str) -> bool {
    let mut inside = false;
    let mut found = false;
    for w in s.as_bytes().windows(4) {
        if w[0] == b'[' {
            inside = true;
            continue;
        }
        if w[0] == b']' {
            inside = false;
            continue;
        }
        if w[0] == w[3] && w[1] == w[2] && w[0] != w[1] {
            if inside {
                return false;
            }
            found = true;
        }
    }
    found
}

fn is_ss1l(s: &str) -> bool {
    let mut inside = false;
    let mut a: HashSet<&[u8]> = HashSet::new();
    let mut b: HashSet<&[u8]> = HashSet::new();
    for w in s.as_bytes().windows(3) {
        if w[0] == b'[' {
            inside = true;
            continue;
        }
        if w[0] == b']' {
            inside = false;
            continue;
        }

        if w[0] == w[2] && w[0] != w[1] {
            if inside {
                b.insert(w);
            } else {
                a.insert(w);
            }
        }
    }
    a.iter().any(|x| b.contains(&[x[1], x[0], x[1]].as_slice()))
}

pub fn part_one(input: &str) -> Option<u64> {
    let n = input.lines().filter(|l| is_abba(l));
    Some(n.count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let n = input.lines().filter(|l| is_ss1l(l));
    Some(n.count() as u64)
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
