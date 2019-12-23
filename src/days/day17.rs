use std::collections::HashMap;
use num::Complex;
use crate::intcode::{Opcodes,Status,Machine};

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
struct Map {
    bot_pos: Complex<i64>,
    bot_orientation: Complex<i64>,
    max_x: i64,
    max_y: i64,
    map: HashMap<Complex<i64>, char>
}
impl Map {
    fn new (mut machine: Machine) -> Self {
        let mut map: HashMap<Complex<i64>, char> = HashMap::new();
        let mut pos = Complex::new(0, 0);
        let mut max_x = 0;
        let mut max_y = 0;
        let mut bot_pos = Complex::new(0, 0);
        let mut bot_orientation = Complex::new(0, 0);
        loop {
            match machine.step() {
                Status::Halt => break,
                Status::Output(10) => {
                    pos.re = 0;
                    pos.im += 1;
                },
                Status::Output(x) => {
                    let c = x as u8 as char;
                    match c {
                        '^' => {
                            bot_pos = pos;
                            bot_orientation = Complex::new(0, -1);
                        },
                        'V' => {
                            bot_pos = pos;
                            bot_orientation = Complex::new(0, 1);
                        },
                        '<' => {
                            bot_pos = pos;
                            bot_orientation = Complex::new(-1, 0);
                        },
                        '>' => {
                            bot_pos = pos;
                            bot_orientation = Complex::new(1, 0);
                        },
                        _   => { }
                    };
                    max_x = max_x.max(pos.re);
                    max_y = max_y.max(pos.im);
                    match c {
                        '.' => map.insert(pos, '.'),
                        _   => map.insert(pos, '#')
                    };
                    pos.re += 1;
                },
                Status::Ok => {},
                err => { println!("{:?}", err) }
            }
        }

        Self {
            map,
            max_y,
            max_x,
            bot_orientation,
            bot_pos
        }
    }

    #[allow(dead_code)]
    fn display(self: &Self) {
        println!("");
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if Complex::new(x, y) == self.bot_pos {
                    match (self.bot_orientation.re, self.bot_orientation.im) {
                        (0, -1) => print!("^"),
                        (0, 1)  => print!("V"),
                        (-1, 0) => print!("<"),
                        (1, 0)  => print!(">"),
                        _       => panic!("Unknown")
                    }
                } else {
                    print!("{}", self.map[&Complex::new(x, y)]);
                }
            }
            println!("");
        }
    }

    fn part1 (self: &Self) -> i64 {
        let mut out = 0;
        for (Complex {re: x, im: y}, c) in &self.map {
            if *c == '.' { continue }
            if
                self.map.get(&Complex::new(*x + 1, *y)) == Some(&'#') &&
                self.map.get(&Complex::new(*x - 1, *y)) == Some(&'#') &&
                self.map.get(&Complex::new(*x, *y + 1)) == Some(&'#') &&
                self.map.get(&Complex::new(*x, *y - 1)) == Some(&'#')
            {
                out += x * y
            }
        }
        out
    }

    fn get_instruction (self: &mut Self) -> String {
        let mut out = Vec::new();
        loop {
            let left_orientation = Complex::new(0, -1) * self.bot_orientation;
            let left_pos = self.bot_pos + left_orientation;
            let left_value = self.map.get(&left_pos);
            let right_orientation = Complex::new(0, 1) * self.bot_orientation;
            let right_pos = self.bot_pos + right_orientation;
            let right_value = self.map.get(&right_pos);
            match (left_value, right_value) {
                (Some('#'), _) => {
                    out.push(String::from("L"));
                    self.bot_orientation = left_orientation;
                },
                (_, Some('#')) => {
                    out.push(String::from("R"));
                    self.bot_orientation = right_orientation;
                },
                _ => break
            }

            let mut i = 0;
            while self.map.get(&(self.bot_pos + self.bot_orientation)) == Some(&'#') {
                self.bot_pos += self.bot_orientation;
                i += 1;
            }

            out.push(i.to_string());

            self.display()
        }

        out
            .iter()
            .fold(String::from(""), |a, b| a + b + ",")
    }
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);

    let map = Map::new(Machine::new(&opcodes));

    // map.display();
    format!("{}", map.part1())
}

// Part2
pub fn part2 (input: &str) -> String {
    let mut opcodes = read_input(input);
    // opcodes[0] = 2;
    // R12L8L4L4L8R6L6
    // R12L8L4L4L8R6L6
    // L8L4R12L6L4R12L8L4L4
    // L8L4R12L6L4R12L8L4L4
    // L8L4R12L6L4L8R6L6
    //
    // R12L8L4L4L8R6L6
    // L8L4R12L6L4R12L8L4L4
    // L8L4R12L6L4L8R6L6

    let mut map = Map::new(Machine::new(&opcodes));
    map.display();
    let instructions = map.get_instruction();

    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day17_part1 () {
        assert_eq!(super::part1("104,46,104,46,104,35,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,10,104,46,104,46,104,35,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,10,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,46,104,46,104,46,104,35,104,35,104,35,104,10,104,35,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,35,104,10,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,10,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,10,104,46,104,46,104,35,104,35,104,35,104,35,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,10,99"), "76");
    }

    #[test]
    fn day17_part2 () {
        assert_eq!(super::part2("104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,46,104,46,104,46,104,35,104,35,104,35,104,35,104,35,104,10,104,35,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,10,104,35,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,10,104,46,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,10,104,46,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,35,104,35,104,46,104,35,104,10,104,46,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,35,104,10,104,94,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,46,104,46,104,46,104,35,104,46,104,35,104,10,104,46,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,35,104,10,104,46,104,46,104,46,104,46,104,46,104,46,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,10,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,10,104,46,104,46,104,46,104,46,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,35,104,46,104,46,104,10,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,46,104,46,104,46,104,10,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,46,104,46,104,46,104,10,104,46,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,35,104,46,104,46,104,46,104,46,104,46,104,46,104,10,104,46,104,46,104,46,104,46,104,35,104,35,104,35,104,35,104,35,104,46,104,46,104,46,104,46,104,46,104,46,99"), "");
    }
}
