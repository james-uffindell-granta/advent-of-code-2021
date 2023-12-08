use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CaveType {
    Small,
    Large,
}

#[derive(Debug, Clone)]
pub struct Cave {
    name: String,
    cave_type: CaveType,
}

#[derive(Debug, Clone)]
pub struct CaveNetwork {
    cave_options: HashMap<String, Vec<String>>,
}

impl CaveNetwork {
    pub fn all_paths_dfs(&self) -> HashSet<Vec<String>> {
        self.all_paths_dfs_from("start", HashSet::from([ "start".to_owned() ]),
        HashSet::new(), None)
    }

    pub fn all_paths_dfs_allowing_twice(&self, cave: &str) -> HashSet<Vec<String>> {
        self.all_paths_dfs_from("start", HashSet::from([ "start".to_owned() ]),
        HashSet::new(), Some(cave))
    }

    pub fn all_paths_dfs_from(&self, cave: &str, visited_small_caves: HashSet<String>,
        twice_visited_small_caves: HashSet<String>, allowed_small_cave_twice: Option<&str>
    ) -> HashSet<Vec<String>> {
        // if we're already at the end, can't go anywhere from here
        if cave == "end" {
            return HashSet::from([ vec![]]);
        }

        let mut paths = HashSet::new();
        for neighbour in self.cave_options.get(cave).unwrap() {
            match allowed_small_cave_twice {
                None => {
                    if visited_small_caves.contains(neighbour) {
                        // can't go that way
                        continue;
                    }  
                },
                Some(c) => {
                    if c != neighbour && visited_small_caves.contains(neighbour) {
                        // not allowed to go this way twice
                        continue;
                    }  

                    if c == neighbour && twice_visited_small_caves.contains(neighbour) {
                        // already gone this way twice
                        continue;
                    }
                }
            }

            // otherwise we're allowed to visit this cave

            // if this is a small neighbour, remember we can't go there again, and recurse
            if neighbour.chars().all(|c| c.is_lowercase()) {
                if visited_small_caves.contains(neighbour) {
                    // this must be the second time we're allowed to visit it
                    for mut path in self.all_paths_dfs_from(neighbour, visited_small_caves.clone(), HashSet::from([neighbour.clone()]), allowed_small_cave_twice) {
                        path.push(neighbour.clone());
                        paths.insert(path);
                    }
                } else {
                    let mut new_visited_caves = visited_small_caves.clone();
                    new_visited_caves.insert(neighbour.clone());
                    for mut path in self.all_paths_dfs_from(neighbour, new_visited_caves, twice_visited_small_caves.clone(), allowed_small_cave_twice) {
                        path.push(neighbour.clone());
                        paths.insert(path);
                    }
                }
            } else {
                // otherwise we can easily go there again, so just find all the paths from that neighbour
                for mut path in self.all_paths_dfs_from(neighbour, visited_small_caves.clone(), twice_visited_small_caves.clone(), allowed_small_cave_twice.clone()) {
                    path.push(neighbour.clone());
                    paths.insert(path);
                }
            }
        }

        paths
    }
}

pub fn part_2(network: &CaveNetwork) -> usize {
    let mut small_caves_to_consider = network.cave_options.keys().filter(|s| s.chars().all(|c| c.is_lowercase())).collect::<HashSet<_>>();
    small_caves_to_consider.remove(&"start".to_owned());
    small_caves_to_consider.remove(&"end".to_owned());
    let mut all_paths = network.all_paths_dfs();
    for cave in small_caves_to_consider {
        all_paths.extend(network.all_paths_dfs_allowing_twice(cave))
    }

    all_paths.len()
}

pub fn parse_input(input: &str) -> CaveNetwork {
    let mut map = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').unwrap();
        map.entry(left.to_owned()).or_insert(Vec::new()).push(right.to_owned());
        map.entry(right.to_owned()).or_insert(Vec::new()).push(left.to_owned());
    }

    CaveNetwork { cave_options: map }
}

fn main() {
    let input = include_str!("../input.txt");
    let network = parse_input(input);
    println!("Part 1: {}", network.all_paths_dfs().len());
    println!("Part 2: {}", part_2(&network));
}

#[test]
pub fn test_1() {
    let input = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    let network = parse_input(input);
    assert_eq!(network.all_paths_dfs().len(), 10);
    assert_eq!(part_2(&network), 36);
}

#[test]
pub fn test_2() {
    let input = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    let network = parse_input(input);
    assert_eq!(network.all_paths_dfs().len(), 19);
    assert_eq!(part_2(&network), 103);
}

#[test]
pub fn test_3() {
    let input = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    let network = parse_input(input);
    assert_eq!(network.all_paths_dfs().len(), 226);
    assert_eq!(part_2(&network), 3509);
}
