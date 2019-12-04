use num::Complex;
use std::collections::{HashMap,HashSet};

// Helper
struct Instruction {
    dir: Complex<i32>,
    length: usize,
}
impl Instruction {
    pub fn new(s: &str) -> Self {
        let dir = match s.get(0..1).unwrap() {
            "R" => Complex::new(1, 0),
            "L" => Complex::new(-1, 0),
            "U" => Complex::new(0, -1),
            _ => Complex::new(0, 1)
        };
        let length = s.get(1..).unwrap().parse().unwrap();
        return Self {
            dir,
            length
        }
    }
}
fn read_input (input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line: &str| line
            .split(",")
            .map(|instr: &str| Instruction::new(instr))
            .collect()
        )
        .collect()
}
fn draw_lines (lines: Vec<Vec<Instruction>>) -> Vec<HashMap<Complex<i32>, i32>> {
    lines
        .iter()
        .map(|line| {
            let mut visited: HashMap<Complex<i32>, i32> = HashMap::new();
            let mut pos: Complex<i32> = Complex::new(0, 0);
            let mut step = 0;
            for Instruction {dir, length} in line {
                for _ in 1..=*length {
                    step += 1;
                    pos += dir;
                    if visited.get(&pos).is_none() {
                        visited.insert(pos, step);
                    }
                }
            }
            visited
        })
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let lines = read_input(input);
    let all_positions = draw_lines(lines);
    let line1 = all_positions.get(0).unwrap();
    let line2 = all_positions.get(1).unwrap();
    let positions1: HashSet<Complex<i32>> = line1.keys().map(|x| *x).collect();
    let positions2: HashSet<Complex<i32>> = line2.keys().map(|x| *x).collect();
    let min = positions1
        .intersection(&positions2)
        .map(|pos| pos.im.abs() + pos.re.abs())
        .fold(100000, |a, b| a.min(b));
    format!("{}", min)
}

// Part2
pub fn part2 (input: &str) -> String {
    let lines = read_input(input);
    let all_positions = draw_lines(lines);
    let line1 = all_positions.get(0).unwrap();
    let line2 = all_positions.get(1).unwrap();
    let positions1: HashSet<Complex<i32>> = line1.keys().map(|x| *x).collect();
    let positions2: HashSet<Complex<i32>> = line2.keys().map(|x| *x).collect();
    let min = positions1
        .intersection(&positions2)
        .map(|pos| line1.get(pos).unwrap() + line2.get(pos).unwrap())
        .fold(100000, |a, b| a.min(b));
    format!("{}", min)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day3_part1 () {
        assert_eq!(super::part1("R8,U5,L5,D3
U7,R6,D4,L4"), "6");
        assert_eq!(super::part1("R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"), "159");
        assert_eq!(super::part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), "135");
    }

    #[test]
    fn day3_part2 () {
        assert_eq!(super::part2("R8,U5,L5,D3
U7,R6,D4,L4"), "30");
        assert_eq!(super::part2("R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83"), "610");
        assert_eq!(super::part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), "410");
    }
}
