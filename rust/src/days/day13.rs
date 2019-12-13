use num::Complex;
use crate::intcode::{Opcodes,Status,Machine};
use std::collections::HashSet;

enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

// Helper
fn read_input (input: &str) -> Opcodes {
    input
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .split(",")
        .map(|s| s.trim().parse().expect("Cannot parse int"))
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    let mut instruction = (None, None, None);
    let mut blocks: HashSet<Complex<i64>> = HashSet::new();
    loop {
        match machine.step() {
            Status::Output(o) => {
                if instruction.0.is_none() {
                    instruction.0 = Some(o);
                } else if instruction.1.is_none() {
                    instruction.1 = Some(o);
                } else {
                    instruction.2 = Some(match o {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        tile => panic!("Unknown tile: {}", tile)
                    });
                    match instruction.2 {
                        Some(Tile::Block) => { blocks.insert(Complex::new(instruction.0.unwrap(), instruction.1.unwrap())); },
                        _ => {}
                    }
                    instruction = (None, None, None);
                }
            },
            Status::Halt => break,
            _ => {}
        }
    }
    format!("{}", blocks.len())
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day13_part1 () {
        assert_eq!(super::part1("0"), "0");
    }

    #[test]
    fn day13_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
