use text_io::*;
use crate::intcode::{Opcodes,Status,Machine};
use std::collections::HashMap;

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
    pub current_pos: (Option<i64>, Option<i64>),
    pub paddle_x: i64,
    pub ball_x: i64,
    pub ball_dx: i64
}
impl Pong {
    pub fn new () -> Self {
        Self {
            blocks: HashMap::new(),
            score: 0,
            current_pos: (None, None),
            paddle_x: 0,
            ball_x: 0,
            ball_dx: 1
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
                        let x = self.current_pos.0.unwrap();
                        let y = self.current_pos.1.unwrap();
                        let pos = (self.current_pos.0.unwrap(), self.current_pos.1.unwrap());
                        match tile {
                            Tile::Paddle => {
                                self.paddle_x = x;
                            },
                            Tile::Ball => {
                                self.ball_dx = if self.ball_x < x { 1 } else { -1 };
                                self.ball_x = x;
                            },
                            _ => {}
                        }
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

    pub fn display (self: &Self) {
        let (x_min, x_max) = self
            .blocks
            .keys()
            .fold((i64::max_value(), 0), |(x_min, x_max), (x, _)| (x_min.min(*x), x_max.max(*x)));
        let (y_min, y_max) = self
            .blocks
            .keys()
            .fold((i64::max_value(), 0), |(y_min, y_max), (_, y)| (y_min.min(*y), y_max.max(*y)));

        println!("");
        println!("Score: {}, ball_dx: {}, ball_x: {}, paddle_x: {}", self.score, self.ball_dx, self.ball_x, self.paddle_x);
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let character = match self.blocks.get(&(x, y)) {
                    Some(Tile::Wall) => "█",
                    Some(Tile::Block) => "#",
                    Some(Tile::Paddle) => "=",
                    Some(Tile::Ball) => "*",
                    _ => " "
                };
                print!("{}", character);
            }
            println!("");
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
    machine.opcodes[0] = 2;
    let mut pong = Pong::new();
    loop {
        let status = pong.parse_output(&mut machine);
        // pong.display();
        match status {
            Status::WaitingForInput => {
                // let _: String = read!("{}\n");
                let Pong {paddle_x, ball_x, ball_dx, blocks: _, score: _, current_pos: _} = pong;
                let input = if ball_dx == 1 && ball_x > paddle_x {
                    1
                } else if ball_dx == -1 && ball_x < paddle_x {
                    -1
                } else {
                    0
                };
                machine.add_input_mut(input);
            },
            _ => break
        }
    }
    format!("{}", pong.score)
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
