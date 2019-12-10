use std::collections::{HashSet,HashMap};
use itertools::Itertools;
use num::Complex;

// Helper
type Coord = Complex<i64>;
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

// Part1
pub fn part1 (input: &str) -> String {
    let asteroids = read_input(input);
    let mut asteroids_set = HashSet::new();
    for asteroid in &asteroids {
        asteroids_set.insert(asteroid);
    }
    let mut nb_reachable: HashMap<Coord, i64> = HashMap::new();

    'combination_loop: for perm in asteroids.iter().combinations(2) {
        let (asteroid_from, asteroid_to) = (perm[0], perm[1]);
        let distance = asteroid_from - asteroid_to;
        let gcd = gcd(distance.re.abs(), distance.im.abs());
        let step = distance / gcd;

        let mut current = asteroid_to.clone();
        while { current += step; current != *asteroid_from } {
            if asteroids_set.contains(&current) {
                continue 'combination_loop;
            }
        }

        *nb_reachable.entry(*asteroid_from).or_insert(0) += 1;
        *nb_reachable.entry(*asteroid_to).or_insert(0) += 1;
    }

    let max = nb_reachable.values().fold(0, |v1, &v2| std::cmp::max(v1, v2));
    format!("{:?}", max)
    // format!("{:?}", nb_reachable)
    // format!("{:?}", 0)
}

// Part2
pub fn part2 (input: &str) -> String {
    format!("{}", 0)
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
        assert_eq!(super::part2("0"), "0");
    }
}
