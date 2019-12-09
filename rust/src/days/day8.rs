use itertools::Itertools;

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
    let width = 25;
    let height = 6;
    let nb_pixel = width * height;
    let nb_layer = input.len() / nb_pixel;

    let chars: Vec<char> = input.chars().collect();
    let mut out = String::new();

    for y in 0..height {
        for x in 0..width {
            for layer_index in 0..nb_layer {
                match chars[layer_index * nb_pixel + y * width + x] {
                    '0' => {out += " "; break},
                    '1' => {out += "#"; break},
                    _   => {}
                }
            }
        }
        out += "\n";
    }

    out
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day8_part1 () {
        assert_eq!(super::part1("0"), "0");
    }

    #[test]
    fn day8_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
