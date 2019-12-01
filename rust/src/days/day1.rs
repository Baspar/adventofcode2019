// Helper
fn read_input (input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|s: &str| s.parse().unwrap())
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    format!("{:?}",
        read_input(input)
            .iter()
            .map(|i: &i64| i / 3 - 2)
            .fold(0i64, |a, b| { a + b as i64 })
    )
}

// Part2
pub fn part2 (_input: &str) -> String {
    format!("{}", _input)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day1_part1 () {
        assert_eq!(super::part1("12"), "2");
        assert_eq!(super::part1("13"), "2");
        assert_eq!(super::part1("14"), "2");
        assert_eq!(super::part1("15"), "3");
        assert_eq!(super::part1("1969"), "654");
        assert_eq!(super::part1("100756"), "33583");
        assert_eq!(super::part1("12\n14"), "4");
    }

    #[test]
    fn day1_part2 () {
        assert_eq!(0, 0);
    }
}
