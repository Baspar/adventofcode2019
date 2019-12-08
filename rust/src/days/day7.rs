use itertools::Itertools;
use std::cmp;

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
            output = main_loop(opcodes.clone(), phase, output);
        }

        max_output = cmp::max(max_output, output);
    }

    format!("{}", max_output)
}

// Part2
pub fn part2 (input: &str) -> String {
    read_input(input);
    // let output = main_loop(opcodes, 5);
    format!("{}", "0")
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
