use num::Complex;
use std::collections::{HashMap,HashSet,BinaryHeap,VecDeque};
use std::cmp::Ordering;

// Helper
#[derive(Debug)]
enum Cell {
    Wall,
    Hall,
    Door(char),
    Key(char)
}

#[derive(Debug)]
struct Maze {
    map: HashMap<Complex<i64>, Cell>,
    pos: Complex<i64>,
    height: usize,
    width: usize,
    keys_count: usize
}
impl Maze {
    fn new (input: &str) -> Self {
        let mut map = HashMap::new();
        let mut pos = None;
        let mut keys_count = 0;
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = Complex::new(x as i64, y as i64);
                if c == '@' {
                    pos = Some(coord);
                }
                match c {
                    'a'..='z' => keys_count += 1,
                    _ => {}
                }
                map.insert(coord, match c {
                    '#' => Cell::Wall,
                    '.' | '@' => Cell::Hall,
                    'a'..='z' => Cell::Key(c),
                    'A'..='Z' => Cell::Door(c.to_lowercase().nth(0).unwrap()),
                    _ => panic!("{} is not a known char", c)
                });
            }
        }
        Self {
            map,
            pos: pos.unwrap(),
            height: input.lines().count(),
            width: input.lines().nth(0).unwrap().len(),
            keys_count
        }
    }

    fn display (self: &Self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Complex::new(x as i64, y as i64);
                print!(
                    "{}",
                    match self.map[&pos] {
                        Cell::Wall => String::from("#"),
                        Cell::Hall => String::from("."),
                        Cell::Door(c) => c.to_string(),
                        Cell::Key(c) => c.to_uppercase().collect(),
                    }
                );
            }
            println!("")
        }
    }

    fn next_states (self: &Self, current_state: State) -> Vec<State> {
        let mut next_states = Vec::new();
        let mut bfs_stack = VecDeque::new();
        let mut visited = HashSet::new();
        bfs_stack.push_front(current_state);

        while let Some(state) = bfs_stack.pop_back() {
            visited.insert(state.pos);
            for direction in &[Complex::new(1, 0), Complex::new(-1, 0), Complex::new(0, 1), Complex::new(0, -1)] {
                let pos = state.pos + direction;
                let cell = self.map.get(&pos).unwrap();

                if visited.contains(&pos) { continue }

                match cell {
                    Cell::Wall => continue,
                    Cell::Door(c) => {
                        if !state.collected_keys.contains(c) {
                            continue
                        }
                    },
                    Cell::Key(c) => {
                        if !state.collected_keys.contains(c) {
                            let mut next_collected_keys = state.collected_keys.clone();
                            next_collected_keys.insert(*c);
                            next_states.push(State {
                                pos,
                                distance: state.distance + 1,
                                collected_keys: next_collected_keys
                            })
                        }
                    },
                    _ => {}
                }

                bfs_stack.push_front(State {
                    pos,
                    distance: state.distance + 1,
                    collected_keys: state.collected_keys.clone()
                })
            }
        }

        next_states
    }
}

#[derive(Debug)]
struct State {
    distance: usize,
    pos: Complex<i64>,
    collected_keys: HashSet<char>
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
impl Eq for State {
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

// Part1
pub fn part1 (input: &str) -> String {
    let maze = Maze::new(input);
    maze.display();

    let init_state = State {
        distance: 0,
        pos: maze.pos,
        collected_keys: HashSet::new()
    };
    let mut q = BinaryHeap::new();
    q.push(init_state);

    while let Some(state) = q.pop() {
        if state.collected_keys.len() == maze.keys_count {
            return format!("{}", state.distance)
        }
        for next_state in maze.next_states(state) {
            q.push(next_state)
        }
    }

    format!("{}", 0)
}

// Part2
pub fn part2 (input: &str) -> String {
    let maze = Maze::new(input);
    format!("{}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day18_part1 () {
        assert_eq!(super::part1("#########\n#b.A.@.a#\n#########"), "8");
        assert_eq!(super::part1("########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################"), "86");
        assert_eq!(super::part1("########################\n#...............b.C.D.f#\n#.######################\n#.....@.a.B.c.d.A.e.F.g#\n########################"), "132");
        // assert_eq!(super::part1("#################\n#i.G..c...e..H.p#\n########.########\n#j.A..b...f..D.o#\n########@########\n#k.E..a...g..B.n#\n########.########\n#l.F..d...h..C.m#\n#################"), "136");
        assert_eq!(super::part1("########################\n#@..............ac.GI.b#\n###d#e#f################\n###A#B#C################\n###g#h#i################\n########################"), "81");
    }

    #[test]
    fn day18_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
