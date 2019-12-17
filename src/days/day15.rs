use std::collections::{HashMap,HashSet,VecDeque};
use num::Complex;
use crate::intcode::{Opcodes,Status,Machine};

#[derive(Debug)]
enum Cell {
    Empty,
    Wall,
    Oxygen
}

type Position = Complex<i64>;
struct Maze {
    position: Position,
    map: HashMap<Position, Cell>,
    backtrack: VecDeque<i64>,
    machine: Machine,
    oxygen_position: Option<Position>
}
impl Maze {
    pub fn new (machine: Machine) -> Self {
        let mut map = HashMap::new();
        map.insert(Complex::new(0, 0), Cell::Empty);
        Self {
            map,
            machine,
            position: Complex::new(0, 0),
            backtrack: VecDeque::new(),
            oxygen_position: None
        }
    }

    pub fn explore (self: &mut Self) -> Option<Position> {
        let inputs = [1, 2, 3, 4];
        let backtrack_inputs = [2, 1, 4, 3];
        let movements = [Complex::new(-1, 0), Complex::new(1, 0), Complex::new(0, -1), Complex::new(0, 1)];
        for i in 0..4 {
            let movement = movements[i];
            let new_position = self.position + movement;
            if self.map.get(&new_position).is_some() {
                continue;
            }

            self.machine.add_input_mut(inputs[i]);
            match self.machine.run_until_interrupted() {
                Status::Output(0) => {
                    self.map.insert(new_position, Cell::Wall);
                },
                Status::Output(1) => {
                    self.map.insert(new_position, Cell::Empty);
                    self.position = new_position;
                    self.backtrack.push_front(backtrack_inputs[i]);
                    return Some(new_position);
                },
                Status::Output(2) => {
                    self.map.insert(new_position, Cell::Oxygen);
                    self.position = new_position;
                    self.backtrack.push_front(backtrack_inputs[i]);
                    self.oxygen_position = Some(new_position);
                    return Some(new_position);
                },
                other => panic!("Wrong output: {:?}", other)
            };
        }
        return None;
    }

    pub fn cannot_backtrack (self: &Self) -> bool {
        self.backtrack.is_empty()
    }

    pub fn shortest_path (self: &Self) -> Option<i64> {
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((0, Complex::new(0, 0)));
        while !q.is_empty() {
            let (distance, pos) = q.pop_front().unwrap();
            if visited.contains(&pos) { continue }
            visited.insert(pos);
            for direction in &[Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)] {
                let new_pos = pos + direction;
                match self.map.get(&new_pos) {
                    Some(Cell::Empty) => { q.push_back((distance + 1, new_pos)); },
                    Some(Cell::Oxygen) => { return Some(distance + 1) },
                    _ => {}
                }
            }
        }
        None
    }

    pub fn backtrack (self: &mut Self) {
        let backtrack_input = self.backtrack.pop_front().expect("Cannot backtrack anymore");
        let movement = match backtrack_input {
            1 => Complex::new(-1, 0),
            2 => Complex::new(1, 0),
            3 => Complex::new(0, -1),
            4 => Complex::new(0, 1),
            other => panic!("Unknown backtrack input {}", other)
        };

        self.machine.add_input_mut(backtrack_input);
        loop {
            match self.machine.run_until_interrupted() {
                Status::Output(_) => break,
                _ => {}
            }
        }

        self.position += movement;
    }
    pub fn display (self: &Self) {
        let (min_x, max_x, min_y, max_y) = self.map
            .keys()
            .fold(
                (i64::max_value(), 0, i64::max_value(), 0),
                |(min_x, max_x, min_y, max_y), pos| (
                    min_x.min(pos.re),
                    max_x.max(pos.re),
                    min_y.min(pos.im),
                    max_y.max(pos.im)
                ));

        for y in min_y..=max_y {
            println!("");
            for x in min_x..=max_x {
                let pos = Complex::new(x, y);
                let c = if pos == Complex::new(0, 0) {
                    "0"
                } else {
                    match self.map.get(&pos) {
                        Some(Cell::Oxygen) => "*",
                        Some(Cell::Empty) => " ",
                        Some(Cell::Wall) => "#",
                        _ => "."
                    }
                };
                print!("{}", c);
            }
        }
    }
}

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
    let machine = Machine::new(&opcodes);
    let mut map = Maze::new(machine);
    loop {
        while map.explore().is_some() { }

        if map.cannot_backtrack() {
            break;
        }

        map.backtrack();
    }

    map.display();

    format!("{}", map.shortest_path().unwrap())
}

// Part2
pub fn part2 (input: &str) -> String {
    let opcodes = read_input(input);
    let mut machine = Machine::new(&opcodes);
    machine.step();
    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day15_part1 () {
        assert_eq!(super::part1("0"), "0");
    }

    #[test]
    fn day15_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
