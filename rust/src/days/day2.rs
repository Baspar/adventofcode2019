// Helper
fn read_input (input: &str) -> Vec<i32> {
    input
        .trim()
        .split(",")
        .map(|s: &str| s.parse().unwrap())
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let mut opcodes = read_input(input);
    opcodes[1] = 12;
    opcodes[2] = 2;
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
    format!("{}", opcodes[0])
}

// Part2
pub fn part2 (_input: &str) -> String {
    format!("{}", _input)
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
