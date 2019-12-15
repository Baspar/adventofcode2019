use itertools::Itertools;
use std::cmp;
use crate::intcode::{Status,Machine};

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
            let mut machine = Machine::new(&opcodes)
                .add_input(phase)
                .add_input(output);
            match machine.run_until_interrupted() {
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
        let mut machines = vec![
            Machine::new(&opcodes).add_input(permutation[0]),
            Machine::new(&opcodes).add_input(permutation[1]),
            Machine::new(&opcodes).add_input(permutation[2]),
            Machine::new(&opcodes).add_input(permutation[3]),
            Machine::new(&opcodes).add_input(permutation[4])
        ];

        let mut last_output_signal = -1;
        let mut output = 0;

        for machine_id in (0..5).cycle() {
            machines[machine_id].add_input_mut(output);
            match machines[machine_id].run_until_interrupted() {
                Status::Output(out) => {
                    if machine_id == 4 { last_output_signal = out; }
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
