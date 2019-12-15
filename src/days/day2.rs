use crate::intcode::{Machine,Status};

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
    let mut opcodes = read_input(input);
    opcodes[1] = 12;
    opcodes[2] = 2;
    let mut machine = Machine::new(&opcodes);
    loop {
        match machine.step() {
            Status::Halt => break,
            _ => {}
        }
    }
    let output = machine.opcodes[0];
    format!("{}", output)
}

// Part2
pub fn part2 (input: &str) -> String {
    let expected_output = 19690720;
    let opcodes = read_input(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let mut opcodes = opcodes.clone();
            opcodes[1] = noun;
            opcodes[2] = verb;
            let mut machine = Machine::new(&opcodes);
            loop {
                match machine.step() {
                    Status::Halt => break,
                    _ => {}
                }
            }
            let output = machine.opcodes[0];
            if expected_output == output {
                return format!("noun = {}, verb = {}", noun, verb);
            }
        }
    }
    return format!("")
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day2_part1 () {
        // assert_eq!(super::main_loop(super::read_input("1,0,0,0,99"), 0, 0), 2);
    }

    #[test]
    fn day2_part2 () {
    }
}
