use regex::Regex;
use itertools::Itertools;
use num::integer::lcm;

// Helper
fn read_input (input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            let regex = Regex::new(r"(-?\d+)").unwrap();
            let mut coord = Vec::new();
            let mut groups = regex.captures_iter(line);
            for _ in 0..3 {
                coord.push(groups.next().unwrap()[1].parse().unwrap());
            }
            coord
        })
        .collect()
}
fn step (moons: &mut Vec<Vec<i64>>, velocities: &mut Vec<Vec<i64>>) {
    for perm in (0..4).into_iter().combinations(2) {
        let [i1, i2] = [perm[0], perm[1]];
        for coord in 0..3 {
            let coord1 = moons[i1][coord];
            let coord2 = moons[i2][coord];
            if coord1 < coord2 {
                velocities[i1][coord] += 1; velocities[i2][coord] -= 1;
            } else if coord1 > coord2 {
                velocities[i1][coord] -= 1; velocities[i2][coord] += 1;
            }
        }
    }

    for i in 0..4 {
        for coord in 0..3 {
            moons[i][coord] += velocities[i][coord];
        }
    }
}
fn run_for (mut moons: Vec<Vec<i64>>, n: i64) -> String {
    let mut velocities = vec![vec![0, 0, 0],vec![0, 0, 0],vec![0, 0, 0],vec![0, 0, 0]];
    for _ in 0..n {
        step(&mut moons, &mut velocities);
    }

    let mut energy: i64 = 0;
    for i in 0..4 {
        let mut kin = 0i64;
        let mut pot = 0i64;
        for j in 0..3 {
            kin += velocities[i][j].abs();
            pot += moons[i][j].abs();
        }
        energy += kin * pot;
    }
    format!("{}", energy)
}

// Part1
pub fn part1 (input: &str) -> String {
    let moons = read_input(input);
    run_for(moons, 1000)
}

// Part2
pub fn part2 (input: &str) -> String {
    let mut moons = read_input(input);
    let initial_positions = moons.clone();
    let mut velocities = vec![vec![0, 0, 0],vec![0, 0, 0],vec![0, 0, 0],vec![0, 0, 0]];
    let mut periods: Vec<Option<i64>> = vec![None, None, None];
    let mut iteration = 0;
    while !periods.iter().all(|x| x.is_some()) {
        step(&mut moons, &mut velocities);
        iteration += 1;
        for axis in 0..3 {
            if periods[axis].is_none() {
                let loop_detected = (0..4)
                    .into_iter()
                    .all(|i| moons[i][axis] == initial_positions[i][axis] && velocities[i][axis] == 0);
                if loop_detected {
                    periods[axis] = Some(iteration);
                }
            }
        }
    }
    let out = lcm(lcm(periods[0].unwrap(), periods[1].unwrap()), periods[2].unwrap());
    format!("{}", out)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day12_part1 () {
        assert_eq!(super::run_for(super::read_input("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"), 10), "179");
        assert_eq!(super::run_for(super::read_input("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"), 100), "1940");
    }

    #[test]
    fn day12_part2 () {
        assert_eq!(super::part2("<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"), "2772");
        assert_eq!(super::part2("<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"), "4686774924");
    }
}
