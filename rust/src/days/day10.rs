use std::collections::{HashSet,HashMap,BTreeMap};
use itertools::Itertools;
use num::Complex;
use num::integer::gcd;
use std::cmp::Ordering;

// Helper
type Coord = Complex<i64>;
struct Angle(Complex<i64>);
type Coords = Vec<Coord>;

impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_angle = Complex::new(self.0.im as f64, self.0.re as f64).arg();
        let other_angle = Complex::new(other.0.im as f64, other.0.re as f64).arg();
        other_angle.partial_cmp(&self_angle).unwrap_or(Ordering::Equal)
    }
}
impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Angle { }

fn read_input (input: &str) -> Coords {
    let mut out = Vec::new();
    for (y, line) in input.trim().split("\n").enumerate() {
        for (x, cell) in line.chars().enumerate() {
            match cell {
                '#' => out.push(Complex::new(x as i64, y as i64)),
                _ => {},
            }
        }
    }
    out
}
fn get_best_asteroid (asteroids: &Coords) -> (i64, Coord) {
    let mut asteroids_set = HashSet::new();
    for asteroid in asteroids {
        asteroids_set.insert(asteroid);
    }
    let mut nb_reachable: HashMap<Coord, i64> = HashMap::new();

    'combination_loop: for perm in asteroids.iter().combinations(2) {
        let (asteroid_from, asteroid_to) = (perm[0], perm[1]);
        let distance = asteroid_from - asteroid_to;
        let divider = gcd(distance.re.abs(), distance.im.abs());
        let step = distance / divider;

        for i in 1..divider {
            if asteroids_set.contains(&(asteroid_to + i * step)) {
                continue 'combination_loop;
            }
        }

        *nb_reachable.entry(*asteroid_from).or_insert(0) += 1;
        *nb_reachable.entry(*asteroid_to).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut argmax = Complex::new(0, 0);
    for (asteroid, reachable) in nb_reachable {
        if reachable > max {
            max = reachable;
            argmax = asteroid;
        }
    }
    (max, argmax)
}

// Part1
pub fn part1 (input: &str) -> String {
    let asteroids = read_input(input);
    let (max, _) = get_best_asteroid(&asteroids);
    format!("{}", max)
}

// Part2
pub fn part2 (input: &str) -> String {
    let asteroids = read_input(input);
    let (_, base) = get_best_asteroid(&asteroids);

    // Building Btree indexed by "angle", sorted clockwise from UP
    let mut relative_positions: BTreeMap<Angle, Vec<(i64, Coord)>> = BTreeMap::new();
    for asteroid in asteroids {
        if asteroid != base {
            let distance = asteroid - base;
            let divider = gcd(distance.re.abs(), distance.im.abs());
            let step = Angle (distance / divider);

            relative_positions
                .entry(step).or_insert(Vec::new())
                .push((divider, asteroid));
        }
    }

    // Sort every asteroid in line according to distance on the line of view
    let mut relative_positions: Vec<Vec<(i64, Coord)>> = relative_positions
        .values()
        .map(|asteroids_in_line| {
            let mut asteroids_in_line = asteroids_in_line.clone();
            asteroids_in_line.sort_by_key(|asteroids_in_line| asteroids_in_line.0);
            asteroids_in_line
        })
        .collect();

    let mut i = 0;
    while !relative_positions.is_empty() {
        let index = i % relative_positions.len();

        match relative_positions[index].pop() {
            None => { relative_positions.remove(index); },
            Some((_, coord)) => {
                if i == 199 {
                    return format!("{}", 100 * coord.re + coord.im);
                }
                i += 1;
            }
        }

    }
    format!("{}", -1)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day10_part1 () {
        assert_eq!(
            super::part1(".#..#\n.....\n#####\n....#\n...##"),
            "8"
        );
        assert_eq!(
            super::part1("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####"),
            "33"
        );
        assert_eq!(
            super::part1("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###."),
            "35"
        );
        assert_eq!(
            super::part1(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#.."),
            "41"
        );
        assert_eq!(
            super::part1(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"),
            "210"
        );
    }

    #[test]
    fn day10_part2 () {
        assert_eq!(super::part2(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##"), "802");
    }
}
