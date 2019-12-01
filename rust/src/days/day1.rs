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
            .fold(0i64, |a, b| { a + b })
    )
}

// Part2
pub fn part2 (input: &str) -> String {
    let fuel = read_input(input)
        .iter()
        .map(|weight: &i64| {
            let mut i = *weight;
            let mut fuel = 0;
            i = i / 3 - 2;
            while i > 0 {
                fuel += i;
                i = i / 3 - 2;
            }
            fuel
        })
        .fold(0i64, |a, b| { a + b });

    format!("{}", fuel)
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
        assert_eq!(super::part2("14"), "2");
        assert_eq!(super::part2("1969"), "966");
        assert_eq!(super::part2("100756"), "50346");
    }
}
