use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("input").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    let char_map = parse(&contents);
    let result = solve_first(&char_map);
    println!("First result is {:?}", result);
}

fn solve_first(char_map: &HashMap<char, Vec<char>>) -> String {
    let mut result = String::new();
    let mut indegrees = indegrees(&char_map);

    let roots = find_roots(&char_map, &indegrees);
    let mut queue: String = roots.iter().collect();

    while !queue.is_empty() {
        // sort string
        let mut chars: Vec<char> = queue.chars().collect();
        chars.sort_by(|a, b| b.cmp(a));
        queue = chars.iter().collect();

        let popped = queue.pop().unwrap();
        result.push(popped);
        if let Some(neighbors) = &char_map.get(&popped) {
            for neighbor in *neighbors {
                let entry = indegrees.entry(*neighbor).or_insert(0);
                *entry -= 1;

                if *entry == 0 {
                    queue.insert(0, neighbor.clone());
                }
            }
        }
    }
    result
}

fn find_roots(char_map: &HashMap<char, Vec<char>>, indegrees: &HashMap<char, u32>) -> Vec<char> {
    let mut roots = Vec::<char>::new();
    char_map.keys().for_each(|k| {
        if !&indegrees.contains_key(&k) {
            roots.push(k.clone());
        }
    });
    roots.sort_by(|lhs, rhs| rhs.cmp(lhs));
    roots
}

fn indegrees(char_map: &HashMap<char, Vec<char>>) -> HashMap<char, u32> {
    let mut indegrees = HashMap::<char, u32>::new();

    for values in char_map.values() {
        for ch in values {
            let entry = indegrees.entry(*ch).or_insert(0);
            *entry += 1;
        }
    }

    indegrees
}

fn parse(contents: &str) -> HashMap<char, Vec<char>> {
    contents
        .lines()
        .map(|str| {
            (str.chars().nth(5).unwrap(), str.chars().nth(36).unwrap())
        })
        .fold(
            HashMap::<char, Vec<char>>::new(),
            |mut char_map, (lhs, rhs)| {
                char_map
                    .entry(lhs)
                    .and_modify(|chars| {
                        chars.push(rhs);
                    })
                    .or_insert_with(||vec![rhs]);

                char_map
            },
        )
}

