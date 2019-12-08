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

#[derive(Debug)]
struct Amplifier {
    pos: usize,
    opcodes: Opcodes,
    input: VecDeque<i64>
}
impl Amplifier {
    fn new (opcodes: &Opcodes, inputs: Vec<i64>) -> Self {
        let mut input = VecDeque::new();
        for i in inputs {
            input.push_back(i);
        }
        Self {
            pos: 0,
            opcodes: opcodes.clone(),
            input
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

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut max_output = 0;
    for permutation in (0..5).permutations(5) {
        let mut output = 0;
        for phase in permutation {
            let mut amplifier = Amplifier::new(&opcodes, vec![phase, output]);
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
        let mut amplifiers = vec![
            Amplifier::new(&opcodes, vec![permutation[0]]),
            Amplifier::new(&opcodes, vec![permutation[1]]),
            Amplifier::new(&opcodes, vec![permutation[2]]),
            Amplifier::new(&opcodes, vec![permutation[3]]),
            Amplifier::new(&opcodes, vec![permutation[4]])
        ];

        let mut last_output_signal = -1;
        let mut output = 0;

        for amplifier_id in (0..5).cycle() {
            amplifiers[amplifier_id].add_input(output);
            match amplifiers[amplifier_id].run_until_interrupted() {
                Status::Output(out) => {
                    if amplifier_id == 4 { last_output_signal = out; }
                    output = out;
                }
                Status::Halt => { break },
                _ => {}
            }
        };
        max_output = cmp::max(max_output, last_output_signal);
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
        assert_eq!(super::part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), "139629729");
        assert_eq!(super::part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), "18216");
    }
}
