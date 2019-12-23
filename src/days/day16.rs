use itertools::Itertools;
use ndarray::{Array1,Array2};

// Helper
fn read_input (input: &str) -> Vec<i64> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            err => panic!("{} is not a digit", err)
        })
        .collect()
}
fn create_matrix (n: usize) -> Array2<i64> {
    let mut matrix = Array2::<i64>::zeros((n, n));
    for y in 0..n {
        let v: Vec<i64> = vec![
            vec![0; y+1],
            vec![1; y+1],
            vec![0; y+1],
            vec![-1; y+1]
        ]
            .iter()
            .flat_map(|x| x)
            .cycle()
            .take(n+1)
            .map(|x| x.clone())
            .collect();
        for x in 0..n {
            *matrix
                .get_mut((y, x))
                .expect("To have value") = v[x + 1];
        }
    }
    matrix
}

// Part1
pub fn part1 (input: &str) -> String {
    let input = read_input(input);
    let matrix = create_matrix(input.len());
    let mut vec = Array1::from(input);
    for _ in 0..100 {
        vec = matrix.dot(&vec).map(|x| x.abs() % 10);
    }

    vec
        .iter()
        .take(8)
        .fold(String::new(), |a, b| format!("{}{}", a, b))
}

// Part2
pub fn part2 (input: &str) -> String {
    let mut input = read_input(input);
    let index = (0..7)
        .fold(0usize, |a, b| 10 * a + input[b] as usize);
    input = input
        .iter()
        .cycle()
        .take(10000 * input.len())
        .skip(index)
        .map(|x| x.clone())
        .collect();
    for _ in 0..100 {
        let mut new_input = input.clone();
        let mut count = 0;
        for i in 1..=input.len() {
            count = (count + input[input.len() - i]).abs() % 10;
            new_input[input.len() - i] = count;
        }

        input = new_input;
    }

    input
        .iter()
        .take(8)
        .fold(String::new(), |a, b| format!("{}{}", a, b))
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day16_part1 () {
        assert_eq!(super::part1("80871224585914546619083218645595"), String::from("24176176"));
        assert_eq!(super::part1("19617804207202209144916044189917"), String::from("73745418"));
        assert_eq!(super::part1("69317163492948606335995924319873"), String::from("52432133"));
    }

    #[test]
    fn day16_part2 () {
        assert_eq!(super::part2("03036732577212944063491565474664"), "84462026");
        assert_eq!(super::part2("02935109699940807407585447034323"), "78725270");
        assert_eq!(super::part2("03081770884921959731165446850517"), "53553731");
    }
}
