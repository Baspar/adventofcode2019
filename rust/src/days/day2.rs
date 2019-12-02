// Helper
fn read_input (input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        // .collect::<Vec<&str>>()
        .join("")
        .split(",")
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
fn main_loop(mut opcodes: Vec<usize>, noun: usize, verb: usize) -> usize {
    opcodes[1] = noun;
    opcodes[2] = verb;
    let mut pos = 0;
    loop {
        match opcodes[pos] {
            1 => {
                let [a, b, c] = [
                    opcodes[pos + 1],
                    opcodes[pos + 2],
                    opcodes[pos + 3]
                ];
                opcodes[c] = opcodes[a] + opcodes[b];
            },
            2 => {
                let [a, b, c] = [
                    opcodes[pos + 1],
                    opcodes[pos + 2],
                    opcodes[pos + 3]
                ];
                opcodes[c] = opcodes[a] * opcodes[b];
            },
            99 => { break },
            _ => { break }
        }
        pos += 4;
    }
    return opcodes[0];
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let output = main_loop(opcodes, 12, 2);
    format!("{}", output)
}

// Part2
pub fn part2 (input: &str) -> String {
    let expected_output = 19690720;
    let opcodes = read_input(input);
    for noun in 0..100 {
        for verb in 0..100 {
            let output = main_loop(opcodes.clone(), noun, verb);
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
