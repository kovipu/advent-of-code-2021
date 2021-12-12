use std::collections::HashMap;

pub fn part_1(input: &str) -> i64 {
    let connections: HashMap<&str, Vec<&str>> = connections_hashmap(&input);
    find_paths(&connections, &[], "start").len() as i64
}

pub fn part_2(input: &str) -> i64 {
    let connections: HashMap<&str, Vec<&str>> = connections_hashmap(&input);
    find_paths_2(&connections, &[], "start", false).len() as i64
}

fn connections_hashmap(input: &str) -> HashMap<&str, Vec<&str>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let (a, b) = line.split_once('-').unwrap();

        match acc.get_mut(a) {
            Some(cons) => cons.push(b),
            None => {
                acc.insert(a, vec![b]);
            }
        }

        match acc.get_mut(b) {
            Some(cons) => cons.push(a),
            None => {
                acc.insert(b, vec![a]);
            }
        }

        acc
    })
}

fn find_paths<'a>(
    connections: &'a HashMap<&str, Vec<&str>>,
    path: &[&'a str],
    cave: &'a str,
) -> Vec<Vec<&'a str>> {
    let mut new_path = path.to_owned();
    new_path.push(cave);

    if cave.to_lowercase() == *cave && path.contains(&cave) {
        return vec![];
    } else if cave == "end" {
        return vec![new_path];
    }

    connections
        .get(cave)
        .unwrap()
        .iter()
        .fold(vec![], |mut acc, connection| {
            for path in find_paths(connections, &new_path, connection) {
                acc.push(path);
            }
            acc
        })
}

fn find_paths_2<'a>(
    connections: &'a HashMap<&str, Vec<&str>>,
    path: &[&'a str],
    cave: &'a str,
    cave_visited_twice: bool,
) -> Vec<Vec<&'a str>> {
    let mut new_path = path.to_owned();
    new_path.push(cave);

    if cave == "end" {
        return vec![new_path];
    } else if cave.to_lowercase() == *cave && path.contains(&cave) && cave_visited_twice {
        return vec![];
    }

    let new_cave_visited_twice =
        cave_visited_twice || cave.to_lowercase() == *cave && path.contains(&cave);

    connections
        .get(cave)
        .unwrap()
        .iter()
        .fold(vec![], |mut acc, connection| {
            if *connection == "start" {
                return acc;
            }

            for path in find_paths_2(connections, &new_path, connection, new_cave_visited_twice) {
                acc.push(path);
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 36);
    }
}

#[cfg(test)]
const INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
