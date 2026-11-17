use md5::{Digest, Md5};
use std::fmt::Write;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<String> {
    let mut base = Md5::new();
    base.update(input.trim().as_bytes());

    let mut res = String::new();
    let mut buf = String::with_capacity(20);
    let mut i: u64 = 0;

    while res.len() < 8 {
        buf.clear();
        write!(buf, "{i}").unwrap();

        let mut hasher = base.clone();
        hasher.update(buf.as_bytes());
        let hash = hasher.finalize();

        // 5 leading hex zeros = first 20 bits are zero
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 0x10 {
            res.push(char::from_digit((hash[2] & 0x0f) as u32, 16).unwrap());
        }
        i += 1;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut base = Md5::new();
    base.update(input.trim().as_bytes());

    let mut res: [Option<char>; 8] = [None; 8];
    let mut buf = String::with_capacity(20);
    let mut i: u64 = 0;

    while res.iter().any(|x| x.is_none()) {
        buf.clear();
        write!(buf, "{i}").unwrap();

        let mut hasher = base.clone();
        hasher.update(buf.as_bytes());
        let hash = hasher.finalize();

        // 5 leading hex zeros = first 20 bits are zero
        // 6th hex < 8
        if hash[0] == 0 && hash[1] == 0 && hash[2] < 0x08 {
            let idx = (hash[2] & 0x0f) as usize;

            let val = char::from_digit((hash[3] >> 4) as u32, 16).unwrap();
            if res[idx].is_none() {
                res[idx] = Some(val);
            }
        }
        i += 1;
    }
    let res = res.iter().map(|x| x.unwrap()).collect::<String>();
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
