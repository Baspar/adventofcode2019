use crate::intcode::{Opcodes,Status,Machine};

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
    let mut amplifier = Machine::new(&opcodes).add_input(1);
    let mut res = None;
    loop {
        match amplifier.step() {
            Status::Output(output) => res = Some(output),
            Status::Halt => break,
            _ => {}
        }
    }

    res.unwrap().to_string()
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut amplifier = Machine::new(&opcodes).add_input(2);
    let mut res = None;
    loop {
        match amplifier.step() {
            Status::Output(output) => res = Some(output),
            Status::Halt => break,
            _ => {}
        }
    }

    res.unwrap().to_string()
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
        assert_eq!(super::part1("104,1125899906842624,99"), "1125899906842624");
    }
}
