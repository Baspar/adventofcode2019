use std::collections::{HashSet,HashMap};
use itertools::Itertools;
use num::Complex;

// Helper
type Coord = Complex<i64>;
type Angle = Coord;
type Coords = Vec<Coord>;

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
fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
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

    let mut relative_positions: HashMap<Angle, Vec<(i64, Coord)>> = HashMap::new();
    for asteroid in asteroids {
        if asteroid != base {
            let distance = asteroid - base;
            let divider = gcd(distance.re.abs(), distance.im.abs());
            let step = distance / divider;

            relative_positions
                .entry(step).or_insert(Vec::new())
                .push((divider, asteroid));
        }
    }

    let mut sorted_horizons: Vec<(f64, Coords)> = Vec::new();
    for (Complex {re, im}, mut aligned_position) in relative_positions {
        let angle = Complex::new(im as f64, re as f64).arg();
        aligned_position.sort_by_key(|(distance, _)| distance.abs());
        aligned_position.reverse();
        let aligned_asteroids: Coords = aligned_position.iter().map(|(_, coord)| *coord).collect();
        sorted_horizons.push((angle, aligned_asteroids));
    }
    sorted_horizons.sort_by(|(angle_a, _), (angle_b, _)| angle_b.partial_cmp(&angle_a).unwrap_or(std::cmp::Ordering::Equal));

    let mut coords: Vec<Coords> = sorted_horizons.iter().map(|(_, coords)| coords.clone()).collect();

    let mut i = 0;
    while !coords.is_empty() {
        let index = i % coords.len();

        match coords[index].pop() {
            None => { coords.remove(index); },
            Some(coord) => {
                if i == 199 {
                    return format!("{}", 100 * coord.re + coord.im);
                }
                i += 1;
            }
        }

    }
    format!("{}", 1)
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
