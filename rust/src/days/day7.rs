use itertools::Itertools;
use std::collections::VecDeque;
use std::cmp;

type Opcodes = Vec<i64>;

enum Status {
    Ok,
    Output(i64),
    WaitingForInput,
    Halt
}

struct Amplifier {
    pos: usize,
    opcodes: Opcodes,
    input: VecDeque<i64>
}
impl Amplifier {
    fn new (opcodes: &Opcodes) -> Self {
        Self {
            pos: 0,
            opcodes: opcodes.clone(),
            input: VecDeque::new()
        }
    }
    fn add_input (self: &mut Self, i: i64) {
        self.input.push_back(i);
    }
    fn step (self: &mut Self) -> Status {
        let mode = |shift: u32| {
            self.opcodes[self.pos] / 10i64.pow(shift + 1) % 10
        };
        let get_param = |shift: u32, mode| -> i64 {
            let value = self.opcodes[self.pos + shift as usize];
            if mode == 0 {
                return self.opcodes[value as usize];
            } else {
                return value;
            }
        };
        match self.opcodes[self.pos] % 100 {
            1 => { // Addition
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                self.opcodes[c as usize] = a + b;
                self.pos += 4;
            },
            2 => { // Multiplication
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                self.opcodes[c as usize] = a * b;
                self.pos += 4;
            },
            3 => { // Input
                let a = get_param(1, 1);
                if self.input.is_empty() {
                    return Status::WaitingForInput;
                }

                self.opcodes[a as usize] = self.input.pop_front().unwrap();

                self.pos += 2;
            },
            4 => { // Output
                let a = get_param(1, mode(1));
                self.pos += 2;
                return Status::Output(a)
            },
            5 => { //jump-if-true
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                if a != 0 {
                    self.pos = b as usize;
                } else {
                    self.pos += 3;
                }
            },
            6 => { //jump-if-false
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                if a == 0 {
                    self.pos = b as usize;
                } else {
                    self.pos += 3;
                }
            },
            7 => { //less-than
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                self.opcodes[c as usize] = if a < b {1} else {0};
                self.pos += 4;
            },
            8 => { //equals
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                self.opcodes[c as usize] = if a == b {1} else {0};
                self.pos += 4;
            },
            99 => { // Exit
                return Status::Halt;
            },
            _ => { // Error
                return Status::Halt;
            }
        }

        Status::Ok
    }
    fn run_until_interrupted (self: &mut Self) -> Status {
        loop {
            match self.step() {
                Status::Ok => {}
                Status::Output(i) => { return Status::Output(i) }
                Status::WaitingForInput => { return Status::WaitingForInput }
                Status::Halt => { return Status::Halt }
            }
        }
    }
}

// Helper
fn read_input (input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .split(",")
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
fn main_loop(mut opcodes: Vec<i64>, phase_settings: i64, input: i64) -> i64 {
    let mut pos = 0;
    let mut out = None;
    let mut first_input_done = false;
    loop {
        let mode = |shift: u32| {
            opcodes[pos] / 10i64.pow(shift + 1) % 10
        };
        let get_param = |shift: u32, mode| -> i64 {
            let value = opcodes[pos + shift as usize];
            if mode == 0 {
                return opcodes[value as usize];
            } else {
                return value;
            }
        };
        match opcodes[pos] % 100 {
            1 => { // Addition
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                opcodes[c as usize] = a + b;
                pos += 4;
            },
            2 => { // Multiplication
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                opcodes[c as usize] = a * b;
                pos += 4;
            },
            3 => { // Input
                let a = get_param(1, 1);
                if first_input_done {
                    opcodes[a as usize] = input;
                } else {
                    first_input_done = true;
                    opcodes[a as usize] = phase_settings;
                }
                pos += 2;
            },
            4 => { // Output
                let a = get_param(1, mode(1));
                out = Some(a);
                pos += 2;
            },
            5 => { //jump-if-true
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                if a != 0 {
                    pos = b as usize;
                } else {
                    pos += 3;
                }
            },
            6 => { //jump-if-false
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                if a == 0 {
                    pos = b as usize;
                } else {
                    pos += 3;
                }
            },
            7 => { //less-than
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                opcodes[c as usize] = if a < b {1} else {0};
                pos += 4;
            },
            8 => { //equals
                let a = get_param(1, mode(1));
                let b = get_param(2, mode(2));
                let c = get_param(3, 1);
                opcodes[c as usize] = if a == b {1} else {0};
                pos += 4;
            },
            99 => { // Exit
                break
            },
            _ => { // Error
                break
            }
        }
    }
    return out.unwrap();
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut max_output = 0;
    for permutation in (0..5).permutations(5) {
        let mut output = 0;
        for phase in permutation {
            let mut amplifier = Amplifier::new(&opcodes);
            amplifier.add_input(phase);
            amplifier.add_input(output);
            match amplifier.run_until_interrupted() {
                Status::Output(i) => output = i,
                _ => {}
            }
        }

        max_output = cmp::max(max_output, output);
    }

    format!("{}", max_output)
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut max_output = 0;
    for permutation in (5..10).permutations(5) {
        let mut output = 0;
        for phase in permutation {
            output = main_loop(opcodes.clone(), phase, output);
        }

        max_output = cmp::max(max_output, output);
    }

    format!("{}", max_output)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day7_part1 () {
        assert_eq!(super::part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), "43210");
        assert_eq!(super::part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), "54321");
        assert_eq!(super::part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), "65210");
    }

    #[test]
    fn day7_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
