use std::collections::HashMap;
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
    max_x: i64,
    max_y: i64,
    map: HashMap<(i64, i64), char>
}
impl Map {
    fn new (mut machine: Machine) -> Self{
        let mut map: HashMap<(i64, i64), char> = HashMap::new();
        let mut pos = (0, 0);
        let mut max_x = 0;
        let mut max_y = 0;
        loop {
            match machine.step() {
                Status::Halt => break,
                Status::Output(10) => {
                    pos.1 += 1;
                    pos.0 = 0;
                },
                Status::Output(x) => {
                    max_x = max_x.max(pos.0);
                    max_y = max_y.max(pos.1);
                    map.insert(pos, x as u8 as char);
                    pos.0 += 1;
                },
                Status::Ok => {},
                err => { println!("{:?}", err) }
            }
        }
        Self {
            map,
            max_y,
            max_x
        }
    }

    #[allow(dead_code)]
    fn display(self: &Self) {
        println!("");
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                print!("{}", self.map[&(x, y)]);
            }
            println!("");
        }
    }

    fn part1(self: &Self) -> i64 {
        let mut out = 0;
        for ((x, y), c) in &self.map {
            if *c == '.' { continue }
            if
                self.map.get(&(*x + 1, *y)) == Some(&'#') &&
                self.map.get(&(*x - 1, *y)) == Some(&'#') &&
                self.map.get(&(*x, *y + 1)) == Some(&'#') &&
                self.map.get(&(*x, *y - 1)) == Some(&'#')
            {
                out += x * y
            }
        }
        out
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
    let opcodes = read_input(input);
    let machine = Machine::new(&opcodes);
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
        // assert_eq!(super::part2("0"), "0");
    }
}
