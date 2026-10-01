advent_of_code::solution!(3);

fn is_triangle(a: usize, b: usize, c: usize) -> bool {
    a + b > c && b + c > a && a + c > b
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            (
                it.next().unwrap().parse::<usize>().unwrap(),
                it.next().unwrap().parse::<usize>().unwrap(),
                it.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .filter(|x| is_triangle(x.0, x.1, x.2))
        .count();

    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|l| {
            let mut it = l.split_whitespace();
            (
                it.next().unwrap().parse::<usize>().unwrap(),
                it.next().unwrap().parse::<usize>().unwrap(),
                it.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let res = res
        .chunks(3)
        .flat_map(|x| {
            [
                (x[0].0, x[1].0, x[2].0),
                (x[0].1, x[1].1, x[2].1),
                (x[0].2, x[1].2, x[2].2),
            ]
        })
        .filter(|x| is_triangle(x.0, x.1, x.2))
        .count();

    Some(res as u64)
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
