use std::collections::VecDeque;

pub type Opcodes = Vec<i64>;

#[derive(Debug)]
pub enum Status {
    Ok,
    Output(i64),
    WaitingForInput,
    Halt,
    Error(i64)
}

#[derive(Debug)]
pub struct Machine {
    pub pos: usize,
    pub relative_base: usize,
    pub opcodes: Vec<i64>,
    pub input: VecDeque<i64>
}

impl Machine {
    pub fn new (opcodes: &Vec<i64>) -> Self {
        Self {
            pos: 0,
            relative_base: 0,
            opcodes: opcodes.clone(),
            input: VecDeque::new()
        }
    }
    fn assign_to (self: &mut Self, pos: usize, value: i64) {
        if self.opcodes.len() <= pos {
            self.opcodes.resize(pos + 1, 0);
        }
        self.opcodes[pos] = value;
    }
    fn get_opcode (self: &mut Self, pos: usize) -> i64 {
        *self.opcodes.get(pos).unwrap_or(&0)
    }
    fn get_address (self: &mut Self, shift: u32) -> i64 {
        let mode = self.get_opcode(self.pos) / 10i64.pow(shift + 1) % 10;
        let value = self.get_opcode(self.pos + shift as usize);
        match mode {
            0 => value,
            2 => self.relative_base as i64 + value,
            _ => panic!("ERR")
        }
    }
    fn get_param (self: &mut Self, shift: u32) -> i64 {
        let mode = self.get_opcode(self.pos) / 10i64.pow(shift + 1) % 10;
        let value = self.get_opcode(self.pos + shift as usize);
        match mode {
            0 => self.get_opcode(value as usize),
            1 => value,
            2 => self.get_opcode((self.relative_base as i64 + value) as usize),
            _ => panic!("ERR")
        }
    }
    pub fn add_input_mut (self: &mut Self, i: i64)  {
        self.input.push_back(i);
    }
    pub fn add_input (mut self: Self, i: i64) -> Self {
        self.add_input_mut(i);
        self
    }
    pub fn step (self: &mut Self) -> Status {
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
                self.relative_base = (self.relative_base as i64 + a) as usize;
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
    pub fn run_until_interrupted (self: &mut Self) -> Status {
        loop {
            match self.step() {
                Status::Ok              => {}
                Status::Output(i)       => { return Status::Output(i) }
                Status::WaitingForInput => { return Status::WaitingForInput }
                Status::Halt            => { return Status::Halt },
                Status::Error(err)      => panic!(err)
            }
        }
    }
}
