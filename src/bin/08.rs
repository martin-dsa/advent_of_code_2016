use std::{
    convert::Infallible,
    fmt::{self},
    print,
    str::FromStr,
};

advent_of_code::solution!(8);

const BOARD_WIDTH: usize = 50;
const BOARD_HEIGHT: usize = 6;

struct Screen([[bool; BOARD_WIDTH]; BOARD_HEIGHT]);

impl Screen {
    fn new() -> Self {
        Screen([[false; BOARD_WIDTH]; BOARD_HEIGHT])
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::Rect(w, h) => {
                for row in self.0.iter_mut().take(h) {
                    row[..w].fill(true);
                }
            }
            Instruction::RotateRow(y, n) => {
                self.0[y].rotate_right(n % BOARD_WIDTH);
            }
            Instruction::RotateCol(x, n) => {
                let mut col = [false; BOARD_HEIGHT];
                for y in 0..BOARD_HEIGHT {
                    col[y] = self.0[y][x];
                }
                col.rotate_right(n % BOARD_HEIGHT);
                for y in 0..BOARD_HEIGHT {
                    self.0[y][x] = col[y];
                }
            }
        }
    }
    fn how_many_lit(&self) -> usize {
        self.0.iter().flatten().copied().filter(|x| *x).count()
    }
}

impl fmt::Debug for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            let line: String = row.iter().map(|&p| if p { '*' } else { ' ' }).collect();
            write!(f, "{line}")?;
        }
        writeln!(f)?;
        Ok(())
    }
}
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl FromStr for Instruction {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("rect ") {
            let (w, h) = rest.split_once('x').unwrap();
            Ok(Instruction::Rect(w.parse().unwrap(), h.parse().unwrap()))
        } else if let Some(rest) = s.strip_prefix("rotate row y=") {
            let (y, n) = rest.split_once(" by ").unwrap();
            Ok(Instruction::RotateRow(
                y.parse().unwrap(),
                n.parse().unwrap(),
            ))
        } else if let Some(rest) = s.strip_prefix("rotate column x=") {
            let (x, n) = rest.split_once(" by ").unwrap();
            Ok(Instruction::RotateCol(
                x.parse().unwrap(),
                n.parse().unwrap(),
            ))
        } else {
            panic!()
        }
    }
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut screen = Screen::new();

    for instruction in input.lines().map(|l| l.parse::<Instruction>().unwrap()) {
        screen.apply_instruction(&instruction);
    }
    Some(screen.how_many_lit() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut screen = Screen::new();

    for instruction in input.lines().map(|l| l.parse::<Instruction>().unwrap()) {
        screen.apply_instruction(&instruction);
    }
    print!("{:?}", screen);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
