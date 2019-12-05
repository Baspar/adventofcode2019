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
fn main_loop(mut opcodes: Vec<i64>, system_id: i64) -> i64 {
    let mut pos = 0;
    let mut out = None;
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
                opcodes[a as usize] = system_id;
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
    let output = main_loop(opcodes, 1);
    format!("{}", output)
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let output = main_loop(opcodes, 5);
    format!("{}", output)
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
