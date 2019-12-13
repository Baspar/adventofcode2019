use num::Complex;
use crate::intcode::{Opcodes,Status,Machine};
use std::collections::{HashSet,HashMap};

// Helper
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}
fn read_input (input: &str) -> Opcodes {
    input
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .split(",")
        .map(|s| s.trim().parse().expect("Cannot parse int"))
        .collect()
}
struct Pong {
    pub blocks: HashMap<(i64,i64), Tile>,
    pub score: i64,
    pub current_pos: (Option<i64>, Option<i64>)
}
impl Pong {
    pub fn new () -> Self {
        Self {
            blocks: HashMap::new(),
            score: 0,
            current_pos: (None, None)
        }
    }

    pub fn parse_output (self: &mut Self, machine: &mut Machine) -> Status {
        loop {
            match machine.step() {
                Status::Output(o) => {
                    if self.current_pos.0.is_none() {
                        self.current_pos.0 = Some(o);
                    } else if self.current_pos.1.is_none() {
                        self.current_pos.1 = Some(o);
                    } else if self.current_pos== (Some(-1), Some(0)) {
                        self.score = o;
                        self.current_pos = (None, None);
                    } else {
                        let tile = match o {
                            0 => Tile::Empty,
                            1 => Tile::Wall,
                            2 => Tile::Block,
                            3 => Tile::Paddle,
                            4 => Tile::Ball,
                            tile => panic!("Unknown tile: {}", tile)
                        };
                        let pos = (self.current_pos.0.unwrap(), self.current_pos.1.unwrap());
                        self.blocks.insert(pos, tile);
                        self.current_pos = (None, None);
                    }
                },
                Status::Halt => return Status::Halt,
                Status::WaitingForInput => return Status::WaitingForInput,
                _ => {}
            }
        }
    }
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    let mut pong = Pong::new();
    pong.parse_output(&mut machine);
    let nb_blocks = pong
        .blocks
        .values()
        .filter(|x| match x { Tile::Block => true, _ => false })
        .count();
    format!("{}", nb_blocks)
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
