// Helper
fn read_input (input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|s: &str| s.parse().unwrap())
        .collect()
}
fn main_loop(noun: i32, verb: i32, opcodes: &mut Vec<i32>) {
    opcodes[1] = noun;
    opcodes[2] = verb;
    let mut pos = 0;
    loop {
        match opcodes[pos] {
            1 => {
                let a = opcodes[pos + 1] as usize;
                let b = opcodes[pos + 2] as usize;
                let c = opcodes[pos + 3] as usize;
                opcodes[c] = opcodes[a] + opcodes[b];
                pos += 4;
            },
            2 => {
                let a = opcodes[pos + 1] as usize;
                let b = opcodes[pos + 2] as usize;
                let c = opcodes[pos + 3] as usize;
                opcodes[c] = opcodes[a] * opcodes[b];
                pos += 4;
            },
            99 => { break },
            _ => { break }
        }
    }
}

// Part1
pub fn part1 (input: &str) -> String {
    let mut opcodes = read_input(input);
    main_loop(12, 2, &mut opcodes);
    format!("{}", opcodes[0])
}

// Part2
pub fn part2 (input: &str) -> String {
    let expected_output = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut opcodes = read_input(input);
            main_loop(noun, verb, &mut opcodes);
            let output = opcodes[0];
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
        assert_eq!(super::part1("1,0,0,0,99"), "2");
    }

    #[test]
    fn day2_part2 () {
        assert_eq!(0, 0);
    }
}
