use itertools::Itertools;

// Helper
fn read_input (_input: &str) -> i32 {
    0
}

// Part1
pub fn part1 (input: &str) -> String {
    let nb_pixel = 25 * 6;

    let mut min_0 = i64::max_value();
    let mut output = 0;

    for layer in input.trim().chars().chunks(nb_pixel).into_iter() {
        let mut count_0 = 0;
        let mut count_1 = 0;
        let mut count_2 = 0;
        for c in layer {
            match c {
                '0' => count_0 += 1,
                '1' => count_1 += 1,
                '2' => count_2 += 1,
                _ => {}
            }
        }
        if count_0 < min_0 {
            output = count_1 * count_2;
            min_0 = count_0;
        }
    }
    format!("{}", output)
}

// Part2
pub fn part2 (input: &str) -> String {
    format!("{}", read_input(input))
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day8_part1 () {
        assert_eq!(super::part1("1234567890120"), "0");
    }

    #[test]
    fn day8_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
