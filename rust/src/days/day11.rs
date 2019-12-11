use std::collections::HashMap;
use num::Complex;
use crate::intcode::{Opcodes,Status,Machine};

// Helper
fn read_input (input: &str) -> Opcodes {
    input
        .lines()
        .collect::<Vec<&str>>()
        .join("")
        .split(",")
        .map(|s| s.trim().parse().expect("Cannot parse int"))
        .collect()
}

// Part1
pub fn part1 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    let mut map = HashMap::<Complex<i64>, i64>::new();
    let mut pos = Complex::new(0, 0);
    let mut orientation = Complex::new(0, -1);
    'main_loop: loop {
        let current_color = map.get(&pos).unwrap_or(&0);
        machine.add_input_mut(current_color.clone());

        let mut color = None;
        let mut rotation = None;
        while color.is_none() || rotation.is_none() {
            match machine.step() {
                Status::Output(o) => {
                    if color.is_none() {
                        color = Some(o);
                    } else {
                        rotation = Some(o);
                    }
                },
                Status::Halt      => break 'main_loop,
                Status::Ok        => {},
                err               => panic!("Received a state {:?}", err)
            }
        }
        map.insert(pos.clone(), color.unwrap());
        orientation *= if rotation.unwrap() == 1 { Complex::new(0, 1) } else { Complex::new(0, -1) };
        pos += orientation;
    }
    format!("{}", map.len())
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    let mut map = HashMap::<Complex<i64>, i64>::new();
    let mut pos = Complex::new(0, 0);
    let mut orientation = Complex::new(0, -1);
    map.insert(pos.clone(), 1);
    'main_loop: loop {
        let current_color = map.get(&pos).unwrap_or(&0);
        machine.add_input_mut(current_color.clone());

        let mut color = None;
        let mut rotation = None;
        while color.is_none() || rotation.is_none() {
            match machine.step() {
                Status::Output(o) => {
                    if color.is_none() {
                        color = Some(o);
                    } else {
                        rotation = Some(o);
                    }
                },
                Status::Halt      => break 'main_loop,
                Status::Ok        => {},
                err               => panic!("Received a state {:?}", err)
            }
        }
        map.insert(pos.clone(), color.unwrap());
        orientation *= if rotation.unwrap() == 1 { Complex::new(0, 1) } else { Complex::new(0, -1) };
        pos += orientation;
    }

    let (min_x, max_x, min_y, max_y) = map
        .keys()
        .fold((i64::max_value(), 0, i64::max_value(), 0), |(min_x, max_x, min_y, max_y), a| (
                std::cmp::min(min_x, a.re),
                std::cmp::max(max_x, a.re),
                std::cmp::min(min_y, a.im),
                std::cmp::max(max_y, a.im)
        ));

    let mut out = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Complex::new(x, y);
            let color = map.get(&pos).unwrap_or(&0);
            out += if *color == 1 { "#" } else { " " };
        }
        out += "\n";
    }
    format!("{}", out)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day11_part1 () {
        assert_eq!(super::part1("
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 0, 104, 0,
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 0, 104, 1,
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 1, 104, 0,
                99"), "6");
    }

    #[test]
    fn day11_part2 () {
        assert_eq!(super::part2("
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 0, 104, 0,
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 0, 104, 1,
                3, 98,
                104, 1, 104, 0,
                3, 98,
                104, 1, 104, 0,
                99"), "  #\n  #\n## \n");
    }
}
