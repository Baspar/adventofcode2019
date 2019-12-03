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

// Part1
pub fn part1 (input: &str) -> String {
    let lines = read_input(input);
    let mut positions: HashMap<Complex<i32>, HashSet<i32>> = HashMap::new();
    let mut cross: Vec<Complex<i32>> = Vec::new();

    let mut i = 0;
    for line in lines {
        let mut pos: Complex<i32> = Complex::new(0, 0);
        for instruction in line {
            let dir = instruction.dir;
            let length = instruction.length;
            for _ in 1..=length {
                pos += dir;
                match positions.get(&pos) {
                    Some(past_positions) => {
                        if !past_positions.contains(&i) {
                            cross.push(pos)
                        }
                    },
                    _ => {
                        let mut v = HashSet::new();
                        v.insert(i);
                        positions.insert(pos, v);
                    }
                }
            }
        }
        i += 1;
    }

    let min = cross
        .into_iter()
        .map(|c: Complex<i32>| c.im.abs() + c.re.abs())
        .fold(100000, |a, b| if a < b { a } else { b });
    format!("{}", min)
}

// Part2
pub fn part2 (input: &str) -> String {
    let lines = read_input(input);
    let mut positions: HashMap<Complex<i32>, HashMap<i32, i32>> = HashMap::new();
    let mut cross: Vec<(Complex<i32>, i32)> = Vec::new();

    let mut i = 0;
    for line in lines {
        let mut pos: Complex<i32> = Complex::new(0, 0);
        let mut step = 0;
        for instruction in line {
            let dir = instruction.dir;
            let length = instruction.length;
            for _ in 1..=length {
                step += 1;
                pos += dir;
                match positions.get(&pos) {
                    Some(past_positions) => {
                        if past_positions.get(&i).is_none() {
                            let past_step = past_positions.get(&(i-1)).unwrap();
                            cross.push((pos, step + past_step))
                        }
                    },
                    _ => {
                        let mut v = HashMap::new();
                        v.insert(i, step);
                        positions.insert(pos, v);
                    }
                }
            }
        }
        i += 1;
    }

    let min = cross
        .into_iter()
        .map(|(_, steps)| steps)
        .fold(100000, |a, b| if a < b { a } else { b });
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
