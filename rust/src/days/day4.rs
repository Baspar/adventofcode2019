// Helper
fn read_input (input: &str) -> (i64, i64) {
    let s: Vec<&str> = input.trim().split("-").collect();
    (
        s.get(0).unwrap().parse().unwrap(),
        s.get(1).unwrap().parse().unwrap()
    )
}
fn is_valid(mut n: i64) -> bool {
    let mut two_same = false;
    let mut never_decrease = true;

    let mut next_digit = n % 10;

    while { n /= 10; n != 0 } {
        let c = n % 10;
        two_same |= next_digit == c;
        never_decrease &= next_digit >= c;
        next_digit = c;
    }

    two_same && never_decrease
}
fn is_valid_part2(mut n: i64) -> bool {
    let mut two_same = false;
    let mut never_decrease = true;
    let mut current_count = 1;

    let mut next_digit = n % 10;

    while { n /= 10; n != 0 } {
        let c = n % 10;
        never_decrease &= next_digit >= c;
        if c == next_digit {
            current_count += 1
        } else {
            if current_count == 2 { two_same = true }
            current_count = 1
        }
        next_digit = c;
    }
    if current_count == 2 { two_same = true }

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
    let (from, to) = read_input(input);
    let n = (from..=to)
        .filter(|i| is_valid_part2(*i))
        .count();
    return format!("{}", n)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day4_part1 () {
        assert_eq!(super::part1("111111-111111"), "1");
        // assert_eq!(super::part1("223450-223450"), "0");
        // assert_eq!(super::part1("123789-123789"), "0");
    }

    #[test]
    fn day4_part2 () {
        assert_eq!(super::part2("112233-112233"), "1");
        assert_eq!(super::part2("123444-123444"), "0");
        assert_eq!(super::part2("111122-111122"), "1");
    }
}
