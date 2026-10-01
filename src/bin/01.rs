use std::{collections::HashSet, convert::Infallible, str::FromStr};

use advent_of_code::Vec2;

advent_of_code::solution!(1);

#[derive(Clone, Copy, Default)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

enum Turn {
    L,
    R,
}

impl FromStr for Turn {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.chars().next().unwrap() {
            'L' => Self::L,
            'R' => Self::R,
            _ => panic!(),
        })
    }
}

#[derive(Default)]
struct Player {
    pos: Vec2,
    direction: Direction,
}

impl Player {
    fn get_dir(dir: Direction, t: Turn) -> (Direction, Vec2) {
        match (dir, t) {
            (Direction::N, Turn::L) | (Direction::S, Turn::R) => {
                (Direction::W, Vec2 { x: -1, y: 0 })
            }
            (Direction::N, Turn::R) | (Direction::S, Turn::L) => {
                (Direction::E, Vec2 { x: 1, y: 0 })
            }
            (Direction::E, Turn::L) | (Direction::W, Turn::R) => {
                (Direction::N, Vec2 { x: 0, y: 1 })
            }
            (Direction::E, Turn::R) | (Direction::W, Turn::L) => {
                (Direction::S, Vec2 { x: 0, y: -1 })
            }
        }
    }

    fn moove(&mut self, t: Turn, distance: isize) {
        let a = Player::get_dir(self.direction, t);
        self.direction = a.0;
        self.pos.x += a.1.x * distance;
        self.pos.y += a.1.y * distance;
    }

    fn move_with_history(&mut self, t: Turn, distance: isize, visited: &mut HashSet<Vec2>) -> bool {
        let a = Player::get_dir(self.direction, t);
        self.direction = a.0;

        for _ in 0..distance {
            self.pos.x += a.1.x;
            self.pos.y += a.1.y;
            if !visited.insert(self.pos) {
                return false;
            }
        }
        true
    }
}

fn init(input: &str) -> (Player, impl Iterator<Item = (Turn, isize)>) {
    let player = Player::default();

    let list = input.trim().split(", ").map(|x| {
        let c = x.split_at(1);
        (c.0.parse::<Turn>().unwrap(), c.1.parse::<isize>().unwrap())
    });
    (player, list)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut player, list) = init(input);

    for instr in list {
        player.moove(instr.0, instr.1);
    }

    Some((player.pos.x + player.pos.y) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut player, list) = init(input);

    let mut visited = HashSet::<Vec2>::new();

    for instr in list {
        if !player.move_with_history(instr.0, instr.1, &mut visited) {
            break;
        }
    }
    Some((player.pos.x + player.pos.y) as u64)
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
