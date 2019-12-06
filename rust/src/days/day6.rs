use std::collections::{HashMap,HashSet,VecDeque};

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
fn path_to (graph: &Graph, root: &String, node: &String) -> Option<VecDeque<String>> {
    if *root == *node {
        return Some(VecDeque::new())
    }

    if let Some(satellites) = graph.get(root) {
        for sat in satellites {
            if let Some(mut path) = path_to(graph, sat, node) {
                path.push_front(root.clone());
                return Some(path);
            }
        }
    }

    return None;
}

// Part1
pub fn part1 (input: &str) -> String {
    let graph = read_input(input);
    let (_, n_orbits) =  get_n_satellites_and_n_orbits(&graph, &String::from("COM"));
    format!("{}", n_orbits)
}

// Part2
pub fn part2 (input: &str) -> String {
    let graph = read_input(input);
    let mut path_to_santa = path_to(&graph, &String::from("COM"), &String::from("SAN")).unwrap();
    let mut path_to_you = path_to(&graph, &String::from("COM"), &String::from("YOU")).unwrap();

    while path_to_you.front().unwrap() == path_to_santa.front().unwrap() {
        path_to_santa.pop_front();
        path_to_you.pop_front();
    }
    // println!("SAN: {:?}", path_to_santa);
    // println!("YOU: {:?}", path_to_you);
    format!("{}", path_to_you.len() + path_to_santa.len())
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
        assert_eq!(super::part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"), "4");
    }
}
