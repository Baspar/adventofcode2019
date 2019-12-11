use std::collections::{VecDeque,HashMap};
use num::Complex;

type Int = i64;
type Opcodes = Vec<Int>;

#[derive(Debug)]
enum Status {
    Ok,
    Output(Int),
    WaitingForInput,
    Halt,
    Error(Int)
}

#[derive(Debug)]
struct Machine {
    pos: usize,
    relative_base: usize,
    opcodes: Opcodes,
    input: VecDeque<Int>
}
impl Machine {
    fn new (opcodes: &Opcodes) -> Self {
        Self {
            pos: 0,
            relative_base: 0,
            opcodes: opcodes.clone(),
            input: VecDeque::new()
        }
    }
    fn add_input (self: &mut Self, i: Int) {
        self.input.push_back(i);
    }
    fn assign_to (self: &mut Self, pos: usize, value: Int) {
        if self.opcodes.len() <= pos {
            self.opcodes.resize(pos + 1, 0);
        }
        self.opcodes[pos] = value;
    }
    fn get_opcode (self: &mut Self, pos: usize) -> Int {
        *self.opcodes.get(pos).unwrap_or(&0)
    }
    fn get_address (self: &mut Self, shift: u32) -> Int {
        let mode = self.get_opcode(self.pos) / 10i64.pow(shift + 1) % 10;
        let value = self.get_opcode(self.pos + shift as usize);
        match mode {
            0 => value,
            2 => self.relative_base as Int + value,
            _ => panic!("ERR")
        }
    }
    fn get_param (self: &mut Self, shift: u32) -> Int {
        let mode = self.get_opcode(self.pos) / 10i64.pow(shift + 1) % 10;
        let value = self.get_opcode(self.pos + shift as usize);
        match mode {
            0 => self.get_opcode(value as usize),
            1 => value,
            2 => self.get_opcode((self.relative_base as Int + value) as usize),
            _ => panic!("ERR")
        }
    }
    fn step (self: &mut Self) -> Status {
        let opcode = self.get_opcode(self.pos) % 100;
        match opcode {
            1 => { // Addition
                let a = self.get_param(1);
                let b = self.get_param(2);
                let c = self.get_address(3);
                self.assign_to(c as usize, a + b);
                self.pos += 4;
            },
            2 => { // Multiplication
                let a = self.get_param(1);
                let b = self.get_param(2);
                let c = self.get_address(3);
                self.assign_to(c as usize, a * b);
                self.pos += 4;
            },
            3 => { // Input
                let a = self.get_address(1);
                if self.input.is_empty() {
                    return Status::WaitingForInput;
                }

                let input = self.input.pop_front().unwrap();
                self.assign_to(a as usize, input);
                self.pos += 2;
            },
            4 => { // Output
                let a = self.get_param(1);
                self.pos += 2;
                return Status::Output(a)
            },
            5 => { //jump-if-true
                let a = self.get_param(1);
                let b = self.get_param(2);
                if a != 0 {
                    self.pos = b as usize
                } else {
                    self.pos += 3;
                }
            },
            6 => { //jump-if-false
                let a = self.get_param(1);
                let b = self.get_param(2);
                if a == 0 {
                    self.pos = b as usize
                } else {
                    self.pos += 3;
                }
            },
            7 => { //less-than
                let a = self.get_param(1);
                let b = self.get_param(2);
                let c = self.get_address(3);
                self.assign_to(c as usize, if a < b {1} else {0});
                self.pos += 4;
            },
            8 => { //equals
                let a = self.get_param(1);
                let b = self.get_param(2);
                let c = self.get_address(3);
                self.assign_to(c as usize, if a == b {1} else {0});
                self.pos += 4;
            },
            9 => { //
                let a = self.get_param(1);
                self.relative_base = (self.relative_base as Int + a) as usize;
                self.pos += 2;
            }
            99 => { // Exit
                return Status::Halt;
            },
            err => { // Error
                return Status::Error(err);
            }
        }

        Status::Ok
    }
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
    let mut map = HashMap::<Complex<Int>, Int>::new();
    let mut pos = Complex::new(0, 0);
    let mut orientation = Complex::new(0, -1);
    'main_loop: loop {
        let current_color = map.get(&pos).unwrap_or(&0);
        machine.add_input(current_color.clone());

        let mut color = None;
        let mut rotation = None;
        while color.is_none() || rotation.is_none() {
            match machine.step() {
                Status::Output(o) => {
                    if color.is_none() {
                        color = Some(o);
                    } else {
                        rotation = Some(o);
                    }
                },
                Status::Halt      => break 'main_loop,
                Status::Ok        => {},
                err               => panic!("Received a state {:?}", err)
            }
        }
        map.insert(pos.clone(), color.unwrap());
        orientation *= if rotation.unwrap() == 1 { Complex::new(0, 1) } else { Complex::new(0, -1) };
        pos += orientation;
    }
    format!("{}", map.len())
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    let mut map = HashMap::<Complex<Int>, Int>::new();
    let mut pos = Complex::new(0, 0);
    let mut orientation = Complex::new(0, -1);
    map.insert(pos.clone(), 1);
    'main_loop: loop {
        let current_color = map.get(&pos).unwrap_or(&0);
        machine.add_input(current_color.clone());

        let mut color = None;
        let mut rotation = None;
        while color.is_none() || rotation.is_none() {
            match machine.step() {
                Status::Output(o) => {
                    if color.is_none() {
                        color = Some(o);
                    } else {
                        rotation = Some(o);
                    }
                },
                Status::Halt      => break 'main_loop,
                Status::Ok        => {},
                err               => panic!("Received a state {:?}", err)
            }
        }
        map.insert(pos.clone(), color.unwrap());
        orientation *= if rotation.unwrap() == 1 { Complex::new(0, 1) } else { Complex::new(0, -1) };
        pos += orientation;
    }

    let (min_x, max_x, min_y, max_y) = map
        .keys()
        .fold((i64::max_value(), 0, i64::max_value(), 0), |(min_x, max_x, min_y, max_y), a| (
                std::cmp::min(min_x, a.re),
                std::cmp::max(max_x, a.re),
                std::cmp::min(min_y, a.im),
                std::cmp::max(max_y, a.im)
        ));

    println!("");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Complex::new(x, y);
            let color = map.get(&pos).unwrap_or(&0);
            print!("{}", if *color == 1 { '#' } else { ' ' });
        }
        println!("");
    }
    format!("{}", map.len())
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day11_part1 () {
        assert_eq!(super::part1("
                3, 98,
                4, 1, 4, 0,
                3, 98,
                4, 0, 4, 0,
                3, 98,
                4, 1, 4, 0,
                3, 98,
                4, 1, 4, 0,
                3, 98,
                4, 0, 4, 1,
                3, 98,
                4, 1, 4, 0,
                3, 98,
                4, 1, 4, 0,
                99"), "0");
    }

    #[test]
    fn day11_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
