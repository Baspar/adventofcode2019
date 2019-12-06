use std::collections::{HashMap, HashSet};


// Helper
type Graph = HashMap<String, HashSet<String>>;
fn read_input (input: &str) -> Graph {
    let orbits: Vec<(String, String)>  = input
        .lines()
        .map(|l| {
            let v: Vec<&str> = l.split(")").collect();
            (String::from(v[0]), String::from(v[1]))
        })
        .collect();

    let mut graph = HashMap::new();
    for (center, satellite) in orbits {
        if graph.get(&center).is_none() {
            graph.insert(center.clone(), HashSet::new());
        }
        graph.get_mut(&center).unwrap().insert(satellite);
    }
    graph
}
fn get_n_satellites_and_n_orbits (graph: &Graph, node: &String) -> (usize, usize) {
    let mut n_satellites = 1;
    let mut n_orbits = 0;
    match graph.get(node) {
        Some(satellites) => {
            for satellite in satellites {
                let (n_satellite_satellites, n_satellite_orbits) = get_n_satellites_and_n_orbits(graph, satellite);
                n_satellites += n_satellite_satellites;
                n_orbits += n_satellite_orbits + n_satellite_satellites;
            }
        },
        _ => {}
    }
    (n_satellites, n_orbits)
}

// Part1
pub fn part1 (input: &str) -> String {
    let graph = read_input(input);
    let (_, n_orbits) =  get_n_satellites_and_n_orbits(&graph, &String::from("COM"));
    format!("{}", n_orbits)
}

// Part2
pub fn part2 (input: &str) -> String {
    let _ = read_input(input);
    format!("{:?}", 0)
}

// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn day6_part1 () {
        assert_eq!(super::part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"), "42");
    }

    #[test]
    fn day6_part2 () {
        assert_eq!(super::part2("0"), "0");
    }
}
