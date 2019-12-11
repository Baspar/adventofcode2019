use crate::intcode::{Status,Machine};

// Helper
fn read_input (input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|s: &str| s.parse().unwrap())
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes).add_input(1);
    loop {
        match machine.step() {
            Status::Output(o) => return format!("{}", o),
            _ => {}
        }
    }
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes).add_input(5);
    loop {
        match machine.step() {
            Status::Output(o) => return format!("{}", o),
            _ => {}
        }
    }
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day2_part1 () {
    }

    #[test]
    fn day2_part2 () {
    }
}
