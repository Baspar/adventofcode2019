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
fn main_loop(mut opcodes: Vec<i64>) -> i64 {
    let mut pos = 0;
    let mut out = None;
    loop {
        let opcode = opcodes[pos] % 100;
        let mode_param_1 = opcodes[pos] / 100 % 10;
        let mode_param_2 = opcodes[pos] / 1000 % 10;
        // let mode_param_3 = opcodes[pos] / 10000 % 10;
        match opcode {
            1 => { // Addition
                let [a, b, c] = [
                    opcodes[pos + 1],
                    opcodes[pos + 2],
                    opcodes[pos + 3]
                ];
                let value_1 = if mode_param_1 == 0 { opcodes[a as usize] } else { a };
                let value_2 = if mode_param_2 == 0 { opcodes[b as usize] } else { b };
                opcodes[c as usize] = value_1 + value_2;
                pos += 4;
            },
            2 => { // Multiplication
                let [a, b, c] = [
                    opcodes[pos + 1],
                    opcodes[pos + 2],
                    opcodes[pos + 3]
                ];
                let value_1 = if mode_param_1 == 0 { opcodes[a as usize] } else { a };
                let value_2 = if mode_param_2 == 0 { opcodes[b as usize] } else { b };
                opcodes[c as usize] = value_1 * value_2;
                pos += 4;
            },
            3 => { // Input
                let a = opcodes[pos + 1];
                opcodes[a as usize] = 1; // Hardcode 1
                pos += 2;
            },
            4 => { // Output
                let a = opcodes[pos + 1];
                let value_1 = if mode_param_1 == 0 { opcodes[a as usize] } else { a };
                out = Some(value_1);
                pos += 2;
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
    let output = main_loop(opcodes);
    format!("{}", output)
}

// Part2
pub fn part2 (input: &str) -> String {
    let expected_output = 19690720;
    let opcodes = read_input(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let output = main_loop(opcodes.clone());
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
        assert_eq!(super::main_loop(super::read_input("1,0,0,0,99"), 0, 0), 2);
    }

    #[test]
    fn day2_part2 () {
    }
}
