use std::collections::VecDeque;

type Int = i64;
type Opcodes = Vec<Int>;

enum Status {
    Ok,
    Output(Int),
    WaitingForInput,
    Halt,
    Error(Int)
}

#[derive(Debug)]
struct Amplifier {
    pos: usize,
    relative_base: usize,
    opcodes: Opcodes,
    input: VecDeque<Int>
}
impl Amplifier {
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
        .trim()
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .split(",")
        .map(|s: &str| s.parse().unwrap())
        .enumerate()
        .fold(Opcodes::new(), |mut m, (i, v)| {
            m.insert(i, v);
            m
        })
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut amplifier = Amplifier::new(&opcodes);
    amplifier.add_input(1);
    let mut res = String::new();
    loop {
        match amplifier.step() {
            Status::Output(output) => res += &format!(",{}", output),
            Status::Halt => break,
            Status::Error(error) => return format!("Error: {}", error),
            _ => {}
        }
    }

    res.remove(0);
    res
    // format!("{}", res.unwrap())
}

// Part2
pub fn part2 (input: &str) -> String {
    let _opcodes = read_input(input);
    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day9_part1 () {
        assert_eq!(super::part1("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"), "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        assert_eq!(super::part1("1102,34915192,34915192,7,4,7,99,0").len(), 16);
        assert_eq!(super::part1("104,1125899906842624,99"), "1125899906842624");
    }

    #[test]
    fn day9_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
