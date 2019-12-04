// Helper
fn read_input (input: &str) -> (i64, i64) {
    let s: Vec<&str> = input.trim().split("-").collect();
    (
        s.get(0).unwrap().parse().unwrap(),
        s.get(1).unwrap().parse().unwrap()
    )
}
fn is_valid(n: i64) -> bool {
    let s = format!("{}", n);
    let chars: Vec<char> = s.chars().collect();

    let mut two_same = false;
    let mut never_decrease = true;

    for i in 1..s.len() {
        let a = chars.get(i - 1).unwrap();
        let b = chars.get(i).unwrap();
        if a == b {
            two_same = true;
        }
        if a > b {
            never_decrease = false;
        }
    }

    two_same && never_decrease
}

// Part1
pub fn part1 (input: &str) -> String {
    let (from, to) = read_input(input);
    let n = (from..=to)
        .filter(|i| is_valid(*i))
        .count();
    return format!("{}", n)
}

// Part2
pub fn part2 (input: &str) -> String {
    format!("{:?}", read_input(input))
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day4_part1 () {
        assert_eq!(super::part1("111111-111111"), "1");
        assert_eq!(super::part1("223450-223450"), "0");
        assert_eq!(super::part1("123789-123789"), "0");
    }

    #[test]
    fn day4_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
